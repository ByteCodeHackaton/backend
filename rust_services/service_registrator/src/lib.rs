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
    authorization: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>
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
            authorization: need_authorization,
            params: None,
            method: None,
            body: None,
            description: None
        });
        self
    }
    pub fn add_endpoint_body(mut self, path: &str, need_authorization: bool, body: &str, description: &str) -> Self
    {
        self.endpoints.push(Endpoint
        {
            path: path.to_owned(),
            authorization: need_authorization,
            params: None,
            method: Some("POST".to_owned()),
            body: Some(body.to_owned()),
            description: Some(description.to_owned())
        });
        self
    }
    pub fn add_endpoint_params(mut self, path: &str, need_authorization: bool, params: &[&str], description: &str) -> Self
    {
        self.endpoints.push(Endpoint
        {
            path: path.to_owned(),
            authorization: need_authorization,
            params: Some(params.iter().map(|v| v.to_string()).collect()),
            method: None,
            body: None,
            description: Some(description.to_owned())
        });
        self
    }
    pub fn as_json(&self) -> String
    {
        serde_json::to_string(self).unwrap()
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
    use serde_json::json;

    #[tokio::test]
    async fn test_reg()
    {
        logger::StructLogger::initialize_logger();
        let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let reg = super::ServiceConfig::new("subway", "localhost:8888")
        .add_endpoint_params("nearest", false, &["id", "time"], "Получение ближайших станций по времени")
        .add_endpoint("stations", false)
        .add_endpoint("path", false)
        .register(reg_service_addr).await;
        let body = reg.unwrap().collect().await.unwrap().to_bytes();
        let name = String::from_utf8_lossy(&body).to_string();
        logger::info!("{:?}", name);
    }
    #[test]
    fn test_json()
    {
        logger::StructLogger::initialize_logger();
        let reg = super::ServiceConfig::new("subway", "localhost:8888")
        .add_endpoint_params("nearest", false, &["id", "time"], "Получение ближайших станций по времени")
        .add_endpoint("stations", false)
        .add_endpoint_body("test", false, &json!({"name": "test_name", "value": "test_value"}).to_string(), "тестовый путь с jsonom")
        .add_endpoint("path", false)
        .as_json();
        logger::info!("{:?}", reg);
    }

    #[test]
    fn test_addr()
    {
        let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        logger::StructLogger::initialize_logger();
        logger::info!("{}", reg_service_addr);
    }
}