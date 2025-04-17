//! Представление физической величины
//!
//! TODO - если зайдет, выделить в отдельный крейт

#![allow(non_snake_case)]

use std::ops;

use serde::{Deserialize, Serialize};

mod dimensionless;
mod pressure;
mod temperature;

/// Тип единица измерения
pub type Unit = u8;
/// Безразмерная величина
pub const DIMENSIONLESS: Unit = 0;
/// Давление
pub const PRESSURE: Unit = 1;
/// Температура
pub const TEMPERATURE: Unit = 2;

/// Физическая величина
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Value<const T: Unit> {
    value: f64,
}

impl<const T: Unit> ops::Add for Value<T> {
    type Output = Value<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Value {
            value: self.value + rhs.value,
        }
    }
}

impl<const T: Unit> ops::Div for Value<T> {
    type Output = Value<DIMENSIONLESS>;

    fn div(self, rhs: Self) -> Self::Output {
        Value {
            value: self.value / rhs.value,
        }
    }
}

impl<const T: Unit> ops::Mul<f64> for Value<T> {
    type Output = Value<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Value {
            value: self.value * rhs,
        }
    }
}

impl<const T: Unit> ops::Mul<Value<T>> for f64 {
    type Output = Value<T>;

    fn mul(self, rhs: Value<T>) -> Self::Output {
        Value {
            value: self * rhs.value,
        }
    }
}

impl<const T: Unit> ops::Sub for Value<T> {
    type Output = Value<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Value {
            value: self.value - rhs.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t1 = Value::<TEMPERATURE>::new_C(0.0);
        let t2: Value<TEMPERATURE> = Value::<TEMPERATURE>::new_K(0.0);
        let _p1 = Value::<PRESSURE>::new_Pa(0.0);

        let _a = t1 / t2;
    }
}
