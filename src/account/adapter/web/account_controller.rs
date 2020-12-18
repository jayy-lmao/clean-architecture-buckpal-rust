use actix_web::{get,post, App, HttpRequest, HttpServer, Result};
use serde::Deserialize;

use crate::account::application::port::incoming::SendMoneyUseCase;

macro_rules! pathVariable {
    ($input:expr) => {
        req.match_info().query($input).parse()?
    }
}


#[derive(Deserialize)]
struct AccountRequest {}

struct AccountController {
    getAccountBalanceQuery: GetAccountBalanceQuery,
    listAccountsQuery: ListAccountsQuery,
    loadAccountsQuery: loadAccountsQuery,

    sendMoneyUseCase: SendMoneyUseCase,
    createAccountUseCase: CreateAccountUseCase,
}

impl AccountController {
    #[get("/accounts")]
    pub fn listAccounts() -> Result<Vec<AccountResource>> {
        unimplemented!();
    }


    #[get("/accounts/{id}")]
    pub fn getAccount() -> Result<AccountResource> {
        let getAccount: AccountId = pathVariable!("id");
        unimplemented!();
    }

    #[get("/accounts/{id}/balance")]
    pub fn getAccountBalance(req: HttpRequest) -> Result<usize> {
        let accountId: AccountId = pathVariable!("id");
        unimplemented!();
    }

    #[post("/accounts")]
    pub fn createAccount(accountReq: web::Json<AccountRequest>) -> Result<AccountResource> {
        unimplemented!();
    }


}
