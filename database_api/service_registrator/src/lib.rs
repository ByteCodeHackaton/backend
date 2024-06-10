use std::{error::Error, net::SocketAddr};

use anyhow::anyhow;
use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, header, Request, Response, StatusCode, Uri};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Endpoint
{
    path: String,
    authorization: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig 
{
    name: String,
    address: String,
    endpoints: Vec<Endpoint>
}

impl ServiceConfig
{
    pub fn new(name: &str, address: &str) -> Self
    {
        Self 
        {
            name: name.to_owned(),
            address: address.to_owned(),
            endpoints: vec![]
        }
    }
    pub fn add_endpoint(mut self, path: &str, need_authorization: bool) -> Self
    {
        self.endpoints.push(Endpoint
        {
            path: path.to_owned(),
            authorization: need_authorization
        });
        self
    }
    /// addr - адрес gateway  
/// sc - конфигурация микросервиса, его эдпоинты с необходимостью авторизации
    pub async fn register(&self, addr:  SocketAddr) -> anyhow::Result<Response<Incoming>>
    {
        
        let client_stream = TcpStream::connect(&addr).await;
        if client_stream.is_err()
        {
            let error = format!("Ошибка подключения к сервису {} -> {}", addr, client_stream.err().unwrap());
            logger::error!("{}", &error);
            return Err(anyhow!(error));
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
        let service = serde_json::to_string(self).unwrap();
        logger::debug!("Запрос регистрации сервиса: {}", &service);
        let uri = Uri::builder()
        .scheme("http")
        .authority(addr.to_string())
        .path_and_query("/register_service")
        .build().unwrap();
        let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(to_body(Bytes::from(service)))?;
        let send = sender.send_request(req).await?;
        Ok(send)
    }

}

fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
    .boxed()
}   

#[cfg(test)]
mod tests
{
    use std::net::SocketAddr;

    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_reg()
    {
        logger::StructLogger::initialize_logger();
        let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let reg = super::ServiceConfig::new("subway", "localhost:8888")
        .add_endpoint("nearest", false)
        .add_endpoint("stations", false)
        .add_endpoint("path", false)
        .register(reg_service_addr).await;
        let body = reg.unwrap().collect().await.unwrap().to_bytes();
        let name = String::from_utf8_lossy(&body).to_string();
        logger::info!("{:?}", name);
    }

    #[test]
    fn test_addr()
    {
        let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        logger::StructLogger::initialize_logger();
        logger::info!("{}", reg_service_addr);
    }
}