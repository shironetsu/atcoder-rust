# [競プロ典型 90 問 \- AtCoder](https://atcoder.jp/contests/typical90)

> 来たる 3 月 30 日より、日曜を除く毎朝 7:40 に競プロ・アルゴリズムの典型的問題を Twitter に投稿する企画「#競プロ典型90問」をスタートします。
> 
> 解説・サンプルコードなども GitHub 上に公開される形式になる予定です。皆さんお楽しみに！
> GitHub：https://github.com/E869120/kyopro_educational_90
>
> at: 2021-03-28
> from: https://twitter.com/e869120/status/1376089196100653060?s=20&t=tL_otlPTigK2rTmQVbepwQ

プロジェクトサイズがでかすぎるからか IntelliSense が遅い。

## [001 \- Yokan Party（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_a)

「スコア」を$m$ として二分探索。左から決め打ちした長さで切る。$(最後に切った位置)+m$ の `lower_bound` が次に切る位置。
$A_{N+1} = L$ を追加しないと、「余り」が出ない場合に最後の一片を切れなくなるので注意。

なんか雰囲気で最初 $A_i$ の累積和を取ってしまう、というのを C++ で解いたときもやった気がする。

## [002 \- Encyclopedia of Parentheses（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_b)

$N=20$ なので `0` → `(`、`1` → `)` として $2^N$ 回の全探索。
最初は $N=2$ から初めて定義に従って括弧列を増やす方法でやっていた。

## [003 \- Longest Circular Road（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_c)

$N$ 頂点・$N-1$ 辺・連結なので木。まず町 0 から最遠の町 $f$ （同じ距離ならどれでもよい）を BFS で見つける。
その $f$ から再び BFS で最遠の町までの距離を求めると、それが直径。答えは直径+1。

## [004 \- Cross Sum（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_d)

やるだけ。

## [005 \- Restricted Digits（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_e)

TODO

## [006 \- Smallest Subsequence（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_f)

1 文字目をどこから取るかを考えると、1 文字目から $N-K+1$ （含む）文字目までの中で、最小・最左のものを取ればよい。
各文字について、文字のある位置を配列で持って、`lower_bound` がその配列の長さ以上でなければ採用。
文字はたかだか 26 種なので、答えの文字列の各位置にある文字は、`a` から `z` まで順に採用可否を調べればよい。

↑1 年以上前に C++で解いたときと同じ発想だった。$O(K \log N)$。解説に $O(N)$ の方法がある。

## [007 \- CP Classes（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_g)

二分探索。差の最小は `i = lower_bound` と `i = upper_bound` どちらでも良くて、`i-1` と `i` のそれぞれで配列内か確認すればどちらかが適格。

## [008 \- AtCounter（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_h)

DP。文字が増えない場合の遷移（行列の単位元成分）を忘れがち。

## [009 \- Three Point Angle（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_i)

各点を中心にとった単位円内で、もっとも角度が 180 度に近い 2 点を取る。`f64` の `tan2` をあらかじめ計算しておいて、`lower_bound`。

[OrderedFloat in ordered_float \- Rust](https://docs.rs/ordered-float/latest/ordered_float/struct.OrderedFloat.html#impl-Add%3C%26%27a%20OrderedFloat%3CT%3E%3E)

OrderedFloat との邂逅。ソートするだけなら

```rust
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

[How to sort a Vec of floats? \- help \- The Rust Programming Language Forum](https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838)

でも良いが、`lower_bound` 使うなら `OrderedFloat` で包んでやった方が楽。

……と思ったが、AtCoder で使える `OrderedFloat` のバージョンが古くて四則演算を impl していなくて面倒。自前で impl する？　うーん。

`lower_bound_by_key(&x, |x| OrderdFloat(x))` か、インポートせずに `lower_bound_by(|other| other.partial_cmp(&x).unwrap())` がいいかも。

[superslice::Ext \- Rust](https://docs.rs/superslice/latest/superslice/trait.Ext.html#tymethod.lower_bound_by_key)

## [010 \- Score Sum Queries（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_j)

やるだけ。片方のクラスに生徒がいないとき、仮想的にその番号の生徒がクラスにいて 0 点だと数えておけばよい。

## [011 \- Gravy Jobs（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_k)

TODO

## [012 \- Red Painting（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_l)

Union-Find。ある頂点を赤く塗るたびに、上下左右の隣接する頂点で既に塗られているものと繋げる。

`x + !0` で `usize`をパニクらせずに `-1` できる。ただし debug モードだとオーバーフローする。
お行儀が良い方法は `.wrapping_add(!0)` だがタイプ数が多い。

## [013 \- Passing（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_m)

始点が 1,$N$ それぞれの場合で Dijkstra 法。

## [016 \- Minimum Coins（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_p)

$Ax+By+Cz=N$ を満たす正整数 $(x,y,z)$ で $x+y+z$ が最小のもの。
$x$ を決めると、拡張ユークリッドで $By+Cz=N-Ax$ の基本解が決まる。
$B\leq C$ とすると $y$ はできるだけ小さな整数がよく、その場合の $y$ が正なら解として適格。

## [017 \- Crossing Segments（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_q)

TODO

## [018 \- Statue of Chokudai（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_r)

やるだけ。

## [019 \- Pick Two（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_s)

$N\leq 200$ なので $O(N^3)$ が間に合うなぁと思う。多分3重ループDP。

操作のパターンは $(2N-1)!!$ 通りなので全部を比較していては間に合わない。
「直接比較しなくても大小関係が判定できる操作の組合せ」がある。
部分列での最小値を求める部分問題に落とし込む。

部分列 $A_i, A_{i+1}, \cdots, A_{i+l-1}$ （$l$ は偶数）の先頭 $A_i$ が対消滅する相手は、
$A_{i+1}, A_{i+3}, \cdots A_{i+l-1}$ のいずれかで、
仮に$A_{i+n}$ と結びつくとすると、$A_{i+1}, \cdots, A_{i+n-1}$ はこの結びつきによって「隔離」される。
後続の $A_{i+n+1}, \cdots, A_{i+l-1}$ も同様にその外側に相手はいない。
$dp[i][i+l]$ をこの（右半開）区間の最小値、$dp[i][i]=0$ を $i=0~2N$ で初期化すると、最小値が次々に決まっていく。
$dp[0][2N]$ が答え。

## [020 \- Log Inequality（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_t)

$a < c^b$ と同値。$13^{17} < 16^{17} = 2^{68}$ なので64ビットで大丈夫、たぶん。（まあ本番なら128ビット使えばOK）

## [021 \- Come Back in One Piece（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_u)

