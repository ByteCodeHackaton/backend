use hyper::{body::{Incoming, Bytes}, Request, Response, Uri};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::OrderError;




pub async fn post<I: Serialize, O: DeserializeOwned>(uri: Uri, obj: &I) -> Result<O, OrderError>
{
    let host = uri.authority().unwrap().as_str().replace("localhost", "127.0.0.1");
    let req = Request::builder()
    .method("GET")
    .uri(&uri)
    .header(HOST, "localhost")
    .body(to_body(Bytes::from(serde_json::to_string(&obj).unwrap())))
    .unwrap();
    logger::info!("Отправка запроса на {}, headers: {:?}", req.uri(), req.headers());
    let addr: SocketAddr = host.parse().unwrap();
    let client_stream = TcpStream::connect(&addr).await;
    if client_stream.is_err()
    {
        logger::error!("Ошибка подключения к сервису {} -> {}", addr, client_stream.err().unwrap());
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
    let body = req.collect().await?.to_bytes();
    let crendentials: Result<Crendentials, serde_json::Error> = serde_json::from_slice(&body);
    if crendentials.is_err()
    {
        let str = String::from_utf8_lossy(&body);
        logger::error!("Неверный формат для авторизации ({}) -> {}", str, crendentials.err().unwrap());
        let resp = error_response(["Неверный формат для авторизации", str.as_ref(), ", необходим формат: '{ \"login\": string, \"password\": string}"].concat(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    Ok(send)
}