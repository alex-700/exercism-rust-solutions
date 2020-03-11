use crate::Allergen::*;
use std::iter::successors;

pub struct Allergies {
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl From<usize> for Allergen {
    fn from(x: usize) -> Self {
        ALLERGENS[x]
    }
}

impl Allergen {
    fn score(self) -> u32 {
        match self {
            Eggs => 1,
            Peanuts => 2,
            Shellfish => 4,
            Strawberries => 8,
            Tomatoes => 16,
            Chocolate => 32,
            Pollen => 64,
            Cats => 128,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: Allergen) -> bool {
        (self.score & allergen.score()) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        successors(Some(self.score), |&x| Some(x / 2).filter(|&x| x != 0))
            .enumerate()
            .filter_map(|(i, x)| Some(i).filter(|&i| i < ALLERGENS.len() && x % 2 == 1))
            .map(Allergen::from)
            .collect()
    }
}
