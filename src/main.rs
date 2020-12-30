use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::sync::Arc;

mod account;
use account::adapter::{
    persistence::account_persistence_adapter::AccountPersistenceAdapter,
    web::account_controller::{account_routes, AccountController},
    web::send_money_controller::{send_money_routes, SendMoneyController},
};
use account::application::service::get_account_balance_service::GetAccountBalanceService;
use account::application::service::send_money_service::SendMoneyService;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Clone)]
pub struct State {
    account_controller: AccountController,
    send_money_controller: SendMoneyController,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().unwrap();
    let account_adapter = Arc::new(AccountPersistenceAdapter::default());

    let get_account_balance_service = Arc::new(GetAccountBalanceService {
        load_account_port: account_adapter.clone(),
    });

    let send_money_service = Arc::new(SendMoneyService {
        load_account_port: account_adapter.clone(),
        update_account_state_port: account_adapter.clone(),
    });
    let send_money_controller = SendMoneyController {
        send_money_use_case: send_money_service.clone(),
    };

    let account_controller = AccountController {
        get_account_balance_query: get_account_balance_service,
    };

    let state = State {
        account_controller,
        send_money_controller,
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(greet))
            .service(account_routes::get_account_balance)
            .service(send_money_routes::send_money)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
