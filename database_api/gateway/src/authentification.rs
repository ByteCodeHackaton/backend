use crate::{body_helpers::*, GatewayError};
use crate::error_response;
use http_body_util::BodyExt;
use hyper::{body::Incoming, Request, Response, StatusCode};
use super::jwt::KEY;
use serde::{Deserialize, Serialize};

pub async fn is_autentificate(req: Request<Incoming>) -> bool
{
    
    match req.headers().get("Authorization") 
    {
        Some(value) => 
        {
            let token_str = value.to_str().unwrap_or("");
            let key = KEY.lock().await;
            let v = key.validate_access(token_str);
            if let Ok(_) = v
            {
                true
            }
            else 
            {
                let e = v.err().unwrap().to_string();
                logger::error!("{}", &e);
                false
            }
        },
        None => 
        {
            let e = "Отсуствует заголовок Authorization!";
            logger::error!("{}", e);
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTokens
{
    pub access: String,
    pub refresh: String
}
pub async fn update_tokens(req: Request<Incoming>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let tokens: Result<UpdateTokens, serde_json::Error> = serde_json::from_slice(&body);
   
    if tokens.is_err()
    {
        logger::error!("Неверный формат для обновления токенов -> {}", tokens.err().unwrap());
        let resp = error_response("Неверный формат для обновления токенов, необходим формат: '{ \"access\": string, \"refresh\": string}".to_owned(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    let tokens = tokens.unwrap();
    let mut keys = KEY.lock().await;
    let res = keys.update_keys(&tokens.refresh)?;
    let update_tokens = UpdateTokens
    {
        access: res.1,
        refresh: res.0
    };
    let resp = json_response(&update_tokens);
    return  Ok(resp);
}

//пока самое сомнительное решение запрашивать аутентификацию у строннего сервиса
//позже неужно добавить адрес эндпоинта для аутентификации в аргументы при старте сервиса
pub async fn authentificate(req: Request<Incoming>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let tokens: Result<UpdateTokens, serde_json::Error> = serde_json::from_slice(&body);
   
    if tokens.is_err()
    {
        logger::error!("Неверный формат для обновления токенов -> {}", tokens.err().unwrap());
        let resp = error_response("Неверный формат для обновления токенов, необходим формат: '{ \"access\": string, \"refresh\": string}".to_owned(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    let tokens = tokens.unwrap();
    let mut keys = KEY.lock().await;
    let res = keys.update_keys(&tokens.refresh)?;
    let update_tokens = UpdateTokens
    {
        access: res.1,
        refresh: res.0
    };
    let resp = json_response(&update_tokens);
    return  Ok(resp);
}
