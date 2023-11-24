use super::how_to_create_error::{AccountError, Account};

pub fn consume_using_unwrap() {
    let mut account = Account::new(1000, 100);

    let amount = account.withdraw(500).unwrap();
    println!("amount = {}", amount);
}

pub fn consume_using_match() {
    let mut account = Account::new(1000, 100);

    match account.withdraw(500) {
        Ok(amount) => amount,
        Err(err) => match err {
            AccountError::ExceedWithdrawalLimit => todo!(),
            AccountError::InsufficientBalance(_) => todo!(),
        },
    };

    match account.withdraw_with_error_msg(500) {
        Ok(amount) => amount,
        Err(err) => todo!(),
    };
}

pub fn consume_using_if_let_without_knowing_error_type() {
    let mut account = Account::new(1000, 100);

    if let Ok(amount) = account.withdraw(500) {
        println!("amount = {}", amount);
    } else {
        println!("fail to withdraw")
    }
}

pub fn consume_using_if_let_with_error_type() {
    let mut account = Account::new(1000, 100);
    let withdrawal_result = account.withdraw(500);

    if let Ok(amount) = withdrawal_result {
        println!("amount = {}", amount);
    } else {
        println!("fail to withdraw - {:?}", withdrawal_result.err().unwrap());

        // to know the error detail can use match like above
    }
}

pub fn propragate_error_to_caller() -> Result<u32, AccountError> {
    let mut account = Account::new(1000, 100);
    let amount = account.withdraw(500)?;

    Ok(amount)
}
