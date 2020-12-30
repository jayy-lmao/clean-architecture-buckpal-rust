use crate::account::application::port::incoming::SendMoneyUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct SendMoneyController {
    pub send_money_use_case: Arc<dyn SendMoneyUseCase + Sync + Send>,
}
impl SendMoneyController {}

pub mod send_money_routes {
    use crate::account::application::port::incoming::SendMoneyCommand;
    use crate::account::domain::*;
    use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

    #[post("/accounts/send/{sourceAccountId}/{targetAccountId}/{amount}")]
    pub async fn send_money(req: HttpRequest, data: web::Data<crate::State>) -> impl Responder {
        let source_account_id: i64 = req.match_info().query("sourceAccountId").parse().unwrap();
        let target_account_id: i64 = req.match_info().query("targetAccountId").parse().unwrap();
        let amount: f32 = req.match_info().query("amount").parse().unwrap();

        let command = SendMoneyCommand::new(
            AccountId::new(source_account_id),
            AccountId::new(target_account_id),
            Money::new(amount),
        );

        match data
            .send_money_controller
            .send_money_use_case
            .send_money(command)
            .await
        {
            Ok(_) => HttpResponse::Ok(),
            Err(_) => HttpResponse::InternalServerError(),
        }
    }
}
