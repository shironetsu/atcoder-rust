use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i64,
        X: i64,
        mut A: [i64;N],
    }

    let sum = A.iter().sum::<i64>();
    let rem = A.iter().map(|&a| a.rem_euclid(X)).sum::<i64>();
    let c = (sum - rem) / X;
    let ans = if c > K {
        sum - K * X
    } else {
        let c = K - c;
        let mut res = A
            .iter()
            .map(|&a| a.rem_euclid(X))
            .filter(|&r| r != 0)
            .collect::<Vec<_>>();
        res.sort_by_key(|&r| -r);
        res.iter().skip(c as usize).sum()
    };
    println!("{}", ans);
}
