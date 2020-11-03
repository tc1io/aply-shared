use serde_json;

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

