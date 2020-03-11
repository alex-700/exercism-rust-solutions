use luhn_from::Luhn as LuhnFrom;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Clone> Luhn for T
where
    LuhnFrom: From<T>,
{
    fn valid_luhn(&self) -> bool {
        LuhnFrom::from(self.clone()).is_valid()
    }
}
