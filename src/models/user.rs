use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable,Deserialize,Serialize,Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
    }
}