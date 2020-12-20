// use crate::account::application::port::incoming::GetAccountBalanceQuery;
#[derive(Debug, Clone, Copy)]
pub struct AccountController {
    // getAccountBalanceQuery: Arc<dyn GetAccountBalanceQuery + Sync + Send>,
// listAccountsQuery: Arc<dyn ListAccountsQuery + Sync + Send>,
// loadAccountsQuery: Arc<dyn LoadAccountsQuery + Sync + Send>,
// createAccountUseCase: CreateAccountUseCase,
}

pub mod account_routes {
    use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
    use serde::{Deserialize, Serialize};

    #[derive(Clone,Copy,Debug,Deserialize,Serialize)]
    struct AccountRequest {}

    #[derive(Clone,Copy,Debug,Serialize,Deserialize)]
    pub struct AccountResource {
        pub accountId: usize,
    }

    /*
    #[get("/accounts")]
    pub fn listAccounts() -> Result<Vec<AccountResource>> {
        unimplemented!();
    }
    */

    #[get("/accounts/{id}")]
    pub async fn getAccount(req: HttpRequest) -> impl Responder {
        // Result<web::Json<AccountResource>> {
        let id: usize = req.match_info().query("id").parse().unwrap();
        HttpResponse::Ok().json(AccountResource { accountId: id })
    }

    /*
    #[get("/accounts/{id}/balance")]
    pub fn getAccountBalance(req: HttpRequest) -> Result<usize> {
        let id: usize = req.match_info().query("id").parse().unwrap();
        unimplemented!();
    }

    #[post("/accounts")]
    pub fn createAccount(accountReq: web::Json<AccountRequest>) -> Result<AccountResource> {
        unimplemented!();
    }
    */
}
