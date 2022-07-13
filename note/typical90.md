# [競プロ典型 90 問 \- AtCoder](https://atcoder.jp/contests/typical90)

> 来たる 3 月 30 日より、日曜を除く毎朝 7:40 に競プロ・アルゴリズムの典型的問題を Twitter に投稿する企画「#競プロ典型 90 問」をスタートします。
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

$N\leq 200$ なので $O(N^3)$ が間に合うなぁと思う。多分 3 重ループ DP。

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

$a < c^b$ と同値。$13^{17} < 16^{17} = 2^{68}$ なので 64 ビットで大丈夫、たぶん。（まあ本番なら 128 ビット使えば OK）

## [021 \- Come Back in One Piece（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_u)

強連結成分分解を学んだ。

はじめの考えはこうだった：有向グラフから吸い込み口と湧き出し口を順次除いていくと、
残ったグラフの各連結成分内の 2 点は互いに行き来可能になる。

そんなものは全然成り立たない。

ついでに再帰によらない深さ優先探索を知った。
葉に達するまで先入れ後出しで溜めて、葉に達したら戻りながら消費していく。
Python では再帰が遅くなるという都合からよく使うらしい。

- [R's Note: \[競プロ\]再帰の回避方法まとめ](https://r-n-note.blogspot.com/2020/07/blog-post.html)
- [Python で非再帰 DFS を楽に実装したい話 \- なすびの精進日記](https://nasubi-blog.hatenablog.com/entry/2021/09/17/160418)

## [022 \- Cubic Cake（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_v)

$$
\frac{A+B+C}{{\rm gcd}(A,B,C)}-3.
$$

## [023 \- Avoid War（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_w)

TODO

## [024 \- Select \+／\- One（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_x)

$\sum_{i=1}^N |A_i-B_i|$ が $K$ より大きければ不可能、$K$ 以下なら $K$との差が偶数なら可、奇数なら不可。

`zip`を迷いなく使えるようになりたい。
考えれば当たり前だが、タプルの参照ではなくて、参照のタプルのイテレータになる。

```rust
let a1 = [1, 2, 3];
let a2 = [4, 5, 6];

let mut iter = a1.iter().zip(a2.iter());

assert_eq!(iter.next(), Some((&1, &4)));
assert_eq!(iter.next(), Some((&2, &5)));
assert_eq!(iter.next(), Some((&3, &6)));
assert_eq!(iter.next(), None);
```

[Iterator in std::iter \- Rust](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)

## [025 \- Digit Product Equation（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_y)

$m-f(m)=B$ を満たすような $m$ が $0$ の桁を含んでいれば $m=B$ 。$B$ が 0 を含んでいればこのような $m$ はちょうど一つ存在して、含んでいなければ存在しない。

$m$ が $0$ の桁を含まない場合、各桁に現れる数は $2$ から $9$ までのいずれかで、制約から高々 11 桁だから、$2, 3, 5, 7$ を素因数に含めば指数は高々 $33, 22, 11, 11$。$n=f(m)$ として、$2^a\cdot 3^b \cdot 5^c\cdot 7^d$ で全探索を行う。$n= f(B+n)$ でかつ $B+n \leq N$ なら、$m=B+n$ ただ一つがこの $n$ に対応して条件を満たす。

オーバーフロー対策が微妙。Rust なら `i128` 使えば心配ない。最悪 `num_bigint`　がある。

## [026 \- Independent Set on a Tree（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_z)

二部グラフの大きい方から $N/2$ 頂点をとる。

## [027 \- Sign Up Requests （★2）](https://atcoder.jp/contests/typical90/tasks/typical90_aa)

やるだけ。

## [028 \- Cluttered Paper（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_ab)

いもす法、感動……。

[いもす法 \- いもす研 \(imos laboratory\)](https://imoz.jp/algorithms/imos_method.html)

2次元セグ木だと思った。

## [029 \- Long Bricks（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_ac)

LazySegmentTree!

サイズ $W$ を 0 で初期化して、Range Max Query として各クエリを処理。
$[L_i, R_i)$ を $h=（その時点での総積）+1$ の ${\rm max}(h, x)$ による作用で更新。

## [030 \- K Factors（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_ad)

変則のエラトステネスの篩。$s[2]\cdots s[N]$ を0で初期化して、0が見つかればその倍数（それ自身を含む）をインクリメントする。
最終的な値が素因数の個数。

## [031 \- VS AtCoder（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_ae)

TODO

## [032 \- AtCoder Ekiden（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_af)

走る順番に関して全探索。$10! = 3628800$ なので間に合う。

## [033 \- Not Too Bright（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_ag)

だいたい $\lceil H/2\rceil\times \lceil W/2\rceil$ だが、${\rm min}(H,W)=1$ の場合だけ例外。

## [034 \- There are few types of elements（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_ah)
区間に含まれる数の multi set （実装的にはmap）を持ちながら尺取法。

## [036 \- Max Manhattan Distance（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_aj)

$u=x-y, v=x+y$ の座標変換。${\rm max}(u_i-u_{\rm min}, u_{\rm max}-u_i, v_i-v_{\rm min}, v_{\rm max}-v_i)$ が各クエリの答え。

## [037 \- Don't Leave the Spice（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_ak)

TODO

## [038 \- Large LCM（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_al)

`i128`。

## [039 \- Tree Distance（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_am)

辺を基準に考える。その頂点が最短経路上の橋として何回使われるかは、（左にある頂点数）×（右にある頂点数）で決まる。
片側にある頂点数は、両頂点の DFS の$((出た順)-(入った順))/2+1$ のうち小さいほう。

再帰なしの DFS を良い感じにスニペット化したい。

## [040 \- Get More Money（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_an)

TODO

## [041 \- Piles in AtCoder Farm（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_ao)

TODO

## [042 \- Multiple of 9（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_ap)

$K$ が 9 の倍数でなければ 0、9 の倍数なら $dp[i] = 各桁の和がiの数$として DP。$dp[K]$ が答え。

## [043 \- Maze Challenge with Lack of Sleep（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_aq)

各マスに「その頂点に侵入した方向ごとの方向転換の最小値」（4 つ）を持つ。初期値 $2\times 10^6$ で良い。

「探索者」にベクトルを持たせるほうが感覚に合う気がするがうまく書けない。

## [044 \- Shift and Swapping（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_ar)

右シフトを 「第$0$項を指すカーソルの左移動」と考える。$c$ 回の移動後に、カーソルは元の数列の $A_{-c}$ を指す。添え字は ${\rm mod} N$。「数列の第 $x$ 項と第 $y$項」を入れ替える操作は、 数列の第 $x-c$ 項と第 $y-c$項への操作に変わる。

## [045 \- Simple Grouping（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_as)

## [046 \- I Love 46（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_at)

${\rm mod }46$ で集約すると $46^3$ のループで数えられる。 

## [047 \- Monochromatic Diagonal（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_au)

## [048 \- I will not drop out（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_av)
部分点・残りの点のうち大きいほうから $K$ 個の和。部分点の獲得は「残りの点」より先に行う必要があるが、制約から降順ソートを行えば勝手にその順になる。

## [049 \- Flip Digits 2（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_aw)


## [050 \- Stair Jump（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_ax)

$k$ 回一歩で $L$ 段で上る方法を選ぶとすると、残りの $N-Lk$ 段は $1$ 段ずつ上ることになる。その並び替え。

$$
\sum_{k=0}^{\lfloor\frac{N}{L}\rfloor} \binom{k + (N-Lk)}{k}
$$

`ModInt` に `binom` を足した。

## [051 \- Typical Shop（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_ay)

半分全探索。最大で $2^{40}$ 通りなので64ビット整数で数えること。

なかなか4WAくらいが抜けない……と思ったら `upper_bound` の対象をソートしていなかった。
逆に残りはなぜ通ったんだろう。

## [052 \- Dice Product（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_az)

$$
\prod_{i=1}^N \sum_{j=1}^6 A_{ij}.
$$


## [053 \- Discrete Dowsing（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_ba)

TODO

## [054 \- Takahashi Number（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_bb)

雑にBFSを使うとTLEとメモリで落ちる。
高橋数が小さい方から決めていくと、一度その算出に利用した共著論文は再び見る必要は無くなるから、それをメモしながら探索する。

解法2の「超頂点を追加する」方法が鮮やか。

[解説 \- 競プロ典型 90 問](https://atcoder.jp/contests/typical90/editorial/1883)

## [055 \- Select 5（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_bc)

5重ループ。$A_aA_bA_cA_dA_e$ はオーバーフローするので積を取るごとに余りを取る。

itertoolsの `combinations` を使って

```rust
    let ans = (0..N).combinations(5).filter(|c|
        c.into_iter().map(|&i|A[i]).fold(1, |p, x| (p * x).rem_euclid(P)) == Q
    ).count();
```

みたいに書いたらTLEで落ちた。

[itertools/combinations\.rs at master · rust\-itertools/itertools](https://github.com/rust-itertools/itertools/blob/master/src/combinations.rs)

## [056 \- Lucky Bag（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_bd)

DPの復元……！知らない概念だった。

$N<100$ なので「選択している状態」を長さ $N$ のビット列で持って遷移させることは可能。
ただし最大600MBでメモリ制約が危なかった。

## [057 \- Flip Flap（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_be)

$\mathbb{F}_2$ で考える。$M \times N$ 行列 $B$ を

$$
B_{ij} = (パネルiがスイッチjによって裏返るなら1, そうでなければ0)
$$

とすると、

$$
\sum_{i=1}^M x_iB_{ij} = S_j
$$

となる $(x_1, \cdots x_M)$ が 所望のスイッチの押し方に相当する。
この連立一次方程式系の拡大係数行列 $(B|S)$ を基本変形で標準形に持っていくと、
解の有無を判定できる。 もし解が存在する場合、$2^{N-{\rm rank}(B)}$ が解の総数。

「行の交換」「列の交換」を関数化して、各行をbitsetで持つと $\mathbb{F}_2$ なら割と簡単に書ける。

## [058 \- Original Calculator（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_bf)
鳩の巣原理でこの変換（$f(x)$とする）は周期 $10^5$ 以下でループする。
最初の $N$ の「出現点」を $0$、一般に$f^i(N)$ の出現点を $i$ として、
$N, f(N), f^2(N), \cdots$ とその逆（どの数が何番目に現れるか）を辞書でメモして、
2回現れるものがあればそこで停止する。
この過程で $K$ に達していればその時の値を返し、そうでない場合、
ループに入る前の長さ（$\rho$ の尾の部分）を $r$, 周期を $p$ として、数列の

$$
r + ((K-r) {\rm mod} p)
$$

番目の値が答え。

## [059 \- Many Graph Queries（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_bg)

## [060 \- Chimera（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_bh)

最長部分増加列（LIS）を学ぶ。

$dp[i] = (長さが i+1 の部分列の末尾の要素)$ を順に更新する。
その過程で各 $i$ に対して、$A_0, \cdots , A_i$ の最長部分増加列の長さを得る。
$A$ を逆転すると、同様に$A_i, \cdots, A_{N-1}$ の最長部分減少列の長さが分かる。
その和-1が条件を満たす部分列で、最大値が $A_i$ になるようなもの。
全ての $i$ に対してその長さを調べると、最大値が答え。全体で $O(N\log N) $。

## [061 \- Deck（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_bi)

2つのvec `a`と`b`を用意する。
$t_i=1$ なら $x_i$ を `a` へpushし、$t_i=2$ なら `b` へpushする。
カードは 
```
←山札の上               山札の下→
aの末尾...aの先頭 bの先頭...bの末尾
```
というふうに並んでいるので、上から $x$ 番目にもインデックスでアクセスできる。

## [062 \- Paint All（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_bj)

最後に塗るボール $i$ は $A_i=i$ か $B_i=i$ の少なくとも一方が成り立つ必要がある。

$A_i\rightarrow i$ と $B_i\rightarrow i$ を有向辺に持つ有向グラフを考える。
最後に塗るボールは自己ループを持つということ。
自己ループを持つ頂点からの距離が有限で遠いほうから順に塗ると全ての辺を塗ることができる（∵ $i$ を塗る段階では $A_i$ か $B_i$ の少なくとも一方は白い）。
もし到達できない頂点があればその頂点は塗れない。

## [063 \- Monochromatic Subgrid（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_bk)

TODO

## [064 \- Uplift（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_bl)

遅延セグ木を使ってしまう（解けるが）。

階差を持っておいてそれを更新していく。高々2点しか変わらない。
地殻変動の左右端が区画の両端に一致する場合の取り扱いに注意。

## [065 \- RGB Balls 2（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_bm)

TODO

## [066 \- Various Arrays（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_bn)

前から順に1から100までの数の個数の期待値を持って、1から $N$ まで更新する。
各回で転倒数の増分の期待値を計算できる。

## [067 \- Base 8 to 9（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_bo)

基数変換。

基数を指定して文字列→整数型変換はstdにあるが逆の整数→文字列はない。

[i64 \- Rust](https://doc.rust-lang.org/std/primitive.i64.html#method.from_str_radix)

`BigInt`を介することになるが、num_bigint` の `to_str_radix` で36進数まで変換できる。36は0-9, a-z, A-Z。

[BigInt in num\_bigint \- Rust](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html#method.to_str_radix)

## [068 \- Paired Information（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_bp)

$x=y$のとき $A_y=A_x$。

$x<y$のとき、

$$
\beging{align*}
A_y &= V_{y-1}-A_{y-1}\\
    &= V_{y-1}-V_{y-2}+A_{y-2}\\
    &= (-1)^{x-y}A_{x} + (-1)^{y+1}\sum_{i=x}^{y-1} (-1)^{i}V_{i}.
\end{align*}
$$

$y<x$のとき上の式の$x,y$を入れ替えて、

$$
\beging{align*}
A_x &= (-1)^{y-x}A_{y} + (-1)^{x+1}\sum_{i=y}^{x-1} (-1)^{i}V_{i}.
A_y &= (-1)^{x-y}A_{x} + (-1)^{y}\sum_{i=y}^{x-1} (-1)^{i}V_{i}.
\end{align*}
$$

が成り立つ。

$V_{i}$ がこの範囲で確定していることはandを2項演算に持つブーリアンのセグ木かunion-findで調べられる。
もし確定していれば、$(-1)^i V_{i}$ をセグ木で持てば和が計算できる。

こう、上のような式をちゃんと場合分けして計算しておかないと、符号で無駄に躓く。

## [069 \- Colorful Blocks 2（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_bq)

$$
\begin{align*}
{\rm ans} = \left\{
    \begin{array}{cc}
        K & N=1 \\
        K(K-1)(K-2)^{N-2} N \geq 2 \\
    \end{array}
\right.
\end{align*}
$$

高速べき乗。一瞬DPに見える。

## [070 \- Plant Planning（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_br)

中央値がマンハッタン距離を最小化するという偽りの記憶があった。サンプルは通ってしまうが全然嘘。

$N=1$のとき工場と発電所を同じ場所にすればマンハッタン距離の和は0になる。

そうでないとき、$x$ をソート済みとして $x_{m-1} \leq u \leq x_m$となる $m$ （その外側では明らかにマンハッタン距離は最小化しない）をとると、

$$
\begin{align*}
\sum_{i=0}^{N-1}|u-x_{i}| 
    &= \sum_{i=0}^{m-1} u - x_i + \sum_{i=m}^{N-1} x_i-u\\
    &= (2m-N) u + \sum_{i=0}^{N-1} x_i - 2\sum_{i=0}^{m-1} x_{i}.
\end{align*}
$$

$2m-N < 0$ のとき $u=x_{m}$、$2m-N\geq 0$ の とき $u=x_{m-1}$ がこれを最小化する。

$x$ と $y$ 独立なのでそれぞれ最小値を計算する。

## [075 \- Magic For Balls（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_bw)
$x$ の素因数が葉になるような二分木の高さ。$\lceil\log_{2}(xの素因数の数)\rceil$

## [076 \- Cake Cut（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_bx)
尺取法。周期境界条件を考慮するために元の数列 $A$ を2個繋げた数列で考える。

## [077 \- Planes on a 2D Plane（★7）](https://atcoder.jp/contests/typical90/tasks/typical90_by)
TODO

## [078 \- Easy Graph Problem（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_bz)
${\rm min}(a_i, b_i)$ から ${\rm max}(a_i, b_i)$ へ有効辺が生えていると考える。
入次数が1の頂点数が答え。

## [079 \- Two by Two（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_ca)
明らかに同じ領域への操作は増加か減少の一方だけで良い。
正方形領域 $A_{x,y},A_{x+1,y},A_{x,y+1},A_{x+1,y+1},$ への操作を
$C_{x,y} (1\leq x < H-1, 1\leq y < W-1)$ とする。

$$
\begin{align}
A_{0,0}+C_{0,0} &= B_{0,0},\\
A_{0,1}+C_{0,0}+C_{0,1} &= B_{0,1},\\
\vdots
\end{align}
$$

から $C$ は次々に決めることができて、$A_{x,y} (x=H-1 または y=W-1)$ の右端と下端で解が存在するか判定する。

## [080 \- Let's Share Bit（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_cb)
$$
\begin{align}
S_i = \left\{ x \middle| 0\leq x < 2^D, x \& A_i \neq 0 \right\}
\end{align}
$$ 

とする。求めるものは、

$$
ans = \bigcap_{i=1}^{N} S_i.
$$

包除原理から、

$$
\begin{align*}
\bigcap_{i=1}^{N} S_i 
=&\overline{\bigcup_{i=1}^{N} \overline{S_i}}\\
=&2^D - \bigcup_{i=1}^{N} \overline{S_i}\\
=&2^D - \left|\overline{S_1}\right| - \cdots \left|\overline{S_N}\right| \\
&+\left|\overline{S_1}\cap \overline{S_2}\right| + \cdots
+\left|\overline{S_{N-1}}\cap \overline{S_N}\right|\\
&-\left|\overline{S_1}\cap \overline{S_2}\cap \overline{S_3}\right| - \cdots
\end{align*}
$$

各項は

$$
\left|\overline{S_1}\cap \overline{S_2}\cap \overline{S_3}\right|
=\left\{ x \middle| 0\leq x < 2^D, x \& (A_1 |A_2|A_3 ) = 0 \right\}
$$

から、$A_1|A_2|A_3$ の 1 の数を $e$ とすると、
$2^{D-e}$ から計算できる。ビット全探索。0を除外することに注意。

## [081 \- Friendly Group（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_cc)

## [082 \- Counting Numbers（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_cd)

桁数が同じもの（非負整数 $n$ に対して $[10^n, 10^{n+1})$）で部分和を取ってから全体の和を取る。
含まれる数の個数は $[L,R]$ とのオーバーラップ。
$10^19$ が現れるとオーバーフローするので、$10^{18}$ だけ特別に扱う。

## [083 \- Colorful Graph（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_ce)

## [084 \- There are two types of characters（★3）](https://atcoder.jp/contests/typical90/tasks/typical90_cf)

全ての $(l,r)$　のペアの個数（$N(N+1)/2$ 個）から、`o` しか含まれない区間と `x` しか含まれない区間の個数を引く。
`o` しか含まれない区間は、`o` が連続する区間の部分区間。ちょうど $m$ 個連続する区間に対して部分区間は $m(m+1)/$ 個ある。
これを全ての連続区間（ちょうど1個だけの場合も含む）で取る。`x`についても同様。

ランレングス圧縮（Run Length Encoding; RLE）。
[連長圧縮 \- Wikipedia](https://ja.wikipedia.org/wiki/%E9%80%A3%E9%95%B7%E5%9C%A7%E7%B8%AE)

## [085 \- Multiplication 085（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_cg)

大小関係を考えない場合、$a,b,c$ に $K$ の素因数をどのように割り振れるか考える。
素因数 $p$ を指数 $e$ で含むとすると、そのわり振り方は${}_eH_3$（重複組み合わせ）通り。
全ての素因数に渡ってこの積を取る。これを $p$ とする。

$$
P = \left\{(a,b,c)\middle|abc=K\right\}
$$

次に$a=b$の場合を考える。この場合、上の記号で素因数 $p$ に対して $\lceil e/2 \rceil + 1$ 通り。
全体では同様に素因数に関する積。これを $q$ とする。

$$
Q = \left\{(a,b,c)\middle|abc=K, a=b\right\}
$$

$a=b=c$ の場合は、$K$ が立方数なら1, そうでなければ0。これを $r$ とする。

$$
\begin{align}
U_{1} &= \left\{(a,b,c)\middle|abc=K,\,a<b<c\right\},\\
U_{2} &= \left\{(a,b,c)\middle|abc=K,\,b<c<a\right\},\\
U_{3} &= \left\{(a,b,c)\middle|abc=K,\,c<a<b\right\},\\
U_{4} &= \left\{(a,b,c)\middle|abc=K,\,a<c<b\right\},\\
U_{5} &= \left\{(a,b,c)\middle|abc=K,\,c<b<a\right\},\\
U_{6} &= \left\{(a,b,c)\middle|abc=K,\,b<a<c\right\},\\
V_{1} &= \left\{(a,b,c)\middle|abc=K,\,a<b=c\right\},\\
V_{2} &= \left\{(a,b,c)\middle|abc=K,\,b=c<a\right\},\\
V_{3} &= \left\{(a,b,c)\middle|abc=K,\,a=b<c\right\},\\
V_{4} &= \left\{(a,b,c)\middle|abc=K,\,c<a=b\right\},\\
V_{5} &= \left\{(a,b,c)\middle|abc=K,\,a=c<b\right\},\\
V_{6} &= \left\{(a,b,c)\middle|abc=K,\,b<a=c\right\},\\
W &= \left\{(a,b,c)\middle|abc=K,\,a=b=c\right\},\\
\end{align}
$$

$$
\begin{align}
U_{1}+\cdots+U_{6} + V_{1}+\cdots+V_{6} + W &= p,\\
V_{1}+V_{3} + W &= q,\\
W &= r,\\
\end{align}
$$

$$
ans = U_{1} + V_{1} + V_{3} + W = \frac{p+3q+2r}{6}
$$

これ必ず自然数になるの面白いな。