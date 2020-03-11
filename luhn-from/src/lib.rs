use luhn::is_valid;

pub struct Luhn {
    s: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(&self.s)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(t: T) -> Self {
        Luhn { s: t.to_string() }
    }
}
