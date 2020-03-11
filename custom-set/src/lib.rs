#[derive(Debug, PartialEq, Clone)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + Ord> CustomSet<T> {
    fn from_sorted_dedup_vec(data: Vec<T>) -> Self {
        CustomSet { data }
    }

    pub fn new(input: &[T]) -> Self {
        let mut v = input.to_vec();
        v.sort();
        v.dedup();
        CustomSet::from_sorted_dedup_vec(v)
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        if let Err(idx) = self.data.binary_search(&element) {
            self.data.insert(idx, element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.len() <= other.data.len() && self.data.iter().all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.data.len() < other.data.len() {
            other.is_disjoint(&self)
        } else {
            !other.data.iter().any(|e| self.contains(e))
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        if self.data.len() < other.data.len() {
            other.intersection(&self)
        } else {
            Self::from_sorted_dedup_vec(
                self.data
                    .iter()
                    .filter(|&e| other.contains(e))
                    .cloned()
                    .collect(),
            )
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self::from_sorted_dedup_vec(
            self.data
                .iter()
                .filter(|&e| !other.contains(e))
                .cloned()
                .collect(),
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        if self.data.len() < other.data.len() {
            other.union(&self)
        } else {
            let mut ans = self.clone();
            other.data.iter().for_each(|e| ans.add(e.clone()));
            ans
        }
    }
}
