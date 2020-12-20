use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::application::port::incoming::SendMoneyUseCase;
use crate::account::domain::*;
use actix_web::{HttpRequest, Result};
use std::sync::Arc;

struct SendMoneyController {
    sendMoneyUseCase: Arc<dyn SendMoneyUseCase + Sync + Send>,
}
impl SendMoneyController {
    // #[post("/accounts/send/{sourceAccountId}/{targetAccountId}/{amount}")]
    pub fn sendMoney(&self, req: HttpRequest) -> Result<()> {
        let sourceAccountId: usize = req
            .match_info()
            .query("sourceAccountId")
            .parse()
            .unwrap();
        let targetAccountId: usize = req
            .match_info()
            .query("targetAccountId")
            .parse()
            .unwrap();
        let amount: f64 = req
            .match_info()
            .query("targetAccountId")
            .parse()
            .unwrap();

        let command = SendMoneyCommand::new(
            AccountId::new(sourceAccountId),
            AccountId::new(targetAccountId),
            Money::new(amount),
        );

        self.sendMoneyUseCase.sendMoney(command);
        Ok(())
    }
}
