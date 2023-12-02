#[derive(Debug)]
pub struct Account {
    amount: u32,
    total_withdrawal_limit: u32,
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    ExceedWithdrawalLimit,
    InsufficientBalance(u32),
}

impl Account {
    pub fn new(amount: u32, total_withdrawal_limit: u32) -> Self {
        Self {
            amount,
            total_withdrawal_limit
        }
    }

    pub fn withdraw_with_error_msg(&mut self, amount: u32) -> Result<u32, String> {
        if amount > self.total_withdrawal_limit {
            return Err(format!("withdraw limit exceeded total withdrawal limit"));
        }

        self.amount = self.amount.checked_sub(amount)
            .ok_or_else(|| format!("insufficient balance in account ({})", self.amount))?;

        Ok(self.amount)
    }

    pub fn withdraw(&mut self, amount: u32) -> Result<u32, AccountError> {
        if amount > self.total_withdrawal_limit {
            return Err(AccountError::ExceedWithdrawalLimit);
        }

        self.amount = self.amount.checked_sub(amount)
            .ok_or_else(|| AccountError::InsufficientBalance(self.amount))?;

        Ok(self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn withdraw_ok() {
        let mut account = Account::new(1000, 100);
        let actual = account.withdraw(50).unwrap();

        assert_eq!(actual, 950);
    }

    #[test]
    fn withdraw_with_error_msg_limit_error() {
        let mut account = Account::new(1000, 100);
        let actual = account.withdraw_with_error_msg(500);
        assert!(actual.is_err());

        let err = actual.unwrap_err();
        assert_eq!(err, format!("withdraw limit exceeded total withdrawal limit"));
    }

    #[test]
    fn withdraw_with_error_msg_insuffient_error() {
        let mut account = Account::new(10, 100);
        let actual = account.withdraw_with_error_msg(50);
        assert!(actual.is_err());

        let err = actual.unwrap_err();
        assert_eq!(err, format!("insufficient balance in account ({})", 10));
    }

    #[test]
    fn withdraw_withdrawal_limit_error() {
        let mut account = Account::new(1000, 100);
        let actual = account.withdraw(500);
        assert!(actual.is_err());

        let err = actual.unwrap_err();
        assert_eq!(err, AccountError::ExceedWithdrawalLimit);
    }

    #[test]
    fn withdraw_insuffient_error() {
        let mut account = Account::new(10, 100);
        let actual = account.withdraw(50);
        assert!(actual.is_err());

        let err = actual.unwrap_err();
        assert_eq!(err, AccountError::InsufficientBalance(10));
    }
}
