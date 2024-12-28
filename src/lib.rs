#[derive(Debug)]
pub struct Money {
    amount: i32,
    currency: String,
}

pub struct Dollar {
    amount: i32,
    currency: String,
}

impl Dollar {
    fn new(amount: i32) -> Dollar {
        Dollar {
            amount,
            currency: String::from("USD"),
        }
    }
}

trait MoneyLike {
    fn amount(&self) -> i32;
    fn currency(&self) -> &str;
    fn multiply(&self, multiplier: i32) -> Money {
        Money {
            amount: self.amount() * multiplier,
            currency: self.currency().to_string(),
        }
    }
}

impl MoneyLike for Money {
    fn amount(&self) -> i32 {
        self.amount
    }
    fn currency(&self) -> &str {
        &self.currency
    }
}

impl MoneyLike for Dollar {
    fn amount(&self) -> i32 {
        self.amount
    }
    fn currency(&self) -> &str {
        &self.currency
    }
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

    fn eq(a: &impl MoneyLike, b: &impl MoneyLike) -> bool {
        if a.amount() == 0 && b.amount() == 0 {
            return true;
        }
        a.amount() == b.amount() && a.currency() == b.currency()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_multiplication() {
        let five = Money::new(5, String::from("USD"));
        assert!(Money::eq(
            &Money::new(5, String::from("USD")),
            &five.multiply(1)
        ));
        assert!(Money::eq(
            &Money::new(10, String::from("USD")),
            &five.multiply(2)
        ));
        assert!(Money::eq(
            &Money::new(-5, String::from("USD")),
            &five.multiply(-1)
        ));
        assert!(Money::eq(
            &Money::new(0, String::from("USD")),
            &five.multiply(0)
        ));
    }

    #[test]
    fn test_dollar_multiplication() {
        let five = Dollar::new(5);
        assert!(Money::eq(&Dollar::new(5), &five.multiply(1)));
        assert!(Money::eq(&Dollar::new(10), &five.multiply(2)));
        assert!(Money::eq(&Dollar::new(-5), &five.multiply(-1)));
        assert!(Money::eq(&Dollar::new(0), &five.multiply(0)));
    }

    #[test]
    fn test_multiplication_identity() {
        let price = Money::new(5, String::from("USD"));
        let result = price.multiply(1);
        assert!(Money::eq(&result, &price));
    }

    #[test]
    fn test_dollar_currency() {
        let dollar = Dollar::new(5);
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
        let money = Dollar::new(10);
        assert_eq!(money.amount, 10);
        assert_eq!(money.currency, "USD");
    }

    #[test]
    fn test_different_currency_equality() {
        let dollars = Dollar::new(10);
        let euros = Money::new(10, String::from("EUR"));
        assert!(!Money::eq(&dollars, &euros));
    }

    #[test]
    fn test_same_currency_equality() {
        let dollars1 = Dollar::new(10);
        let dollars2 = Dollar::new(10);
        assert!(Money::eq(&dollars1, &dollars2));
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

    #[test]
    fn test_money_immutability_after_multiplication() {
        let original = Dollar::new(5);
        let result = original.multiply(2);
        assert!(Money::eq(&original, &Dollar::new(5)));
        assert!(Money::eq(&result, &Dollar::new(10)));
    }

    #[test]
    fn test_money_immutability_with_multiple_operations() {
        let original = Dollar::new(10);
        let _ = original.multiply(2);
        let _ = original.multiply(3);
        let _ = original.multiply(4);
        assert!(Money::eq(&original, &Dollar::new(10)));
    }

    #[test]
    fn test_money_currency_immutability() {
        let original = Dollar::new(10);
        let currency_before = original.currency.clone();
        let _ = original.multiply(2);
        assert_eq!(original.currency, currency_before);
    }

    #[test]
    fn test_money_equality() {
        let money1 = Money::new(5, String::from("USD"));
        let money2 = Money::new(5, String::from("USD"));
        let money3 = Money::new(10, String::from("USD"));
        let euros = Money::new(5, String::from("EUR"));

        assert!(Money::eq(&money1, &money2));
        assert!(!Money::eq(&money1, &money3));
        assert!(!Money::eq(&money1, &euros));
    }

    #[test]
    fn test_money_equality_same_object() {
        let money = Dollar::new(5);

        assert!(Money::eq(&money, &money));
    }

    #[test]
    fn test_money_equality_zero_amount() {
        let money1 = Dollar::new(0);
        let money2 = Dollar::new(0);

        assert!(Money::eq(&money1, &money2));
    }

    #[test]
    fn test_money_equality_different_currencies() {
        let dollars = Dollar::new(5);
        let euros = Money::new(5, String::from("EUR"));

        assert!(!Money::eq(&dollars, &euros));
    }

    #[test]
    fn test_dollar_equality() {
        let dollar1 = Dollar::new(5);
        let dollar2 = Dollar::new(5);
        let dollar3 = Dollar::new(10);

        assert!(Money::eq(&dollar1, &dollar2));
        assert!(!Money::eq(&dollar1, &dollar3));
    }

    #[test]
    fn test_dollar_money_equality() {
        let dollar = Dollar::new(5);
        let money = Money::new(5, String::from("USD"));
        let euros = Money::new(5, String::from("EUR"));

        assert!(Money::eq(&dollar, &money));
        assert!(Money::eq(&money, &dollar));
        assert!(!Money::eq(&dollar, &euros));
    }

    #[test]
    fn test_zero_amounts() {
        let dollar = Dollar::new(0);
        let euros = Money::new(0, String::from("EUR"));
        let money = Money::new(0, String::from("USD"));

        assert!(Money::eq(&dollar, &money));
        assert!(Money::eq(&dollar, &euros));
        assert!(Money::eq(&money, &euros));
    }

    #[test]
    fn test_multiplication() {
        let dollar = Dollar::new(5);
        let money = Money::new(5, String::from("USD"));

        assert_eq!(dollar.amount, 5);
        assert_eq!(money.amount, 5);
        assert_eq!(dollar.currency, "USD");
    }
}
