mod error;
mod jwt;
mod authentification;
mod registry;
mod rate_limiter;
mod body_helpers;
mod cors;
mod request;

use authentification::{authentificate, get_claims, update_tokens, verify_token};
use body_helpers::{empty_response, error_empty_response, error_response, json_response, ok_response, unauthorized_response, BoxBody};
use hyper::header::{HeaderValue, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, HOST};
use hyper::{HeaderMap, Method};
use hyper::{server::conn::http1, Uri};
use error::GatewayError;
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use jwt::{Claims, KEY};
use rate_limiter::RateLimiter;
use registry::{deregister_service, get_all_services_configs, register_service, Endpoint, ServiceConfig, ServiceRegistry};
use tokio::net::{TcpListener, TcpStream};
use tower::ServiceBuilder;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use std::net::SocketAddr;
use hyper::{body::Incoming, Request, Response, StatusCode};


async fn service_handler(req: Request<Incoming>, claims: Option<Claims>) -> Result<Response<BoxBody>, GatewayError>
{
    // не вижу смысла пока отправлять публичный ключ и токен на микросервисы для идентификации, просто оправлю user_id ну и что то еще если будет нужно, если будет большее количество микросервисов можно будет добавить связку публичный ключ\access token
    // let pc = {
    //     let key = KEY.lock().await;
    //     key.get_public_key()
    // };
    let request = 
    {
        if let Some(cl) = claims
        {
            Request::builder()
            .method(req.method())
            .uri(req.uri())
            .header("User-Id", cl.user_id())
            .header(HOST, "localhost")
            .body(req.into_body())
            .unwrap()
        }
        else 
        {
            Request::builder()
            .method(req.method())
            .uri(req.uri())
            .header(HOST, "localhost")
            .body(req.into_body())
            .unwrap()
        }
    };
    let auth = request.uri().authority();
    let target_host = auth.unwrap().as_str().replace("localhost", "127.0.0.1");
    let addr: SocketAddr = target_host.parse().unwrap();
    // Отправка запроса на связанный сервис
    let response = crate::request::send_request(addr, request).await?;
    //let headers = response.headers_mut();
    //headers.append(ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS".parse().unwrap());
    //headers.append(ACCESS_CONTROL_ALLOW_HEADERS, "User-Id".parse().unwrap());
    //headers.append(ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173".parse().unwrap());
    //headers.append(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
    // трансформация body иньекция в текущий json
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


async fn check_services_path(mut req: Request<Incoming>,  registry: Arc<ServiceRegistry>) -> Result<(Request<Incoming>, Endpoint), Response<BoxBody>>
{
    let path = req.uri().path().strip_prefix('/');
    if path.is_none()
    {
        return Err(error_response(format!("Ошибка запроса {}, указан неверный путь к сервису",  req.uri()), StatusCode::BAD_REQUEST));
    }
    let path = path.unwrap();
    //разделяем путь между сервисом и дальнейшим путем
    let parts = path.split_once('/');
    if parts.is_none()
    {
        return Err(error_response(format!("Ошибка запроса {}, не уточнен сервис к которому производится запрос", path), StatusCode::BAD_REQUEST));
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
                //если сервис требует авторизации, то проверяем авторизован ли юзер и отправляем unauthorized если нет
                *req.uri_mut() = uri;
                return Ok((req, endpoint.clone()));
                // if endpoint.need_authorization()
                // {
                //     let claims = get_claims(&req).await;
                //     if claims.is_none()
                //     {
                //         return Err(unauthorized_response());
                //     }
                //     logger::info!("Запрос переадресован на (авторизованная зона) {}", &uri_str);
                //     return service_handler(req, claims).await;
                // }
                // logger::info!("Запрос переадресован на {}", &uri_str);
                // return service_handler(req, None).await;
            }
            else 
            {
                return Err(error_response(format!("Ошибка в сервисе {} не найден путь {}", service_name, path), StatusCode::BAD_REQUEST));
            }
        },
        None => return Err(error_response(format!("Сервис {} не найден", service_name), StatusCode::NOT_FOUND)),
    };
}

async fn handle_request(
    mut req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    registry: Arc<ServiceRegistry>,
) -> Result<Response<BoxBody>, GatewayError> 
{
    let res = check_services_path(req, registry).await;
    if res.is_err()
    {
        return Ok(res.err().unwrap());
    }
    let (req, endpoint) = res.unwrap();
    if endpoint.need_authorization()
    {
        let claims = get_claims(&req).await;
        if claims.is_none()
        {
            return Ok(unauthorized_response());
        }
        logger::info!("Запрос переадресован на (авторизованная зона) {}", req.uri());
        return service_handler(req, claims).await;
    }
    logger::info!("Запрос переадресован на {}", req.uri());
    return service_handler(req, None).await;
    // if !rate_limiter.allow(remote_addr) 
    // {
    //     return Ok(error_empty_response(StatusCode::TOO_MANY_REQUESTS));
    // }
}

async fn router(
    req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError>
{   
    if req.method() == &Method::OPTIONS
    {
        return Ok(empty_response(StatusCode::OK));
    }
    let path = req.uri().path();
    if path == "/register_service" 
    {
        return register_service(req, Arc::clone(&registry)).await;
    }
    if path == "/deregister_service" 
    {
        return deregister_service(req, Arc::clone(&registry)).await;
    }
    if path == "/authentification" 
    {
        return authentificate(req).await;
    }
    if path == "/authentification/status" 
    {
        return verify_token(&req).await;
    }
    if path == "/authentification/refresh" 
    {
        return update_tokens(req).await;
    }
    if path == "/services_list" 
    {
        return services_list(Arc::clone(&registry)).await;
    }
    handle_request(req, remote_addr, rate_limiter, registry).await
}

pub async fn services_list(registry: Arc<ServiceRegistry>,) -> Result<Response<BoxBody>, GatewayError> 
{
    let resp = json_response(&get_all_services_configs(registry));
    return  Ok(resp);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    logger::StructLogger::initialize_logger();
    let registry = Arc::new(ServiceRegistry::try_from_exists());
    let rate_limiter = Arc::new(RateLimiter::new());
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
            let svc = tower::service_fn(move |req| 
            {
                router(
                req,
                remote_addr,
                Arc::clone(&rate_limitter_clone),
                Arc::clone(&registry_clone))
            });
            let service = ServiceBuilder::new()
                .buffer(5)
                .concurrency_limit(5)
                .rate_limit(5, Duration::from_secs(1))
                .layer(cors::cors_layer())
                .service(svc);
            let service = TowerToHyperService::new(service);
            if let Err(err) = http1::Builder::new().keep_alive(true).serve_connection(io, service).await 
            {
                logger::error!("server error: {}", err);
            }
        });
    }
}


#[cfg(test)]
mod tests
{
    use hyper::Uri;

    use crate::registry::{Endpoint, ServiceConfig};
    #[test]
   fn test_fint_path()
   {
        let test_uri = "http://localhost:8080/db_service/api/v1/employee/list?limit=20&off=0".parse::<Uri>().unwrap();
        let path = test_uri.path().strip_prefix('/');
        let path = path.unwrap();
        println!("{}", path);
        //разделяем путь между сервисом и дальнейшим путем
        let parts = path.split_once('/');
        let (service_name, path) = parts.unwrap();
        println!("{} {}", service_name, path);
        let service = ServiceConfig
        {
            name: "db_service".to_owned(),
            address: "localhost:5412".to_owned(),
            endpoints: vec![
                Endpoint
                {
                    path: "/api/v1/employee/list".to_owned(),
                    authorization: false,
                    params: None,
                    body: None,
                    method: Some("POST".to_owned()),
                    description: None
                }
            ]

        };
        let end = service.get_endpoint(path).unwrap();
        println!("{}", end.path);
        let sn = ["db_service", "/"].concat();
        let p_q = test_uri.path_and_query().unwrap().to_string().replace(&sn, "");
        let uri = Uri::builder()
        .scheme("http")
        .authority("localhost:5050")
        .path_and_query(p_q).build().unwrap();
        //если сервис требует авторизации, то проверяем авторизован ли юзер и отправляем unauthorized если нет
        let uri_str = uri.to_string();
        println!("{}", uri_str);
   }
}