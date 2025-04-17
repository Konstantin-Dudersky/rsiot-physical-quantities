use super::*;

impl Value<PRESSURE> {
    /// Задать давление в \[Pa\]
    pub fn new_Pa(value: f64) -> Self {
        Self { value }
    }

    /// Получить давление в \[Pa\]
    pub fn Pa(&self) -> f64 {
        self.value
    }

    /// Задать давление в \[Pa\]
    pub fn new_kPa(value: f64) -> Self {
        Self {
            value: value * 1000.0,
        }
    }

    /// Получить давление в \[Pa\]
    pub fn kPa(&self) -> f64 {
        self.value / 1000.0
    }

    /// Получить давление в \[мм рт. ст.\]
    pub fn mmHg(&self) -> f64 {
        self.value * 0.00750063755419211
    }
}
