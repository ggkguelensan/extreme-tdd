#[derive(Debug, PartialEq)]
struct Money {
    amount: i32,
    currency: String,
}

impl Money {
    fn new(amount: i32, currency: String) -> Money {
        Money { amount, currency }
    }

    fn multiply(&self, multiplier: i32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency.clone(),
        }
    }

    fn dollar(amount: i32) -> Money {
        Money {
            amount,
            currency: String::from("USD"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_by_positive_number() {
        let price = Money {
            amount: 5,
            currency: String::from("USD"),
        };

        let result = price.multiply(3);

        assert_eq!(
            result,
            Money {
                amount: 15,
                currency: String::from("USD"),
            }
        );
    }

    #[test]
    fn test_multiply_by_zero() {
        let price = Money {
            amount: 5,
            currency: String::from("USD"),
        };

        let result = price.multiply(0);

        assert_eq!(
            result,
            Money {
                amount: 0,
                currency: String::from("USD"),
            }
        );
    }

    #[test]
    fn test_multiply_by_negative() {
        let price = Money {
            amount: 5,
            currency: String::from("USD"),
        };

        let result = price.multiply(-3);

        assert_eq!(
            result,
            Money {
                amount: -15,
                currency: String::from("USD"),
            }
        );
    }

    #[test]
    fn test_multiply_negative_amount() {
        let price = Money {
            amount: -5,
            currency: String::from("USD"),
        };

        let result = price.multiply(3);

        assert_eq!(
            result,
            Money {
                amount: -15,
                currency: String::from("USD"),
            }
        );
    }

    #[test]
    fn test_multiplication_identity() {
        let price = Money {
            amount: 5,
            currency: String::from("USD"),
        };

        let result = price.multiply(1);

        assert_eq!(result, price);
    }

    #[test]
    fn test_dollar_multiplication() {
        let five = Money::dollar(5);

        let product = five.multiply(2);
        assert_eq!(product, Money::dollar(10));

        let product2 = five.multiply(3);
        assert_eq!(product2, Money::dollar(15));
    }

    #[test]
    fn test_dollar_currency() {
        let dollar = Money::dollar(5);
        assert_eq!(dollar.currency, "USD");
    }

    #[test]
    fn test_money_creation() {
        let money = Money::new(10, String::from("EUR"));
        assert_eq!(money.amount, 10);
        assert_eq!(money.currency, "EUR");
    }

    #[test]
    fn test_dollar_factory() {
        let money = Money::dollar(10);
        assert_eq!(money.amount, 10);
        assert_eq!(money.currency, "USD");
    }

    #[test]
    fn test_different_currency_equality() {
        let dollars = Money::dollar(10);
        let euros = Money::new(10, String::from("EUR"));
        assert_ne!(dollars, euros);
    }

    #[test]
    fn test_same_currency_equality() {
        let dollars1 = Money::dollar(10);
        let dollars2 = Money::dollar(10);
        assert_eq!(dollars1, dollars2);
    }

    #[test]
    fn test_zero_amount() {
        let money = Money::new(0, String::from("USD"));
        assert_eq!(money.amount, 0);
    }

    #[test]
    fn test_negative_amount() {
        let money = Money::new(-10, String::from("USD"));
        assert_eq!(money.amount, -10);
    }
}
