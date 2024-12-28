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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_money_by_shares() {
        // Arrange: создаем Money, представляющую цену за одну акцию - $5 USD
        let price_per_share = Money {
            amount: 5,
            currency: String::from("USD"),
        };

        // Act: умножаем на количество акций (3)
        let total = price_per_share.multiply(3);

        // Assert: проверяем, что результат равен $15 USD
        let expected = Money {
            amount: 15,
            currency: String::from("USD"),
        };

        assert_eq!(total, expected);
    }
}

