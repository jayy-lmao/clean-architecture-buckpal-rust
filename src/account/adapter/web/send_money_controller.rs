use actix_web::{get, App, HttpRequest, HttpServer, Result};
use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::domain::*;


macro_rules! pathVariable {
    ($input:expr) => {
        req.match_info().query($input).parse()?
    };
}

struct SendMoneyController {
    sendMoneyUseCase: SendMoneyUseCase,
}
impl SendMoneyController {
    #[post("/accounts/send/{sourceAccountId}/{targetAccountId}/{amount}")]
    pub fn createAccount(accountReq: web::Json<AccountRequest>) -> Result<AccountResource> {
        let sourceAccountId: AccountId = pathVariable!("sourceAccountId");
        let targetAccountId: AccountId = pathVariable!("targetAccountId");
        let amount: i64 = pathVariable!("targetAccountId");

        let command = SendMoneyCommand::new(
            new AccountId(sourceAccountId),
            new AccountId(targetAccountId),
            Money(Amount),
        );

        unimplemented!();
    }
}
