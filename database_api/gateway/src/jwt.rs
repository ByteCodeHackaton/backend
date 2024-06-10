use std::collections::HashMap;

use jsonwebtoken::{
    decode, encode, errors::Error, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, TokenData, Validation
};
use once_cell::sync::Lazy;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use time::{Duration, OffsetDateTime};

use crate::error::GatewayError;
pub struct JWT
{
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    public_key: Vec<u8>,
    algo: Algorithm,
    pairs: HashMap<String, String>,
    expired: HashMap<String, Vec<String>>
}
pub static KEY: Lazy<Mutex<JWT>> = Lazy::new(|| 
{
    let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());
    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let public_key = pair.public_key().as_ref();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());
    Mutex::new(JWT
    { 
        encoding_key,
        decoding_key,
        public_key: public_key.to_vec(),
        algo: Algorithm::EdDSA,
        pairs: HashMap::new(),
        expired: HashMap::new(),

    })
});

impl JWT
{
    pub fn new_access(&self, user_id: &str) -> String
    {
        let iat =  OffsetDateTime::now_utc();
        let exp = iat + Duration::minutes(5);
        let claims = Claims { sub: "test".to_string(), exp: exp, iat: iat, id: user_id.to_owned()};
        encode(&jsonwebtoken::Header::new(self.algo.clone()), &claims, &self.encoding_key).unwrap()
    }
    pub fn new_refresh(&self, user_id: &str) -> String
    {
        let iat =  OffsetDateTime::now_utc();
        let exp = iat + Duration::hours(12);
        let claims = Claims { sub: "test".to_string(), exp: exp, iat: iat, id: user_id.to_owned()};
        encode(&jsonwebtoken::Header::new(self.algo.clone()), &claims, &self.encoding_key).unwrap()
    }
    pub fn get_pair(&mut self, user_id: &str) -> (String, String)
    {
        let access = self.new_access(user_id);
        let refresh = self.new_refresh(user_id);
        self.pairs.insert(refresh.clone(), access.clone());
        (refresh, access)
    }
    pub fn update_keys(&mut self, key: &str) -> Result<(String, String), GatewayError>
    {
        let refresh = self.validate(key)?;
        let id = &refresh.claims.id;
        if self.pairs.contains_key(key)
        {
            if let Some(exp) = self.expired.get_mut(id)
            {
                exp.push(key.to_owned());
            }
            else 
            {
                self.expired.insert(id.clone(), vec![key.to_owned()]);
            }
            let pairs = self.get_pair(id);
            self.pairs.remove(key);
            Ok(pairs)
        }
        //ключ уже обновлялся по этому refresh key, необходимо удалить все связанные данные, возможно ключ попал к злоумышленнику
        else
        {
            if let Some(user_expired_keys) = self.expired.get(id)
            {
                for k in user_expired_keys
                {
                    self.pairs.remove(k);
                }
            }
            self.expired.remove(id);
            return Err(GatewayError::JWTRefreshError(["для юзера ", id, " рефреш токен уже обновлялся, все данные сброшены, вам необходимо заново залогиниться в систему"].concat()));
        }
    }
    pub fn validate(&self, token: &str) -> Result<TokenData<Claims>, Error>
    {
        let validation = Validation::new(self.algo.clone());
        decode::<Claims>(token, &self.decoding_key, &validation)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims 
{
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    id: String,
}

fn get_keys()
{
    let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());
    let iat =  OffsetDateTime::now_utc();
    let exp = iat + Duration::minutes(5);
    let claims = Claims { sub: "test".to_string(), exp: exp, iat: iat, id: "id юзера".to_owned()};

    let token =
        encode(&jsonwebtoken::Header::new(Algorithm::EdDSA), &claims, &encoding_key).unwrap();

    let validation = Validation::new(Algorithm::EdDSA);
    let _token_data = decode::<Claims>(&token, &decoding_key, &validation).unwrap();
    logger::debug!("{}", _token_data.claims.id);
}

mod jwt_numeric_date 
{
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

#[cfg(test)]
mod tests
{
    #[tokio::test]
    async fn gen_key()
    {
        logger::StructLogger::initialize_logger();
        let mut key  = super::KEY.lock().await;
        let id = "1234".to_owned();
        let pairs = key.get_pair(&id);
        logger::info!("refr: {} access: {}", &pairs.0, &pairs.1);
        let upd = key.update_keys(&pairs.0).unwrap();
        logger::info!("upd_refr: {} upd_access: {}", &upd.0, &upd.1);
        let upd2 = key.update_keys(&pairs.0);
        logger::info!("upd_refr_err:{}", &upd2.err().unwrap());
        logger::info!("{:?}, {:?}", key.expired, key.pairs);
    }
}