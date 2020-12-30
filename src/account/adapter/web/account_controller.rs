use crate::account::application::port::incoming::GetAccountBalanceQuery;
use crate::account::domain::*;
use std::sync::Arc;
// use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
#[derive(Clone)]
pub struct AccountController {
    pub get_account_balance_query: Arc<dyn GetAccountBalanceQuery + Sync + Send>,
    // listAccountsQuery: Arc<dyn ListAccountsQuery + Sync + Send>,
    // loadAccountPort: Arc<dyn LoadAccountPort + Sync + Send>,
    // createAccountUseCase: CreateAccountUseCase,
}
impl AccountController {
    pub async fn get_account_balance(&self, id: i64) -> Option<f32> {
        let account_balance = self
            .get_account_balance_query
            .get_account_balance(AccountId::new(id))
            .await;
        match account_balance {
            Ok(balance) => Some(balance.to_f32()),
            Err(err) => {
                dbg!(err);
                None
            }
        }
    }
}

pub mod account_routes {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    struct AccountRequest {}

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct AccountResource {
        pub account_id: usize,
    }

    /*
    #[get("/accounts")]
    pub fn listAccounts() -> Result<Vec<AccountResource>> {
        unimplemented!();
    }
    */

    // #[get("/accounts/{id}")]
    // pub async fn getAccount(req: HttpRequest, data: web::Data<crate::State>) -> impl Responder {
    //     let id: i64 = req.match_info().query("id").parse().unwrap();
    //     let account = data.account_controller.loadAccount(id);

    //     HttpResponse::Ok().json(account)
    // }

    #[get("/accounts/{id}/balance")]
    async fn get_account_balance(
        web::Path(id): web::Path<i64>,
        data: web::Data<crate::State>,
    ) -> impl Responder {
        let balance = data.account_controller.get_account_balance(id).await;
        HttpResponse::Ok().json(balance)
    }
    /*

    #[post("/accounts")]
    pub fn createAccount(accountReq: web::Json<AccountRequest>) -> Result<AccountResource> {
        unimplemented!();
    }
    */
}
