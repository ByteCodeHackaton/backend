use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
pub struct RateLimiter 
{
    visitors: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl RateLimiter 
{
    pub fn new() -> Self 
    {
        RateLimiter 
        {
            visitors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    ///не больше 5 штук
    pub fn allow(&self, addr: SocketAddr) -> bool 
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