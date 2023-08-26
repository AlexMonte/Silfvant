#[derive(Eq, PartialEq, Clone, Debug)]
pub enum FactValue {
    None,
    Is(bool),
    Int(i32),
    Float(f32),
    String(String),
}

pub struct WorldValue {
    pub value: FactValue,
    pub salience: i8,
}

impl WorldValue {
    pub fn new(value: FactValue, salience: i8) -> Self {
        Self { value, salience }
    }
    //limit salience to 0 - 255
    pub fn increase_salience(&mut self, salience: i8) -> i8 {
        if self.salience + salience > 255 {
            255
        }
        self.salience += salience
    }
    pub fn decrease_salience(&mut self, salience: i8) -> i8 {
        if self.salience - salience < 0 {
            0
        }
        self.salience -= salience;
    }
    pub fn set_salience(&mut self, salience: i8) -> Result<(), String> {
        if salience < 0.0 || salience > 1 {
            return Err("Salience must be between 0.0 and 1.0".to_string());
        }
        self.salience = salience;
        Ok(())
    }

    fn eq_with_salience(&self, other: &Self) -> bool {
        if self.value == other.value && self.salience == other.salience {
            return true;
        }
        false
    }
}

impl Default for WorldValue {
    fn default() -> Self {
        Self {
            value: FactValue::None,
            salience: 0,
        }
    }
}

impl Eq for WorldValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
