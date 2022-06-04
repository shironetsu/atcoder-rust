use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [String;H],
    }
    let mut t = vec![];
    for i in 0..H {
        for (j, c) in S[i].chars().enumerate() {
            if c == 'o' {
                t.push((i as i32, j as i32));
            }
        }
    }
    let ans = (t[0].0 - t[1].0).abs() + (t[0].1 - t[1].1).abs();
    println!("{}", ans);
}
