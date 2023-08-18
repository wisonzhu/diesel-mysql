use actix_web::{get,post,delete,HttpResponse, Responder, web};
use crate::config::database::DbPool;
use crate::service::ping_service::{ping, getuser,listuser,create_userdata,delete_userdata};
use log::{error, info};
use serde::{Serialize, Deserialize};
use crate::model::user::User;


#[derive(Deserialize,Serialize)]
pub struct UserInfo {
    username: String,
}


#[get("/test")]
pub async fn hello(pool: web::Data<DbPool>) -> impl Responder {
    let pong = ping(&pool).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    info!("Succes in pinging database");
    HttpResponse::Ok().body(format!("Hello world! Succesfully connected to Database! Query Results: {}", &pong.unwrap().data))
}


#[get("/getuser")]
pub async fn get_userinfo(pool: web::Data<DbPool>) -> impl Responder {
    let pong = getuser(&pool).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    info!("Succes in pinging database");
    HttpResponse::Ok().body(format!("Hello world! Succesfully connected11 to Database! Query \
    Results: {}", &pong.unwrap().data))
}


#[get("/listuser")]
pub async fn list_users(pool: web::Data<DbPool>) ->  impl Responder {
    let pong = listuser(&pool).await.map_err(|_e| {
        error!("Error in pinging database");
        HttpResponse::InternalServerError().finish()
    });
    info!("Succes in pinging database");
    // HttpResponse::Ok().body(format!("Welcome {}!", "test"))
    HttpResponse::Ok().json(&pong.unwrap())

}


#[post("/create_user")]
pub async fn create_user(pool: web::Data<DbPool>,new_user: web::Json<User>) ->  impl Responder {
        let user = new_user.into_inner();
    
        let result = create_userdata(&pool, user).await.map_err(|_e| {
            error!("Error in pinging database");
            HttpResponse::InternalServerError().finish()
        });
        info!("Succes in pinging database");
        HttpResponse::Ok().body(format!("Welcome {}!", "test"))
    }
    

#[delete("/delete_user/{user_id}")]
pub async fn delete_user(pool: web::Data<DbPool>,web::Path(user_id): web::Path<i64>) ->  impl Responder {    
        let result = delete_userdata(&pool, user_id).await.map_err(|_e| {
            error!("Error in pinging database");
            HttpResponse::InternalServerError().finish()
        });
        info!("Succes in pinging database");
        HttpResponse::Ok().body(format!("Welcome {}!", "test"))
    }



#[post("/apitest")]
pub async fn jsondemo(post: web::Json<UserInfo>) -> impl Responder {
    info!("Succes in pinging database");
    HttpResponse::Ok().body(format!("Welcome {}!", post.username))
}