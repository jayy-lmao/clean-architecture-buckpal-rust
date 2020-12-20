use crate::account::application::port::incoming::GetAccountBalanceQuery;
use crate::account::domain::*;

use actix_web::{web, HttpRequest, Result};
use serde::{Deserialize, Serialize};

use std::sync::Arc;


#[derive(Deserialize)]
struct AccountRequest {}

#[derive(Serialize)]
struct AccountResource {}

struct AccountController {
getAccountBalanceQuery: Arc<dyn GetAccountBalanceQuery + Sync + Send>,
// listAccountsQuery: Arc<dyn ListAccountsQuery + Sync + Send>,
// loadAccountsQuery: Arc<dyn LoadAccountsQuery + Sync + Send>,
// createAccountUseCase: CreateAccountUseCase,
}

impl AccountController {
    // #[get("/accounts")]
    pub fn listAccounts() -> Result<Vec<AccountResource>> {
        unimplemented!();
    }

    // #[get("/accounts/{id}")]
    pub fn getAccount(req: HttpRequest) -> Result<AccountResource> {
        let id: usize = req.match_info().query("id").parse().unwrap();
        unimplemented!();
    }

    // #[get("/accounts/{id}/balance")]
    pub fn getAccountBalance(req: HttpRequest) -> Result<usize> {
        let id: usize = req.match_info().query("id").parse().unwrap();
        unimplemented!();
    }

    // #[post("/accounts")]
    pub fn createAccount(accountReq: web::Json<AccountRequest>) -> Result<AccountResource> {
        unimplemented!();
    }
}
