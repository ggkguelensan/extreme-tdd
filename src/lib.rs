trait Money {
    fn get_amount(&self) -> i32;
    fn get_currency(&self) -> &str;

    fn new(amount: i32, currency: String) -> Self
    where
        Self: Sized;

    fn multiply(&self, by: i32) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_amount() * by, self.get_currency().to_string())
    }

    fn eq<T: Money>(&self, other: &T) -> bool {
        if self.get_amount() == 0 && other.get_amount() == 0 {
            return true;
        }
        self.get_amount() == other.get_amount() && self.get_currency() == other.get_currency()
    }
}

struct Dollar {
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

impl Money for Dollar {
    fn get_amount(&self) -> i32 {
        self.amount
    }
    fn get_currency(&self) -> &str {
        &self.currency
    }
    fn new(amount: i32, currency: String) -> Self {
        Dollar { amount, currency }
    }
}

struct Franc {
    amount: i32,
    currency: String,
}

impl Franc {
    fn new(amount: i32) -> Self {
        Franc {
            amount,
            currency: String::from("CHF"),
        }
    }
}

impl Money for Franc {
    fn get_amount(&self) -> i32 {
        self.amount
    }
    fn get_currency(&self) -> &str {
        &self.currency
    }
    fn new(amount: i32, currency: String) -> Self {
        Franc { amount, currency }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_multiplication() {
        let five = Dollar::new(5);
        assert!(Money::eq(&Dollar::new(5), &five.multiply(1)));
        assert!(Money::eq(&Dollar::new(10), &five.multiply(2)));
        assert!(Money::eq(&Dollar::new(-5), &five.multiply(-1)));
        assert!(Money::eq(&Dollar::new(0), &five.multiply(0)));
    }

    #[test]
    fn test_dollar_multiplication_chain() {
        let five = Dollar::new(5);
        let result = five.multiply(2).multiply(3);
        assert_eq!(result.get_amount(), 30);
        assert_eq!(result.get_currency(), "USD");
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert!(Money::eq(&Franc::new(5), &five.multiply(1)));
        assert!(Money::eq(&Franc::new(10), &five.multiply(2)));
        assert!(Money::eq(&Franc::new(-5), &five.multiply(-1)));
        assert!(Money::eq(&Franc::new(0), &five.multiply(0)));
    }

    #[test]
    fn test_dollar_currency() {
        let dollar = Dollar::new(5);
        assert_eq!(dollar.currency, "USD");
    }

    #[test]
    fn test_dollar_factory() {
        let money = Dollar::new(10);
        assert_eq!(money.amount, 10);
        assert_eq!(money.currency, "USD");
    }

    #[test]
    fn test_franc_currency() {
        let frank = Franc::new(5);
        assert_eq!(frank.currency, "CHF");
    }

    #[test]
    fn test_different_currency_equality() {
        let dollars = Dollar::new(10);
        let frank = Franc::new(10);
        assert!(!Money::eq(&dollars, &frank));
    }

    #[test]
    fn test_same_currency_equality() {
        let dollars1 = Dollar::new(10);
        let dollars2 = Dollar::new(10);
        assert!(Money::eq(&dollars1, &dollars2));
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
        let money1 = Dollar::new(5);
        let money2 = Dollar::new(5);
        let money3 = Dollar::new(10);
        let frank = Franc::new(5);

        assert!(Money::eq(&money1, &money2));
        assert!(!Money::eq(&money1, &money3));
        assert!(!Money::eq(&money1, &frank));
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
    fn test_dollar_equality() {
        let dollar1 = Dollar::new(5);
        let dollar2 = Dollar::new(5);
        let dollar3 = Dollar::new(10);

        assert!(Money::eq(&dollar1, &dollar2));
        assert!(!Money::eq(&dollar1, &dollar3));
    }
}
