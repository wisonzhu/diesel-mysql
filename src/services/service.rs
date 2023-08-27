use actix_web::{web};
use actix_web::error::BlockingError;
use diesel::RunQueryDsl;
use diesel::sql_types::Text;
use diesel::result::Error;
use crate::config::database::DbPool;
use serde::Serialize;
use crate::models::user::User;
use crate::models::user::users::table;
use crate::models::user::users::dsl::*;
use diesel::prelude::*;
use crate::routes::route::UpdatedUserInfo;

#[derive(QueryableByName, PartialEq, Debug,Serialize)]
#[table_name = "users"]
pub struct QueryResult {
    #[sql_type="Text"]
    // pub col1: i64,
    pub data: String,
}

pub async fn ping(pool: &web::Data<DbPool>) -> Result<QueryResult, BlockingError<Error>>{
    let conn = pool.get().unwrap();
    let ping = web::block(move || diesel::sql_query("SELECT RIGHT('foobarbar', 4) as data")
        .get_result::<QueryResult>(&conn))
        .await;
        return ping
}

pub async fn getuser(pool: &web::Data<DbPool>) -> Result<QueryResult, BlockingError<Error>>{
    let conn = pool.get().unwrap();
    let getuser = web::block(move || diesel::sql_query("select name as data from users")
        .get_result::<QueryResult>(&conn))
        .await;
    return getuser
}



pub async fn create_userdata(pool: &DbPool, new_user: User) -> Result<(), Error> {
    let conn = pool.get().unwrap();
    diesel::insert_into(table)
        .values(&new_user)
        .execute(&conn)?;
    Ok(())
}



pub async fn delete_userdata(pool: &DbPool, user_id: i64) -> Result<(), Error> {
    let conn = pool.get().unwrap();
        diesel::delete(table.filter(id.eq(user_id)))
            .execute(&conn)?;
        Ok(())
}


pub async fn update_userdata(pool: &DbPool, user_id: i64, updated_info: &UpdatedUserInfo) -> Result<(), Error> {
    let conn = pool.get().unwrap();

    let updated_rows = diesel::update(table.filter(id.eq(user_id)))
        .set(name.eq(&updated_info.name))
        .execute(&conn)?;

    if updated_rows == 0 {
        return Err(Error::NotFound);
    }

    Ok(())
}


// pub async fn listuser(pool: &web::Data<DbPool>) -> Result<Vec<QueryResult>, BlockingError<Error>>{
//     let conn = pool.get().unwrap();
//     let getuser1 = web::block(move || diesel::sql_query("select name as data from users")
//         .get_results::<QueryResult>(&conn))
//         .await;
//     for i in getuser1.iter() {
//         println!("test {:?}", i)
//     }
//     return  getuser1
// }



pub async fn listuser(pool: &web::Data<DbPool>) -> Result<Vec<User>, Error> {
    let conn = pool.get().unwrap();
    // let getuser1 = web::block(move || diesel::sql_query("select name as data from users")
    //     .get_results::<User>(&conn))
    //     .await;

    let results = table.load::<User>(&conn)?;

    for i in results.iter() {
        println!("test {:?}", i)
    }

    Ok(results)
}