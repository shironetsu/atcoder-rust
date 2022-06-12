use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
    }

    let r = (A * A + B * B).sqrt();
    println!("{} {}", A / r, B / r);
}
