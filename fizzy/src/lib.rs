type Predicate<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: Predicate<T>,
    subs: String,
}

impl<T> Matcher<T> {
    // TODO: get rid of 'static lifetime here
    pub fn new<F: Fn(T) -> bool + 'static>(matcher: F, subs: impl ToString) -> Matcher<T> {
        Matcher {
            matcher: Box::new(matcher) as Predicate<T>,
            subs: subs.to_string(),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Copy + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Default::default(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut ans = self;
        ans.matchers.push(matcher);
        ans
    }

    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |t| {
            let s: String = self
                .matchers
                .iter()
                .filter(move |m| (m.matcher)(t))
                .map(|m| m.subs.clone())
                .collect();
            if s.is_empty() {
                t.to_string()
            } else {
                s
            }
        })
    }
}

pub fn fizz_buzz<T: PartialEq + Copy + std::ops::Rem<Output = T> + From<u8> + ToString>() -> Fizzy<T>
{
    Fizzy::new()
        .add_matcher(Matcher::new(|t: T| t % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|t: T| t % T::from(5) == T::from(0), "buzz"))
}
