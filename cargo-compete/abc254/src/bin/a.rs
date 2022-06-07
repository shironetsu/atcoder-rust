use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    println!("{}", N.chars().skip(1).collect::<String>());
}
