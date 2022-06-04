use proconio::input;

const MODULO: i64 = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    if K == 0 {
        let mut ans = 1;
        for _ in 0..N {
            ans *= M as i64;
            ans = ans.rem_euclid(MODULO);
        }
        println!("{}", ans);
        return;
    }

    let mut p = vec![vec![0; M + 1]; N];

    for j in 1..=M {
        p[0][j] = 1;
    }

    for i in 1..N {
        let a = p[i - 1].iter().sum::<i64>().rem_euclid(MODULO);
        let mut b = p[i - 1]
            .iter()
            .skip(1)
            .take(K)
            .sum::<i64>()
            .rem_euclid(MODULO);
        //println!("{} {} {}",i, a, b);
        for j in 1..=M {
            p[i][j] = (a - b).rem_euclid(MODULO);
            b -= if j + 1 >= K { p[i - 1][j + 1 - K] } else { 0 };
            b += if j + K <= M { p[i - 1][j + K] } else { 0 };
        }
    }
    let ans = p[N - 1].iter().sum::<i64>().rem_euclid(MODULO);
    println!("{}", ans);
}
