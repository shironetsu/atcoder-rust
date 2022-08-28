use cargo_snippet::snippet;
use std::fmt::Display;

#[snippet("answer")]
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

#[snippet("answer-impl")]
#[snippet(prefix = "use std::fmt::Display;")]
impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
    }
}
