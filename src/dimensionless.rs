use super::*;

impl Value<DIMENSIONLESS> {
    /// Задать давление в \[Pa\]
    pub fn new_v(value: f64) -> Self {
        Self { value }
    }

    /// Получить давление в \[Pa\]
    pub fn v(&self) -> f64 {
        self.value
    }
}
