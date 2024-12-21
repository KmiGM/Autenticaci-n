#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_db_pools;

use rocket::serde::{json::Json, Deserialize, Serialize};
use crate::schema::users;
use crate::models::User ;
use rocket::fs::{FileServer, relative};
use rocket_db_pools::{Database, Connection};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::Queryable;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;
use argon2; 
use argon2::{Argon2, Config}; // Cambia esta línea
use std::env;
mod models;

#[database("sqlite_db")]
struct Db(SqliteConnection);

#[derive(Serialize, Deserialize)]
struct RegisterUser   {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginUser   {
    username: String,
    password: String,
}

#[post("/register", format = "json", data = "<user>")]
async fn register_user(user: Json<RegisterUser >, db: &Connection<Db>) -> Status {    
    use models::User  ;
    use diesel::insert_into;

    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_encoded(user.password.as_bytes(), b"somesalt", &Config::default()).unwrap();
    
    let new_user = User {
        id: 0, // Se autoincrementará
        username: user.username.clone(),
        password: hashed_password,
        role_id: None, // Asignar un rol si es necesario
    };

    let result = insert_into(users::table)
        .values(&new_user)
        .execute(&*db)
        .await;

    match result {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[post("/login", format = "json", data = "<user>")]
async fn login_user(user: Json<LoginUser >, db: &Connection<Db>, cookies: &CookieJar<'_>) -> Status {

    let user_record = users::table
        .filter(users::username.eq(&user.username))
        .first::<User  >(&*db)
        .await;

    match user_record {
        Ok(record) => {
            let argon2 = Argon2::default();
            if argon2.verify_encoded(&record.password, user.password.as_bytes()).is_ok () {
                // Establecer una cookie de sesión
                cookies.add(Cookie::new("user_id", record.id.to_string()));
                Status::Ok;
            }else {
                Status::Unauthorized
            }
        },
        Err(_) => Status::NotFound,
    }
}

/*
fn hash_password(password: &str) -> String {
    let argon2 = argon2::Argon2::default(); // Crear una instancia de Argon2
    argon2.hash_encoded(password.as_bytes(), b"somesalt", &argon2::Config::default()).unwrap()
}

fn verify_password(password: &str, hashed: &str) -> bool {
    argon2::verify_encoded(hashed, password.as_bytes()).unwrap_or(false)
}
*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![register_user, login_user])
        .mount("/files", FileServer::from(relative!("static")))
}