use crate::account::application::port::incoming::SendMoneyUseCase;
use std::sync::Arc;

pub struct SendMoneyController {
    pub sendMoneyUseCase: Arc<dyn SendMoneyUseCase + Sync + Send>,
}
impl SendMoneyController {}

pub mod send_money_routes {
    use crate::account::application::port::incoming::SendMoneyCommand;
    use crate::account::domain::*;
    use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
    use anyhow::anyhow;
    use serde::{Deserialize, Serialize};

    #[post("/accounts/send/{sourceAccountId}/{targetAccountId}/{amount}")]
    pub async fn sendMoney(
        &self,
        req: HttpRequest,
        data: web::Data<crate::State>,
    ) -> impl Responder {
        let sourceAccountId: i64 = req.match_info().query("sourceAccountId").parse().unwrap();
        let targetAccountId: i64 = req.match_info().query("targetAccountId").parse().unwrap();
        let amount: f32 = req.match_info().query("targetAccountId").parse().unwrap();

        let command = SendMoneyCommand::new(
            AccountId::new(sourceAccountId),
            AccountId::new(targetAccountId),
            Money::new(amount),
        );

        data.sendMoneyUseCase.sendMoney(command);
        Ok(())
    }
}
