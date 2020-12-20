use super::SendMoneyCommand;

pub trait SendMoneyUseCase {
    fn sendMoney(&self, command: SendMoneyCommand) -> bool;
}
