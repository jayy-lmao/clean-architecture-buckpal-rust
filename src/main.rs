use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod account;
use account::adapter::web::account_controller::{AccountController,account_routes};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Debug,Clone,Copy)]
struct State {
    account_controller: AccountController,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let account_controller = AccountController {};
    let state = State {
        account_controller
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(greet))
            .service(account_routes::getAccount)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
