use std::fmt::Display;
use std::ops::Rem;

pub struct Matcher<'a, T> {
    matcher: Box<dyn Fn(T) -> bool + 'a>,
    subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Self
    where
        F: Fn(T) -> bool + 'a,
        S: Display,
    {
        let matcher = Box::new(matcher);
        Self {
            matcher,
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: Copy + PartialEq + Display + 'a,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |n| {
            let mut s = String::new();
            for matcher in &self.matchers {
                if matcher.matcher.as_ref()(n) {
                    s.push_str(matcher.subs.clone().as_str());
                }
            }
            if s.is_empty() {
                s.push_str(&n.to_string())
            }
            s
        })
    }
}

pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Copy + From<u8> + Rem<Output = T> + PartialEq + Display + 'a,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n| n % T::from(5) == T::from(0), "buzz"))
}
