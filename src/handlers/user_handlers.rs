use actix_web::{HttpResponse, Responder, HttpRequest};
use serde::Deserialize;
use actix_web::web;
use sqlx::PgPool;
use bcrypt::{hash, verify};
use crate::auth::jwt::{create_token, verify_token};
use crate::ratelimiter::limiter::RateLimiter;


#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from handler!")
}

pub async fn signup(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let hashed_password = match hash(&user.password, 10) {
    Ok(h) => h,
    Err(_) => return HttpResponse::InternalServerError().body("Hashing failed"),
};

    let result = sqlx::query(
        "INSERT INTO users (email, password) VALUES ($1, $2)"
    )
    .bind(&user.email)
    .bind(&hashed_password)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(err) => {
            println!("DB Error: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {

    let result = sqlx::query!(
        "SELECT email, password FROM users WHERE email = $1",
        user.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    let record = match result {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid email"),
        Err(_) => return HttpResponse::InternalServerError().body("DB error"),
    };

    let is_valid = match verify(&user.password, &record.password) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().body("Verify failed"),
    };

    if is_valid {
        HttpResponse::Ok().body("Login successful");
        let token = create_token(&record.email);
    HttpResponse::Ok().body(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid password")
    }
    
}

pub async fn protected_route(
    req: HttpRequest,
    limiter: web::Data<RateLimiter>,
) -> impl Responder {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or("unknown".to_string());

    if !limiter.check(ip) {
        return HttpResponse::TooManyRequests().body("Rate limit exceeded");
    }

    // 🔹 Token check
    let auth_header = req.headers().get("Authorization");

    let token = match auth_header {
        Some(header) => {
            let auth_str = header.to_str().unwrap_or("");
            auth_str.replace("Bearer ", "")
        }
        None => return HttpResponse::Unauthorized().body("No token"),
    };

    let claims = match verify_token(&token) {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    HttpResponse::Ok().body(format!("Welcome {}", claims.sub))
}