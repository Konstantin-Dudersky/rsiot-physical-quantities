use super::*;

const C_TO_K: f64 = 273.15;

impl Value<TEMPERATURE> {
    /// Задать температуру в \[℃\]
    pub fn new_C(value: f64) -> Self {
        Self {
            value: value + C_TO_K,
        }
    }

    /// Получить температуру в \[℃\]
    pub fn C(&self) -> f64 {
        self.value - C_TO_K
    }

    /// Задать температуру в \[K\]
    pub fn new_K(kelvin: f64) -> Self {
        Self { value: kelvin }
    }

    /// Получить температуру в \[K\]
    pub fn K(&self) -> f64 {
        self.value
    }
}
