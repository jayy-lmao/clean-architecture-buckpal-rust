use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::sync::Arc;

mod account;
use account::adapter::{
    account_persistence_adapter::AccountPersistenceAdapter,
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
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv();
    let account_adapter = Arc::new(AccountPersistenceAdapter::default());

    let getAccountBalanceService = Arc::new(GetAccountBalanceService {
        loadAccountPort: account_adapter.clone(),
    });

    let sendMoneyService = Arc::new(SendMoneyService {
        loadAccountPort: account_adapter.clone(),
        updateAccountStatePort: account_adapter.clone(),
    });
    let send_money_controller = SendMoneyController {
        sendMoneyUseCase: sendMoneyService.clone(),
    };

    let account_controller = AccountController {
        getAccountBalanceQuery: getAccountBalanceService,
    };

    let state = State { account_controller };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(greet))
            .service(account_routes::getAccountBalance)
            .service(send_money_routes::sendMoney)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
