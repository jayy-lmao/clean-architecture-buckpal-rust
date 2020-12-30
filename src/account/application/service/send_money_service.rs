use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::application::port::incoming::SendMoneyUseCase;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::application::port::outgoing::update_account_state_port::UpdateAccountStatePort;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct SendMoneyService {
    pub load_account_port: Arc<dyn LoadAccountPort + Sync + Send>,
    pub update_account_state_port: Arc<dyn UpdateAccountStatePort + Sync + Send>,
}

#[async_trait]
impl SendMoneyUseCase for SendMoneyService {
    async fn send_money(&self, command: SendMoneyCommand) -> anyhow::Result<bool> {
        // requireAccountExists(command.getSourceAccountId());
        // requireAccountExists(command.getTargetAccountId());
        // TODO: validate business rules
        // TODO: manipulate model state
        let timestamp = Utc::now().naive_utc();
        let mut source_account = self
            .load_account_port
            .load_account(command.get_source_account_id(), timestamp)
            .await?;

        let mut target_account = self
            .load_account_port
            .load_account(command.get_target_account_id(), timestamp)
            .await?;

        let money = command.money();

        let withdraw_success = source_account.withdraw(money, command.get_target_account_id());
        let deposit_success = target_account.deposit(money, command.get_source_account_id());

        if withdraw_success && deposit_success {
            self.update_account_state_port
                .update_account_state(source_account, timestamp)
                .await?;

            self.update_account_state_port
                .update_account_state(target_account, timestamp)
                .await?;
        };

        Ok(withdraw_success)
    }
}
