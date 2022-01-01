use super::db::DbConnection;
use super::model::{NewUser, User};
use crate::model::UserData;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/users", format = "application/json")]
pub fn get_all(conn: DbConnection) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users
    }))
}

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConnection, new_user: Json<NewUser>) -> Json<Value> {
    let is_successfully_added = User::add_user(new_user.into_inner(), &conn);
    let added_user = User::get_all_users(&conn);
    Json(json!({ "status": is_successfully_added, "result": added_user.first() }))
}

#[post("/users/get", format = "application/json", data = "<user_data>")]
pub fn get_user(conn: DbConnection, user_data: Json<UserData>) -> Json<Value> {
    let user = User::get_user_by_username(user_data.into_inner(), &conn);
    Json(json!({
        "status": 200,
        "result": user
    }))
}
