mod error;

use hyper::server::conn::{http1};
use hyper_util::{client::legacy::{Client, connect::HttpConnector}, rt::TokioExecutor, rt::TokioIo};
use error::GatewayError;
use http_body_util::{BodyExt, Full};
use hyper::body::Body;
use hyper::header;
use hyper_tls::HttpsConnector;
use tokio::net::TcpListener;
use std::collections::HashMap;
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
#[derive(Debug, Serialize, Deserialize)]
struct ServiceConfig 
{
    name: String,
    address: String,
}

struct ServiceRegistry 
{
    services: Arc<RwLock<HashMap<String, String>>>,  // Service Name -> Service Address (URL/URI)
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

    fn register(&self, name: String, address: String) 
    {
        let mut services = self.services.write().unwrap();
        services.insert(name, address);
    }

    fn deregister(&self, name: &str) 
    {
        let mut services = self.services.write().unwrap();
        services.remove(name);
    }

    fn get_address(&self, name: &str) -> Option<String> 
    {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
}

//непонятно можно просто типы проставить?
async fn register_service(req: Request<Incoming>, registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let body_str = String::from_utf8_lossy(&body);
    let parts: Vec<&str> = body_str.split(',').collect();
    if parts.len() != 2 
    {
        let resp = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(to_body(Bytes::from("Неверный формат для регистрации сервиса, формат: 'name,address'")))?;
        return  Ok(resp);    
    }
    let name = parts[0].to_string();
    let address = parts[1].to_string();
    registry.register(name.clone(), address);
    let resp = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(to_body(Bytes::from(format!("Сервис {} успешно зарегистрирован", name))))?;
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
            .body(to_body(Bytes::from(format!("Сервис {} успешно удален из активных", name))))?;
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

async fn service_handler(req: Request<Incoming>,  client: Arc<Client<HttpsConnector<HttpConnector>, BoxBody>>,) -> Result<Response<BoxBody>, GatewayError>
{
    // Example of request transformation: Adding a custom header
    let req = Request::builder()
        .method(req.method())
        .uri(req.uri())
        .header("X-Custom-Header", "My API Gateway")
        .body(req.into_body())
        .unwrap();

    // Отправка запроса на связанный сервис
    println!("Отправка запроса на {}", req.uri());
    let req = Request::new(req.boxed());
    let resp = client.request(req).await?;

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

    Ok(Response::new(resp.boxed()))
}

/*async fn handle_request(req: Request<Body>, rate_limiter: Arc<RateLimiter>, client: Arc<hyper::Client<HttpsConnector<HttpConnector>>>, service_registry: &ServiceRegistry) -> Result<Response<Body>, hyper::Error> {*/
async fn handle_request(
    mut req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    client: Arc<Client<HttpsConnector<HttpConnector>, BoxBody>>,
    registry: Arc<ServiceRegistry>,
) -> Result<Response<BoxBody>, GatewayError> 
{
    if !rate_limiter.allow(remote_addr) 
    {
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(to_body(Bytes::from_static(b"Too many requests")))
            .unwrap());
    }

    println!("Получен запрос от {}:{}", remote_addr.ip(), remote_addr.port());

    let unauthorized_response = Response::builder()
    .status(StatusCode::UNAUTHORIZED)
    .body(to_body(Bytes::from_static(b"Unauthorized")))
    .unwrap();
    // Authentication
    match req.headers().get("Authorization") 
    {
        Some(value) => 
        {
            let token_str = value.to_str().unwrap_or("");
            if !authenticate(token_str) 
            {
                return Ok(unauthorized_response);
            }
        },
        None => 
        {
            return Ok(unauthorized_response);
        }
    }

    let path = req.uri().path();

    // первый путь это имя сервиса.
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() < 2 
    {
        return Ok(Response::new(to_body(Bytes::from_static(b"Invalid request URI"))));
    }

    let service_name = parts[1];
    match registry.get_address(service_name) 
    {
        Some(address) => {
            // используем адрес для перенаправления запроса.
            // Создаем новый адрес на основе полученного
            let mut address = address;
            if !address.starts_with("http://") && !address.starts_with("https://") 
            {
                address = format!("http://{}", address);
            }
            let forward_uri = format!("{}{}", address, req.uri().path_and_query().map_or("", |x| x.as_str()));

            if let Ok(uri) = forward_uri.parse() 
            {
                *req.uri_mut() = uri;
            } 
            else 
            {
                return Ok(Response::new(to_body(Bytes::from_static(b"Invalid service URI"))));
            }

            // отправляем запрос в service handler
            service_handler(req, client).await
        },
        None => return Ok(Response::new(to_body(Bytes::from_static(b"Service not found")))),
    }

}

async fn router(
    req: Request<Incoming>,
    remote_addr: SocketAddr,
    rate_limiter: Arc<RateLimiter>,
    client: Arc<Client<HttpsConnector<HttpConnector>, BoxBody>>,
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
    handle_request(req, remote_addr, rate_limiter, client, registry).await
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    logger::StructLogger::initialize_logger();
    let rate_limiter = Arc::new(RateLimiter::new());
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, BoxBody>(https);
    let client = Arc::new(client);
    let registry = Arc::new(ServiceRegistry::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    loop 
    {
        let rate_limitter_clone = Arc::clone(&rate_limiter);
        let client_clone = Arc::clone(&client);
        let registry_clone = Arc::clone(&registry);
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move 
        {
            logger::info!("Запрос от {}", &remote_addr);
            // N.B. should use hyper service_fn here, since it's required to be implemented hyper Service trait!
            let service = hyper::service::service_fn(move |req| 
            {
                router(
                req,
                remote_addr,
                Arc::clone(&rate_limitter_clone),
                Arc::clone(&client_clone),
                Arc::clone(&registry_clone))
            });
            
            if let Err(err) = http1::Builder::new().keep_alive(true).serve_connection(io, service).await 
            {
                logger::error!("server error: {}", err);
            }
        });
    }

    // Handle Requests
    // let make_svc = make_service_fn(move |conn: &AddrStream| {
    //     let remote_addr = conn.remote_addr();
    //     let rate_limiter = Arc::clone(&rate_limiter);
    //     let client = Arc::clone(&client);
    //     let registry_clone = Arc::clone(&registry);

    //     let service = service_fn(move |req| 
    //     {
    //         router(req, remote_addr, Arc::clone(&rate_limiter), Arc::clone(&client), Arc::clone(&registry_clone))
    //     });

    //     async { Ok::<_, hyper::Error>(service) }
    // });


    // let addr = ([127, 0, 0, 1], 8080).into();

    // let server = Server::bind(&addr)
    //     .http1_keepalive(true)
    //     .http2_keep_alive_timeout(Duration::from_secs(120))
    //     .serve(make_svc);

    // println!("API Gateway running on http://{}", addr);

    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }
}



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