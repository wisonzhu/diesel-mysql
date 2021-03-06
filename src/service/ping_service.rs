use actix_web::{web};
use actix_web::error::BlockingError;
use diesel::RunQueryDsl;
use diesel::sql_types::Text;
use diesel::result::Error;
use crate::config::database::DbPool;

#[derive(QueryableByName, PartialEq, Debug)]
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

// pub async fn insertuser(pool: &web::Data<DbPool>) -> Result<QueryResult, BlockingError<Error>>{
//     let conn = pool.get().unwrap();
//     let insertuser = web::block(move || diesel::insert_into("select name as data from users")
//         .get_result::<QueryResult>(&conn))
//         .await;
//     return insertuser
// }