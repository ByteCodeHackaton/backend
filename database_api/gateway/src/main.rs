mod error;

use hyper::{body, server::conn::http1, service::Service, Uri};
use hyper_util::{client::legacy::{Client, connect::HttpConnector}, rt::TokioExecutor, rt::TokioIo};
use error::GatewayError;
use http_body_util::{BodyExt, Full};
use hyper::body::Body;
use hyper::header;
use hyper_tls::HttpsConnector;
use tokio::net::{TcpListener, TcpStream};
use std::{collections::HashMap, future::Future, pin::Pin};
use std::sync::{Arc, Mutex, RwLock};
use std::net::SocketAddr;
use std::time::Duration;
use hyper::{body::{Bytes, Incoming}, Request, Response, StatusCode};
//use hyper::client::HttpConnector;
use hyper::service::{service_fn};
use serde_json::json;
use jsonwebtoken::{decode, DecodingKey, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;

const SECRET_KEY: &'static str = "secret_key";
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceConfig 
{
    name: String,
    address: String,
    endpoints: Vec<Endpoint>
}

impl ServiceConfig
{
    fn get_endpoint(&self, path: &str) -> Option<&Endpoint> 
    {
        self.endpoints.iter().find(|f| &f.path == path)
    }
}
#[derive(Debug)]
struct ServiceRegistry 
{
    services: Arc<RwLock<HashMap<String, ServiceConfig>>>,  // Service Name -> Service Address (URL/URI)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Endpoint
{
    path: String,
    authorization: bool
}

impl ServiceRegistry 
{
    fn new() -> Self 
    {
        ServiceRegistry 
        {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn register(&self, name: String, config: ServiceConfig) 
    {
        let mut services = self.services.write().unwrap();
        services.insert(name, config);
    }

    fn deregister(&self, name: &str) 
    {
        let mut services = self.services.write().unwrap();
        services.remove(name);
    }

    fn get_address(&self, name: &str) -> Option<String> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.address.clone()))
    }
    fn get_endpoints(&self, name: &str) -> Option<Vec<Endpoint>> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.endpoints.clone()))
    }
    fn get_config(&self, name: &str) -> Option<ServiceConfig> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.clone()))
    }
}

//непонятно можно просто типы проставить?
async fn register_service(req: Request<Incoming>, registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let config: Result<ServiceConfig, serde_json::Error> = serde_json::from_slice(&body);
    if config.is_err()
    {
        let body_str = String::from_utf8_lossy(&body);
        logger::error!("Неверный формат для регистрации сервиса -> {}, {}", body_str, config.err().unwrap());
        let resp = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(to_body(Bytes::from("Неверный формат для регистрации сервиса, необходим формат: '{ \"name\": string, \"address\": string, \"endpoints\": [ { \"path\": string, \"authorization\": boolean } ] }'")))?;
        return  Ok(resp);    
    }
    let config = config.unwrap();
    let service_name = config.name.clone();
    registry.register(service_name.clone(), config);
    logger::debug!("{:?}", &registry);
    let resp = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(to_body(Bytes::from(format!("Сервис {} успешно зарегистрирован", service_name))))?;
        return  Ok(resp);
   
}

async fn deregister_service(req: Request<Incoming>, registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let name = String::from_utf8_lossy(&body).to_string();
    registry.deregister(&name);
    let resp = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(to_body(Bytes::from(format!("Сервис {} успешно удален", name))))?;
    return  Ok(resp);   
}

struct RateLimiter 
{
    visitors: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl RateLimiter 
{
    fn new() -> Self 
    {
        RateLimiter 
        {
            visitors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    ///не больше 5 штук
    fn allow(&self, addr: SocketAddr) -> bool 
    {
        let mut visitors = self.visitors.lock().unwrap();
        let counter = visitors.entry(addr).or_insert(0);
        if *counter >= 5 
        { 
            false
        } 
        else 
        {
            *counter += 1;
            true
        }
    }
}

fn authenticate(token: &str) -> bool 
{
    //по умолчанию алгоритм Algorithm::HS256
    let mut validation = Validation::default();
    validation.set_audience(&["Me"]);
    match decode::<serde_json::Value>(&token, &DecodingKey::from_secret(SECRET_KEY.as_ref()), &validation) 
    {
        Ok(_data) => true,
        Err(err) => 
        {
            eprintln!("JWT Decoding error: {:?}", err);
            match *err.kind() 
            {
                ErrorKind::InvalidToken => false,  // ошибка токена
                _ => false
            }
        }
    }
}

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
    logger::info!("Отправка запроса на {}", req.uri());
    let req = Request::new(req.boxed());
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

    println!("Получен запрос от {}:{}", remote_addr.ip(), remote_addr.port());
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

    let path = req.uri().path().strip_prefix('/');
    if path.is_none()
    {
        return Ok(error_response(format!("Ошибка запроса {}, указан неверный путь к сервису",  req.uri().path()), StatusCode::BAD_REQUEST));
    }
    let path = path.unwrap();
    // если слеша нет то сервиса в запросе нет
    let parts = path.split_once('/');
    if parts.is_none()
    {
        return Ok(error_response(format!("Ошибка запроса {}, не уточнен сервис к которому производится запрос", path), StatusCode::BAD_REQUEST));
    }
    let (service_name, path) = parts.unwrap();
    logger::info!("запрос сервиса {} с эндпоинтом {}", service_name, path);
    match registry.get_config(service_name) 
    {
        Some(address) => 
        {
            if let Some(endpoint) = address.get_endpoint(path)
            {
                //TODO если авторизация необходима то реализовать проверку на этом этапе как раз для этого endpoint и получали
                let uri = Uri::builder()
                .scheme("http")
                .authority(address.address.clone())
                .path_and_query(["/", &endpoint.path].concat()).build().unwrap();
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
    logger::info!("адрес запроса {}", path);
    if path == "/register_service" 
    {
        return register_service(req, Arc::clone(&registry)).await;
    }

    if path == "/deregister_service" 
    {
        return deregister_service(req, Arc::clone(&registry)).await;
    }
    // Handle other requests using the previously defined handler
    handle_request(req, remote_addr, rate_limiter, registry).await
}

async fn send(addr:  SocketAddr, req: Request<BoxBody>) -> Result<Response<Incoming>, GatewayError>
{
    
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

fn error_response(err: String, code: StatusCode) -> Response<BoxBody>
{
    Response::builder()
    .status(code)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .body(to_body(Bytes::from(err))).unwrap()
}
fn error_empty_response(code: StatusCode) -> Response<BoxBody>
{
    Response::builder()
    .status(code)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .body(to_body(Bytes::new())).unwrap()
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


fn full<T: Into<Bytes>>(chunk: T) -> BoxBody 
{
    Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
    .boxed()
}   