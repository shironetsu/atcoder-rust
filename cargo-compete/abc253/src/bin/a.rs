use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    if (a <= b && b <= c) || (c <= b && b <= a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
