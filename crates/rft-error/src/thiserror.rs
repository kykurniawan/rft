#[cfg(test)]
mod test {
    use std::ops::Add;

    use chrono::{DateTime, Utc};
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum PaymentError {
        #[error("Insufficient funds")]
        InsufficientFunds,

        #[error("Invalid card number")]
        InvalidCardNumber,

        #[error("Invalid CVV")]
        InvalidCvv,

        #[error("Card expired")]
        CardExpired,
    }

    impl PaymentError {
        fn print(&self) {
            match self {
                PaymentError::InsufficientFunds => println!("Insufficient funds"),
                PaymentError::InvalidCardNumber => println!("Invalid card number"),
                PaymentError::InvalidCvv => println!("Invalid CVV"),
                PaymentError::CardExpired => println!("Card expired"),
            }
        }
    }

    fn make_payment(
        card_number: String,
        card_cvv: String,
        card_expiry: DateTime<Utc>,
        card_balance: u64,
        pay_amount: u64,
    ) -> Result<String, PaymentError> {
        if card_balance < pay_amount {
            return Err(PaymentError::InsufficientFunds);
        }

        if card_expiry < Utc::now() {
            return Err(PaymentError::CardExpired);
        }

        if card_cvv != "123" {
            return Err(PaymentError::InvalidCvv);
        }

        if card_number.len() != 16 {
            return Err(PaymentError::InvalidCardNumber);
        }

        Ok(String::from("Payment successful"))
    }

    #[test]
    fn test_error() {
        let payment_1 = make_payment(
            String::from("1234567890123456"),
            String::from("123"),
            Utc::now().add(chrono::Duration::days(365)),
            100,
            50,
        );

        let payment_2 = make_payment(
            String::from("1234567890123456"),
            String::from("123"),
            Utc::now(),
            100,
            100,
        );

        match payment_1 {
            Ok(_) => println!("Payment 1 successful"),
            Err(err) => {
                println!("Payment 1 failed");
                err.print();
            }
        }

        match payment_2 {
            Ok(_) => println!("Payment 2 successful"),
            Err(err) => {
                println!("Payment 2 failed");
                err.print();
            }
        }
    }
}
