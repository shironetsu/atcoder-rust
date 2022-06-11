use proconio::input;

macro_rules! min {
    ($($x:expr),+) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            *v.iter().min().unwrap()
        }
    };
}

macro_rules! chmin {
    ($x: expr, $y: expr) => {
        if $x > $y {
            $x = $y;
            true
        } else {
            false
        }
    };
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u64;N],
    }

    let mut ans = std::u64::MAX;

    //A0
    for p in 0..=1 {
        let mut dp = vec![vec![0; 2]; N];
        dp[0][1] = if p == 0 { std::u64::MAX } else { A[0] };
        dp[0][0] = if p == 0 { 0 } else { std::u64::MAX };
        for i in 1..N {
            dp[i][0] = if p == 0 && i == N - 1 {
                std::u64::MAX
            } else {
                dp[i - 1][1]
            };
            dp[i][1] = min!(dp[i - 1][0], dp[i - 1][1]) + A[i];
        }
        chmin!(ans, min!(dp[N - 1][0], dp[N - 1][1]));
    }

    println!("{}", ans);
}
