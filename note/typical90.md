# [競プロ典型 90 問 \- AtCoder](https://atcoder.jp/contests/typical90)

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">来たる 3 月 30 日より、日曜を除く毎朝 7:40 に競プロ・アルゴリズムの典型的問題を Twitter に投稿する企画「<a href="https://twitter.com/hashtag/%E7%AB%B6%E3%83%97%E3%83%AD%E5%85%B8%E5%9E%8B90%E5%95%8F?src=hash&amp;ref_src=twsrc%5Etfw">#競プロ典型90問</a>」をスタートします。<br><br>解説・サンプルコードなども GitHub 上に公開される形式になる予定です。皆さんお楽しみに！<br>GitHub：<a href="https://t.co/v0FgbqbwGn">https://t.co/v0FgbqbwGn</a> <a href="https://t.co/Tp0NFa1TqQ">pic.twitter.com/Tp0NFa1TqQ</a></p>&mdash; E869120🗼 (@e869120) <a href="https://twitter.com/e869120/status/1376089196100653060?ref_src=twsrc%5Etfw">March 28, 2021</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

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

各点を中心にとった単位円内で、もっとも角度が180度に近い2点を取る。`f64` の `tan2` をあらかじめ計算しておいて、`lower_bound`。

[OrderedFloat in ordered\_float \- Rust](https://docs.rs/ordered-float/latest/ordered_float/struct.OrderedFloat.html#impl-Add%3C%26%27a%20OrderedFloat%3CT%3E%3E)

OrderedFloatとの邂逅。ソートするだけなら

```rust
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

[How to sort a Vec of floats? \- help \- The Rust Programming Language Forum](https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838)

でも良いが、`lower_bound` 使うなら `OrderedFloat` で包んでやった方が楽。

……と思ったが、AtCoderで使える `OrderedFloat` のバージョンが古くて四則演算をimplしていなくて面倒。自前でimplする？　うーん。

`lower_bound_by_key(&x, |x| OrderdFloat(x))` か、インポートせずに `lower_bound_by(|other| other.partial_cmp(&x).unwrap())` がいいかも。

[superslice::Ext \- Rust](https://docs.rs/superslice/latest/superslice/trait.Ext.html#tymethod.lower_bound_by_key)

## [010 \- Score Sum Queries（★2）](https://atcoder.jp/contests/typical90/tasks/typical90_j)

やるだけ。片方のクラスに生徒がいないとき、仮想的にその番号の生徒がクラスにいて0点だと数えておけばよい。