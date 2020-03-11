use unicode_segmentation::UnicodeSegmentation;

pub struct RailFence {
    rails: u32,
}

#[derive(Default)]
struct RailIter {
    cur: u32,
    rails: u32,
    reverse: bool,
}

impl RailIter {
    fn new(rails: u32) -> Self {
        RailIter {
            rails,
            ..Default::default()
        }
    }
}

impl Iterator for RailIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.cur;
        if self.reverse {
            self.cur -= 1;
        } else {
            self.cur += 1;
        }
        self.reverse ^= self.cur == (self.rails - 1) || self.cur == 0;
        Some(ans)
    }
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        RailFence { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut v: Vec<String> = vec!["".into(); self.rails as usize];
        text.graphemes(true)
            .zip(RailIter::new(self.rails))
            .for_each(|(g, x)| v[x as usize].push_str(g));
        v.into_iter().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let size = cipher.graphemes(true).count();
        let mut indices = vec![Vec::new(); self.rails as usize];
        RailIter::new(self.rails)
            .take(size)
            .enumerate()
            .for_each(|(i, j)| indices[j as usize].push(i));
        let mut parts = vec![""; size];
        cipher
            .graphemes(true)
            .zip(indices.into_iter().flatten())
            .for_each(|(s, i)| parts[i] = s);
        parts.into_iter().collect()
    }
}
