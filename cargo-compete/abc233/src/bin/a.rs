use proconio::input;
use num_integer;

#[allow(non_snake_case)]
fn main() {
    input!{
        X: i32,
        Y: i32,
    }

    let ans = num_integer::div_ceil((Y-X).max(0), 10);
    println!("{}", ans);
}
