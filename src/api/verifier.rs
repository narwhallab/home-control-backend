use std::future::{Ready, ready};

use actix_web::{Responder, HttpResponse, web:: Form, post, HttpRequest, FromRequest, error::ErrorUnauthorized, http::header};
use jsonwebtoken::{Header, EncodingKey, get_current_timestamp, Validation, Algorithm, DecodingKey};
use serde::{Serialize, Deserialize};
use serde_json::json;
use super::util::{encrypt, parse_cookies};
use crate::PASSWORD;

pub struct AuthToken;

pub struct CookieToken {
    pub authorized: bool
}

impl FromRequest for AuthToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            return ready(Err(ErrorUnauthorized("No authorization header")));
        }
        

        // todo fix bearer, Bearer, ... caps
        let header_str = auth_header.unwrap().to_str().unwrap();
        if header_str[..=6].to_lowercase() != "bearer " {
            return ready(Err(ErrorUnauthorized("Invalid authorization header")));
        }
    
        match jsonwebtoken::decode::<Claims>(&header_str[7..], &DecodingKey::from_secret("super-secret-key".as_ref()), &Validation::new(Algorithm::HS256)) {
            Ok(_data) => ready(Ok(AuthToken)),
            _ => ready(Err(ErrorUnauthorized("Invalid token")))
        }
    }
}

impl FromRequest for CookieToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let cookie_header = req.headers().get(header::COOKIE);
        if cookie_header.is_none() {
            return ready(Ok(CookieToken { authorized: false }));
        }
        
        let header_str = cookie_header.unwrap().to_str().unwrap();

        let cookies = parse_cookies(header_str);

        if !cookies.contains_key("access_key") {
            return ready(Ok(CookieToken { authorized: false }));
        }
    
        match jsonwebtoken::decode::<Claims>(&cookies["access_key"], &DecodingKey::from_secret("super-secret-key".as_ref()), &Validation::new(Algorithm::HS256)) {
            Ok(_data) => ready(Ok(CookieToken { authorized: true })),
            _ => ready(Ok(CookieToken { authorized: false }))
        }
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
pub async fn login(req: Form<LoginRequest>) -> impl Responder {
    if encrypt(&req.password) != PASSWORD {
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

    HttpResponse::Found()
        .append_header(("Set-Cookie", format!("access_key={}", token)))
        .append_header(("Location", "/")).finish()
}