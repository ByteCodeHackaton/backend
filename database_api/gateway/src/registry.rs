use http_body_util::BodyExt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use hyper::{body::Incoming, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use crate::{body_helpers::{error_response, ok_response, BoxBody}, error::GatewayError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig 
{
    name: String,
    address: String,
    endpoints: Vec<Endpoint>
}

impl ServiceConfig
{
    pub fn get_endpoint(&self, path: &str) -> Option<&Endpoint> 
    {
        if let Some((path, _)) = path.split_once("?")
        {
            self.endpoints.iter().find(|f| f.path.replace("/", "") == path.replace("/", ""))
        }
        else
        {
            //убирем слеши, так как не можем гарантировать что не будут различаться конечные слеши
            self.endpoints.iter().find(|f| f.path.replace("/", "") == path.replace("/", ""))
        }
    }
    pub fn get_name(&self) -> &str
    {
        &self.name
    } 
    pub fn get_address(&self) -> &str
    {
        &self.address
    } 
}
#[derive(Debug)]
pub struct ServiceRegistry 
{
    services: Arc<RwLock<HashMap<String, ServiceConfig>>>,  // Service Name -> Service Address (URL/URI)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint
{
    path: String,
    authorization: bool
}
impl Endpoint
{
    pub fn need_authorization(&self) -> bool
    {
        self.authorization
    }
    pub fn path(&self) -> &str
    {
        &self.path
    }
}

impl ServiceRegistry 
{
    pub fn new() -> Self 
    {
        ServiceRegistry 
        {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn try_from_exists() -> Self 
    {
        let services = utilites::read_file_to_binary("services.json");
        if services.is_ok()
        {
            let obj = serde_json::de::from_slice::<Vec<ServiceConfig>>(services.as_ref().unwrap());
            if obj.is_ok()
            {
                let mut hm : HashMap<String, ServiceConfig> = HashMap::new();
                for s in obj.unwrap()
                {
                    logger::info!("из файла services.json загружен сервис {:?}", &s.name);
                    hm.insert(s.name.clone(), s);
                }
                ServiceRegistry 
                {
                    
                   services: Arc::new(RwLock::new(hm))
                }
            }
            else
            {
                ServiceRegistry 
                {
                    services: Arc::new(RwLock::new(HashMap::new())),
                }
            }
        }
        else
        {
            ServiceRegistry 
            {
                services: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }
    pub fn save(&self)
    {
        let guard  = self.services.read().unwrap();
        let vec = guard.values().map(|v| v).collect::<Vec<&ServiceConfig>>();
        let _  = utilites::serialize_to_file(vec, "services.json", None);
    }

    pub fn register(&self, name: String, config: ServiceConfig) 
    {
        let mut services = self.services.write().unwrap();
        services.insert(name, config);
        drop(services);
        self.save();
    }

    pub fn deregister(&self, name: &str) 
    {
        let mut services = self.services.write().unwrap();
        services.remove(name);
        drop(services);
        self.save();
    }

    pub fn get_address(&self, name: &str) -> Option<String> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.address.clone()))
    }
    pub fn get_endpoints(&self, name: &str) -> Option<Vec<Endpoint>> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.endpoints.clone()))
    }
    pub fn get_config(&self, name: &str) -> Option<ServiceConfig> 
    {
        let services = self.services.read().unwrap();
        services.get(name).and_then(|a| Some(a.clone()))
    }
}

//непонятно можно просто типы проставить?
pub async fn register_service(req: Request<Incoming>, registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let config: Result<ServiceConfig, serde_json::Error> = serde_json::from_slice(&body);
    if config.is_err()
    {
        let body_str = String::from_utf8_lossy(&body);
        logger::error!("Неверный формат для регистрации сервиса -> {}, {}", body_str, config.err().unwrap());
        let resp = error_response("Неверный формат для регистрации сервиса, необходим формат: '{ \"name\": string, \"address\": string, \"endpoints\": [ { \"path\": string, \"authorization\": boolean } ] }'".to_owned(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    let config = config.unwrap();
    let service_name = config.name.clone();
    registry.register(service_name.clone(), config);
    logger::debug!("{:?}", &registry);
    let resp = ok_response(format!("Сервис {} успешно зарегистрирован", service_name));
    return  Ok(resp);
   
}

pub async fn deregister_service(req: Request<Incoming>, registry: Arc<ServiceRegistry>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let name = String::from_utf8_lossy(&body).to_string();
    registry.deregister(&name);
    let resp = ok_response(format!("Сервис {} успешно удален", name));
    return  Ok(resp);   
}
