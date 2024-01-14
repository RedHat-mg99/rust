use actix_web::delete;
use actix_web::{get, post, put, HttpResponse, Error, http::StatusCode, web};
use chrono::Utc;
use serde_json::json;

use crate::AppState;
use crate::models::model::{User, CreateUser, UpdateUser, AcionUser};

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

#[get("/all_users")]
pub async fn get_all_users(pool: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(match get_all_user(pool).await {
        Ok(users) => users,
        _ => HttpResponse::from(HttpResponse::InternalServerError()),
    }
    )
}

async fn get_all_user(pool: web::Data<AppState>) -> Result<HttpResponse, Error>{
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM usersdb"
    ).fetch_all(&pool.db).await;
    if query_result.is_err() {
        let message: &str = "Something bad happened while fetching list Users";
        return Ok(HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message})))
    }
    let users = query_result.unwrap();
    if users.capacity() == 0 {
        let message = format!("No user in the database");
        return Ok(HttpResponse::NotFound().json(json!({
            "status": "failed",
            "message": message,
        })));
    }
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "Number matched": users.len(),
        "users": users
    })))
}

#[get("/user")]
pub async fn get_user(pool: web::Data<AppState>, body: web::Json<AcionUser>) -> Result<HttpResponse, Error> {
    Ok(match get_one_user(pool, body).await {
        Ok(users) => users,
        _ => HttpResponse::from(HttpResponse::InternalServerError()),
    }
    )
}

async fn get_one_user(pool: web::Data<AppState>, body: web::Json<AcionUser>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM usersdb WHERE id = $1",
        body.id
    ).fetch_all(&pool.db).await;
    let user = query_result.unwrap();
    if user.capacity() == 0 {
        let message = format!("User with ID: {} not found", body.id);
        return Ok(HttpResponse::NotFound().json(json!({
            "status": "failed",
            "message": message,
        })));
    }
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "user": user
    })))
}

#[post("/user")]
pub async fn add_user(pool: web::Data<AppState>, body: web::Json<CreateUser>) -> Result<HttpResponse, Error> {
    let now = Utc::now();
    let query_result = sqlx::query_as!(
        User,
        "INSERT into usersdb (username, password, email, date_created) values ($1, $2, $3, $4) RETURNING *",
        body.username.to_string(),
        body.password.to_string(),
        body.email.to_string(),
        now,
    ).fetch_one(&pool.db)
    .await;
    match query_result {
        Ok(user) => {
            let response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({"content": user})
            });
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            if e.to_string().contains("duplicate") {
                return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!({
                        "status": "failed",
                        "message": "duplicated"
                    }))
            );
            }
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "status": "error",
                    "message" : format!("{:?}",e)
                })
            ));
        }
    }
}

#[put("/update")]
pub async fn update_user(pool: web::Data<AppState>, body: web::Json<UpdateUser>) -> Result<HttpResponse, Error> {
    let now = Utc::now();
    let query_result = sqlx::query_as!(
        User,
        "UPDATE usersdb SET (password, email, date_updated) = ($2, $3, $4) WHERE username = $1 RETURNING *",
        body.username.to_string(),
        body.password.to_string(),
        body.email.to_string(),
        now,
    ).fetch_one(&pool.db)
    .await;

    match query_result {
        Ok(user) => {
            let response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({"content": user})
            });
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            if e.to_string().contains("duplicate") {
                return Ok(HttpResponse::BadRequest()
                    .json(serde_json::json!({
                        "status": "failed",
                        "message": "duplicated"
                    }))
            );
            }
            return Ok(HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "status": "error",
                    "message" : format!("{:?}",e)
                })
            ));
        }
    }
}

#[delete("/user")]
pub async fn delete_user(pool: web::Data<AppState>, body: web::Json<AcionUser>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query!(
        "DELETE from usersdb WHERE id = $1", body.id)
    .execute(&pool.db)
    .await
    .unwrap()
    .rows_affected();
    if query_result == 0 {
        let message = format!("User with ID: {} not found", body.id);
        return Ok(HttpResponse::NotFound().json(json!({
            "status": "failed",
            "message": message,
        })));
    }
    return Ok(HttpResponse::NoContent().finish())
}