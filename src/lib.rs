#[derive(Debug, PartialEq)]
struct Money {
    amount: i32,
    currency: String,
}

impl Money {
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
}
