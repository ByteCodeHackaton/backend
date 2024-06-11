mod error;
mod jwt;
mod authentification;
mod registry;
mod rate_limiter;
mod body_helpers;

use authentification::update_tokens;
use body_helpers::{error_empty_response, error_response, BoxBody};
use hyper::{server::conn::http1, Uri};
use error::GatewayError;
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;
use rate_limiter::RateLimiter;
use registry::{deregister_service, register_service, ServiceRegistry};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::net::SocketAddr;
use hyper::{body::Incoming, Request, Response, StatusCode};

///
async fn service_handler(req: Request<Incoming>) -> Result<Response<BoxBody>, GatewayError>
{
    // Example of request transformation: Adding a custom header
    let req = Request::builder()
        .method(req.method())
        .uri(req.uri())
        .header("X-Custom-Header", "My API Gateway")
        .body(req.into_body())
        .unwrap();
    let auth = req.uri().authority();
    let target_host = auth.unwrap().as_str().replace("localhost", "127.0.0.1");
    let addr: SocketAddr = target_host.parse().unwrap();
    // Отправка запроса на связанный сервис
    //let req = Request::new(req.boxed());
    let response = send(addr, req).await?;

    // Example of response transformation: Append custom JSON
    // let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    // let data_result: Result<serde_json::Value, _> = serde_json::from_slice(&body_bytes);

    // let mut data = match data_result {
    //     Ok(d) => d,
    //     Err(_) => {
    //         return Ok(Response::builder()
    //             .status(StatusCode::BAD_GATEWAY)
    //             .body(Body::from("Failed to parse upstream response"))
    //             .unwrap())
    //     }
    // };

    // data["custom"] = json!("This data is added by the gateway");
    //пока не будем делать инъекцию json просто трансформирую данные
    logger::debug!("получен ответ от сервиса {:?}", response.body());
    Ok(Response::new(response.boxed()))
}


async fn handle_request(
    mut req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    registry: Arc<ServiceRegistry>,
) -> Result<Response<BoxBody>, GatewayError> 
{
    if !rate_limiter.allow(remote_addr) 
    {
        return Ok(error_empty_response(StatusCode::TOO_MANY_REQUESTS));
    }
    //TODO сделать сервис аутентификации
    // let unauthorized_response = Response::builder()
    // .status(StatusCode::UNAUTHORIZED)
    // .body(to_body(Bytes::from_static(b"Unauthorized")))
    // .unwrap();
    // // Authentication
    // match req.headers().get("Authorization") 
    // {
    //     Some(value) => 
    //     {
    //         let token_str = value.to_str().unwrap_or("");
    //         if !authenticate(token_str) 
    //         {
    //             return Ok(unauthorized_response);
    //         }
    //     },
    //     None => 
    //     {
    //         return Ok(unauthorized_response);
    //     }
    // }
    //убираем начальный слеш
    let path = req.uri().path().strip_prefix('/');
    if path.is_none()
    {
        return Ok(error_response(format!("Ошибка запроса {}, указан неверный путь к сервису",  req.uri().path()), StatusCode::BAD_REQUEST));
    }
    let path = path.unwrap();
    //разделяем путь между сервисом и дальнейшим путем
    let parts = path.split_once('/');
    if parts.is_none()
    {
        return Ok(error_response(format!("Ошибка запроса {}, не уточнен сервис к которому производится запрос", path), StatusCode::BAD_REQUEST));
    }
    let (service_name, path) = parts.unwrap();
    match registry.get_config(service_name) 
    {
        Some(service) => 
        {
            if let Some(endpoint) = service.get_endpoint(path)
            {

                let sn = [service_name, "/"].concat();
                let p_q = req.uri().path_and_query().unwrap().to_string().replace(&sn, "");
                let uri = Uri::builder()
                .scheme("http")
                .authority(service.get_address())
                .path_and_query(p_q).build().unwrap();

                logger::info!("Запрос переадресован на {}", uri.to_string());
                *req.uri_mut() = uri;

                return service_handler(req).await;
            }
            else 
            {
                return Ok(error_response(format!("Ошибка в сервисе {} не найден путь {}", service_name, path), StatusCode::BAD_REQUEST));
            }
        },
        None => return Ok(error_response(format!("Сервис {} не найден", service_name), StatusCode::NOT_FOUND)),
    }

}

async fn router(
    req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError>
{   
    let path = req.uri().path();
    if path == "/register_service" 
    {
        return register_service(req, Arc::clone(&registry)).await;
    }

    if path == "/deregister_service" 
    {
        return deregister_service(req, Arc::clone(&registry)).await;
    }
    if path == "/update_tokens" 
    {
        return update_tokens(req).await;
    }
    // if path == "/authentification" 
    // {
    //     return authentificate(req).await;
    // }
    handle_request(req, remote_addr, rate_limiter, registry).await
}

async fn send(addr:  SocketAddr, req: Request<Incoming>) -> Result<Response<Incoming>, GatewayError>
{
    
    logger::info!("Отправка запроса на {}", req.uri());
    let client_stream = TcpStream::connect(&addr).await;
    if client_stream.is_err()
    {
        logger::error!("Ошибка подключения к сервису {} -> {}", addr, client_stream.err().unwrap());
        return Err(GatewayError::SendError(addr.to_string()));
    }
    let io = TokioIo::new(client_stream.unwrap());
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move 
    {
        if let Err(err) = conn.await 
        {
            logger::error!("Ошибка подключения: {:?}", err);
        }
    });
    let send = sender.send_request(req).await?;
    Ok(send)
    
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    logger::StructLogger::initialize_logger();
    let rate_limiter = Arc::new(RateLimiter::new());
    let registry = Arc::new(ServiceRegistry::new());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    loop 
    {
        let rate_limitter_clone = Arc::clone(&rate_limiter);
        let registry_clone = Arc::clone(&registry);
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move 
        {
            logger::info!("Запрос от {}", &remote_addr);
            let service = hyper::service::service_fn(move |req| 
            {
                router(
                req,
                remote_addr,
                Arc::clone(&rate_limitter_clone),
                Arc::clone(&registry_clone))
            });
            //let svc = ServiceBuilder::new().layer_fn(RateLimiter::new()).service(service);
            // if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
            //     eprintln!("server error: {}", err);
            // }

            if let Err(err) = http1::Builder::new().keep_alive(true).serve_connection(io, service).await 
            {
                logger::error!("server error: {}", err);
            }
        });
    }
}


// impl Service<Request<Incoming>> for RateLimiter
// {
//     type Response = Response<Full<Bytes>>;
//     type Error = hyper::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

//     fn call(&self, req: Request<Incoming>) -> Self::Future 
//     {
//         fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> 
//         {
//             Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
//         }
        
//         if !self.allow(remote_addr) 
//         {
//             return Ok(Response::builder()
//                 .status(StatusCode::TOO_MANY_REQUESTS)
//                 .body(to_body(Bytes::from_static(b"Too many requests")))
//                 .unwrap());
//         }

//         if req.uri().path() != "/favicon.ico" {
//             *self.counter.lock().expect("lock poisoned") += 1;
//         }

//         let res = match req.uri().path() {
//             "/" => mk_response(format!("home! counter = {:?}", self.counter)),
//             "/posts" => mk_response(format!("posts, of course! counter = {:?}", self.counter)),
//             "/authors" => mk_response(format!(
//                 "authors extraordinare! counter = {:?}",
//                 self.counter
//             )),
//             _ => mk_response("oh no! not found".into()),
//         };
//         Box::pin(async { res })
//     }
// }

#[cfg(test)]
mod tests
{
   
}