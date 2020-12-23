use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::application::port::incoming::SendMoneyUseCase;
use crate::account::application::port::outgoing::load_account_port::LoadAccountPort;
use crate::account::application::port::outgoing::update_account_state_port::UpdateAccountStatePort;
use std::sync::Arc;
use chrono::Utc;
use async_trait::async_trait;
use anyhow;

pub struct SendMoneyService {
    loadAccountPort: Arc<dyn LoadAccountPort + Sync + Send>,
    // accountLock: AccountLockPort,
    updateAccountStatePort: Arc<dyn UpdateAccountStatePort + Sync + Send>,
}

#[async_trait]
impl SendMoneyUseCase for SendMoneyService {
    async fn sendMoney(&self, command: SendMoneyCommand) -> anyhow::Result<bool> {
        // requireAccountExists(command.getSourceAccountId());
        // requireAccountExists(command.getTargetAccountId());
        // TODO: validate business rules
        // TODO: manipulate model state
        let timestamp = Utc::now().naive_utc();
        let mut sourceAccount = self.loadAccountPort.loadAccount(command.getSourceAccoundId(), timestamp).await?;

        let money = command.money();

        let success = sourceAccount.withdraw(money, command.getTargetAccountId());
        if success {
            self.updateAccountStatePort.updateAccountState(sourceAccount, timestamp);
        };

        Ok(success)
    }
}
