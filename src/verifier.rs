use std::future::{Ready, ready};

use actix_web::{Responder, HttpResponse, web::Json, post, HttpRequest, FromRequest, error::ErrorUnauthorized};
use jsonwebtoken::{Header, EncodingKey, get_current_timestamp, Validation, Algorithm, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::PASSWORD;

pub struct AuthToken {

}

impl FromRequest for AuthToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            return ready(Err(ErrorUnauthorized("No authorization header")));
        }
        
        let header_str = auth_header.unwrap().to_str().unwrap();
        if !header_str.starts_with("bearer ") {
            return ready(Err(ErrorUnauthorized("Invalid authorization header")));
        }
    
        if let Ok(data) = jsonwebtoken::decode::<Claims>(&header_str.replace("bearer ", ""), &DecodingKey::from_secret("super-secret-key".as_ref()), &Validation::new(Algorithm::HS256)) {
            return ready(Ok(AuthToken {}))
        }
    
        return ready(Err(ErrorUnauthorized("Invalid token")))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

#[post("/login")]
pub async fn login(req: Json<LoginRequest>) -> impl Responder {
    if req.password != PASSWORD {
        return HttpResponse::Unauthorized().json(json! {
            {
                "error": "Invalid password"
            }
        });
    }

    let claims = Claims {
        sub: "authorized_user".to_string(),
        exp: get_current_timestamp() + 60 * 60 * 24
    };

    let token = jsonwebtoken::encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret("super-secret-key".as_ref())).unwrap();


    HttpResponse::Ok().append_header(("Set-Cookie", format!("access_key={}", token))).json(json! {
        {
            "token": token
        }
    })
}