use std::env;

use diesel::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use salvo::prelude::*;

use crate::handler::menu_handler::{*};
use crate::handler::role_handler::{*};
use crate::handler::user_handler::{*};
use crate::middleware::auth::auth_token;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod schema;
pub mod middleware;

#[handler]
async fn hello() -> &'static str {
    "Hello World123123"
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub static RB: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("database_url").expect("database_url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
});

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:8100").bind().await;
    Server::new(acceptor).serve(route()).await;
}

fn route() -> Router {
    Router::new().path("/api").get(hello)
        .push(Router::new().path("login").post(login))
        .push(
            Router::new().hoop(auth_token)
                .push(Router::new().path("query_user_role").post(query_user_role))
                .push(Router::new().path("update_user_role").post(update_user_role))
                .push(Router::new().path("query_user_menu").get(query_user_menu))
                .push(Router::new().path("user_list").post(user_list))
                .push(Router::new().path("user_save").post(user_save))
                .push(Router::new().path("user_update").post(user_update))
                .push(Router::new().path("user_delete").post(user_delete))
                .push(Router::new().path("update_user_password").post(update_user_password))
                .push(Router::new().path("role_list").post(query_role_list))
                .push(Router::new().path("role_save").post(add_role))
                .push(Router::new().path("role_update").post(update_role))
                .push(Router::new().path("role_delete").post(delete_role))
                .push(Router::new().path("query_role_menu").post(query_role_menu))
                .push(Router::new().path("update_role_menu").post(update_role_menu))
                .push(Router::new().path("menu_list").post(menu_list))
                .push(Router::new().path("menu_save").post(menu_save))
                .push(Router::new().path("menu_update").post(menu_update))
                .push(Router::new().path("menu_delete").post(menu_delete))
        )
}