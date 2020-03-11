use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::iter::once;

#[derive(Default, Copy, Clone)]
struct TeamResult {
    won: u32,
    drawn: u32,
    lost: u32,
}

impl TeamResult {
    fn points(&self) -> u32 {
        self.won * 3 + self.drawn
    }
    fn played(&self) -> u32 {
        self.won + self.drawn + self.lost
    }
}

#[rustfmt::skip::macros(format)]
fn team_to_string((name, res): (&str, TeamResult)) -> String {
    format!(
        "{:<31}|  {} |  {} |  {} |  {} |  {}",
        name, res.played(), res.won, res.drawn, res.lost, res.points()
    )
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<_, TeamResult> = HashMap::new();

    match_results.lines().for_each(|s| {
        let mut v: Vec<_> = s.split(';').collect();
        if v[2] == "draw" {
            results.entry(v[0]).or_default().drawn += 1;
            results.entry(v[1]).or_default().drawn += 1;
        } else {
            if v[2] == "loss" {
                v.swap(0, 1);
            }
            results.entry(v[0]).or_default().won += 1;
            results.entry(v[1]).or_default().lost += 1;
        }
    });

    once(HEADER.into())
        .chain(
            results
                .into_iter()
                .sorted_by_key(|&(name, res)| (Reverse(res.points()), name))
                .map(team_to_string),
        )
        .join("\n")
}
