use serde_json;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::result::Result;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub organisation: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Token {
    pub sub:String,
    pub name:String,
    pub organisation: String
}



pub fn user_from_token(token:String) -> User {
    let t: Token = serde_json::from_str(token.as_str()).unwrap();
    User { id: t.sub.clone() , name: t.name.clone() , organisation: t.organisation.clone() }
}

// TODO consolidate with the above?
pub fn user_from_bearer(token:&str) -> Option<User> {
    //serde_json::from_str(token).map(|t:Token| User { id: t.sub.clone() , name: t.name.clone() , organisation: t.organisation.clone() })
    serde_json::from_str(token).map(|t:Token| User { id: t.sub.clone() , name: t.name.clone() , organisation: t.organisation.clone() }).ok()
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    // company: String,
    exp: usize,
}

pub fn getsub(c:&Claims) -> String {
    c.sub.clone()
}

pub fn validate(token:String) -> Result<Claims,String> {

    let b = include_bytes!("pubkey.pem");
    ///     print!("{}", String::from_utf8_lossy(bytes));
    // let tok2 = encode(&Header::new(Algorithm::RS256), &claims, &EncodingKey::from_rsa_pem(bytes).expect("xx"));

    let tok = decode::<Claims>(token.as_str(), &DecodingKey::from_rsa_pem(b).expect("sdddd"), &Validation::new(Algorithm::RS256)).expect("not valid");

    Ok(tok.claims)
}

