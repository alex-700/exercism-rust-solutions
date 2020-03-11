use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: Bucket,
) -> Option<BucketStats> {
    let (start, forbidden) = match start_bucket {
        Bucket::One => ((capacity_1, 0), (0, capacity_2)),
        Bucket::Two => ((0, capacity_2), (capacity_1, 0)),
    };
    let mut q = VecDeque::new();
    let mut h = HashMap::new();
    q.push_back(start);
    h.insert(start, 1);
    while let Some(p) = q.pop_front() {
        if p == forbidden {
            continue;
        }
        let moves = h[&p];
        let (c1, c2) = p;
        if c1 == goal || c2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: if c1 == goal { Bucket::One } else { Bucket::Two },
                other_bucket: c1 + c2 - goal,
            });
        }
        let next_states = vec![
            (capacity_1, c2),
            (c1, capacity_2),
            (c1, 0),
            (0, c2),
            {
                let d = c1.min(capacity_2 - c2);
                (c1 - d, c2 + d)
            },
            {
                let d = c2.min(capacity_1 - c1);
                (c1 + d, c2 - d)
            },
        ];
        q.extend(next_states.into_iter().filter(|&x| match h.entry(x) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(moves + 1);
                true
            }
        }));
    }
    None
}
