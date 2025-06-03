use std::cmp::PartialEq;
use std::fmt::{Formatter, Result, Display};
use std::ops::Add;
use serde_json::Value;

#[derive(PartialEq, Clone, Debug)]
pub enum Currency {
    Dollar,
    Euro,
    None
}

pub struct Money {
    currency: Currency,
    whole: u64,
    cents: u64,
}

impl Money {
    pub fn from_json_number(n: Value, currency: Currency) -> Option<Self> {
        let number = match n {
            Value::Number(num) => num.as_f64().unwrap(),
            Value::String(str) => {
                str.replace(',', ".").parse::<f64>().unwrap() 
            },
            _ => return None
        };
        
        Some(Self {
            currency,
            whole: number as u64,
            cents: (number - (number as u64) as f64 * 10.) as u64,
        })
    }
}
impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            currency: self.currency.clone(),
            whole: self.whole + other.whole + 1   * (self.cents + other.whole > 100) as u64,
            cents: self.cents + other.cents - 100 * (self.cents + other.cents > 100) as u64,
        }
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = match self.currency {
            Currency::Dollar => "$",
            Currency::Euro => "€",
            Currency::None => "",
        };
        write!(f, "{}{}.{:02}", symbol, self.whole, self.cents)
    }
}