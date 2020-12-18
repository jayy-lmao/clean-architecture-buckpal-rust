use crate::account::application::port::incoming::SendMoneyCommand;
use crate::application::port::incoming::SendMoneyUseCase;
use crate::account::domain::*;
use actix_web::{get, App, HttpRequest, HttpServer, Result};

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
            AccountId(sourceAccountId),
            AccountId(targetAccountId),
            Money(Amount),
        );

        sendMoneyUseCase::sendMoney(command);
    }
}
