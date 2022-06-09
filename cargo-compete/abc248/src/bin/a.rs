use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input!{
        S: String,
    }

    let sum = S.chars().map(|x|{x.to_digit(10).unwrap()}).sum::<u32>();
    println!("{}", 45-sum);
}
