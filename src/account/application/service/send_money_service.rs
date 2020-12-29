use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::application::port::incoming::SendMoneyUseCase;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::application::port::outgoing::update_account_state_port::UpdateAccountStatePort;
use anyhow;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct SendMoneyService {
    pub loadAccountPort: Arc<dyn LoadAccountPort + Sync + Send>,
    pub updateAccountStatePort: Arc<dyn UpdateAccountStatePort + Sync + Send>,
}

#[async_trait]
impl SendMoneyUseCase for SendMoneyService {
    async fn sendMoney(&self, command: SendMoneyCommand) -> anyhow::Result<bool> {
        println!("Sending MOney");
        // requireAccountExists(command.getSourceAccountId());
        // requireAccountExists(command.getTargetAccountId());
        // TODO: validate business rules
        // TODO: manipulate model state
        let timestamp = Utc::now().naive_utc();
        dbg!(timestamp);
        println!("Timestampy");
        let mut sourceAccount = self
            .loadAccountPort
            .loadAccount(command.getSourceAccoundId(), timestamp)
            .await
            .unwrap();

        let mut targetAccount = self
            .loadAccountPort
            .loadAccount(command.getTargetAccountId(), timestamp)
            .await
            .unwrap();

        let money = dbg!(command.money());

        println!("money");

        let withdraw_success = sourceAccount.withdraw(money, command.getTargetAccountId());
        let deposit_success = targetAccount.deposit(money, command.getSourceAccoundId());

        if withdraw_success && deposit_success {



            let update_source = self.updateAccountStatePort
                .updateAccountState(dbg!(sourceAccount), timestamp)
                .await;

            let update_target = self.updateAccountStatePort
                .updateAccountState(dbg!(targetAccount), timestamp)
                .await;
            dbg!(update_source);
            dbg!(update_target);
        };

        Ok(withdraw_success)
    }
}
