use super::SendMoneyCommand;

pub trait SendMoneyUseCase {
    fn sendMoney(command: SendMoneyCommand);
}
