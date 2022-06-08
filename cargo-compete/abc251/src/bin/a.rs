use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }

    println!("{}", S.repeat(6 / S.len()));
}
