use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    grade_to_student: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_to_student
            .entry(grade)
            .or_default()
            .insert(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_to_student.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grade_to_student
            .get(&grade)
            .map(|v| v.iter().cloned().collect())
    }
}
