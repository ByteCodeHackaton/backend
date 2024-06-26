use std::net::SocketAddr;

use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, header::{self, HOST}, Request, Response, StatusCode, Uri};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use crate::error::OrderError;
pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;


pub async fn post<I: Serialize, O>(uri: Uri, obj: &I) -> Result<O, OrderError> where for<'de> O: Deserialize<'de>
{
    let host = uri.authority().unwrap().as_str().replace("localhost", "127.0.0.1");
    let req = Request::builder()
    .method("POST")
    .uri(&uri)
    .header(HOST, "localhost")
    .body(to_body(Bytes::from(serde_json::to_string(&obj).unwrap())))
    .unwrap();
    logger::info!("Отправка запроса на {}, headers: {:?}", req.uri(), req.headers());
    let addr: SocketAddr = host.parse().unwrap();
    let client_stream = TcpStream::connect(&addr).await;
    if client_stream.is_err()
    {
        logger::error!("Ошибка подключения к сервису {} -> {}", &addr, client_stream.err().unwrap());
        return Err(OrderError::SendError(addr.to_string()));
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
    let body = send.collect().await?.to_bytes();
    let response: O = serde_json::from_slice(&body)?;

    Ok(response)
}

pub async fn get<O>(uri: Uri) -> Result<O, OrderError> where for<'de> O: Deserialize<'de>
{
    let host = uri.authority().unwrap().as_str().replace("localhost", "127.0.0.1");
    let req = Request::builder()
    .method("GET")
    .uri(&uri)
    .header(HOST, "localhost")
    .body(to_body(Bytes::new()))
    .unwrap();
    logger::info!("Отправка запроса на {}, headers: {:?}", req.uri(), req.headers());
    let addr: SocketAddr = host.parse().unwrap();
    let client_stream = TcpStream::connect(&addr).await;
    if client_stream.is_err()
    {
        logger::error!("Ошибка подключения к сервису {} -> {}", &addr, client_stream.err().unwrap());
        return Err(OrderError::SendError(addr.to_string()));
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
    if send.status() == StatusCode::OK
    {
        let body = send.collect().await?.to_bytes();
        //logger::debug!("{}", String::from_utf8_lossy(&body));
        let response: O = serde_json::from_slice(&body)?;
        Ok(response)
    }
    else
    {
        logger::error!("Ошибка получения инфомации от сервиса {} -> {}", &addr, send.status());
        return Err(OrderError::SendError(format!("Ошибка получения инфомации от сервиса {} -> {}", &addr, send.status())));
    }
   
}



pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
    .boxed()
}  
pub fn empty_response(code: StatusCode) -> Response<BoxBody>
{
    Response::builder()
    .status(code)
    .body(to_body(Bytes::new())).unwrap()
}

pub fn error_response(err: String, code: StatusCode) -> Response<BoxBody>
{
    standart_headers()
    .status(code)
    .body(to_body(Bytes::from(err))).unwrap()
}
pub fn error_empty_response(code: StatusCode) -> Response<BoxBody>
{
    standart_headers()
    .status(code)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .body(to_body(Bytes::new())).unwrap()
}
pub fn ok_response(msg: String) -> Response<BoxBody>
{
    standart_headers()
    .status(StatusCode::OK)
    //.header(ACCESS_CONTROL_ALLOW_HEADERS, "User-Id")
    .body(to_body(Bytes::from(msg))).unwrap()
}
pub fn json_response<S: Serialize>(obj: &S) -> Response<BoxBody>
{
    let result = serde_json::to_string(obj).unwrap();
    standart_headers()
    .status(StatusCode::OK)
    .body(to_body(Bytes::from(result))).unwrap()
}

pub fn unauthorized_response() -> Response<BoxBody>
{
    standart_headers()
    .status(StatusCode::UNAUTHORIZED)
    .body(to_body(Bytes::from_static(b"Unauthorized")))
    .unwrap()
}

pub fn standart_headers() -> hyper::http::response::Builder
{
    Response::builder()
    //.header(ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173")
    //.header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    //.header(ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
}