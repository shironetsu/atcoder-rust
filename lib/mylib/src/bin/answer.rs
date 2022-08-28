use mylib::answer::Answer;

fn main() {
    let v = vec![1, 2, 3];
    v.ans();
    v.ansl();

    let w = vec!["foo", "bar", "baz"];
    w.ansl();
}
