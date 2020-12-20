use crate::account::application::port::incoming::SendMoneyCommand;
use crate::account::application::port::incoming::SendMoneyUseCase;

pub struct SendMoneyService {
    // loadAccountPort: LoadAccountPort,
    // accountLock: AccountLockPort,
    // updateAccountStatePort: UpdateAccountStatePort,
}

impl SendMoneyUseCase for SendMoneyService {
    fn sendMoney(&self, command: SendMoneyCommand)-> bool {
        // requireAccountExists(command.getSourceAccountId());
        // requireAccountExists(command.getTargetAccountId());
        // TODO: validate business rules
        // TODO: manipulate model state
        // TODO: return output
        unimplemented!();
    }
}
