# リンク

- [AtCoder：競技プログラミングコンテストを開催する国内最大のサイト](https://atcoder.jp/)
- [qryxip/cargo\-compete: A Cargo subcommand for competitive programming](https://github.com/qryxip/cargo-compete)
- [2020 Update · rust\-lang\-ja/atcoder\-rust\-resources Wiki](https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update)
- [アルゴリズムロジック](https://algo-logic.info/)
- [Main Page \- Algorithms for Competitive Programming](https://cp-algorithms.com/#navigation)
- [AIZU ONLINE JUDGE: Programming Challenge](https://judge.u-aizu.ac.jp/onlinejudge/)
- [nekolib \- Rust](https://rsk0315.github.io/library-rs/nekolib/index.html)

Rust の AC 解答の URL

例：

```
https://atcoder.jp/contests/abc256/submissions?f.Task=abc256_a&f.LanguageName=Rust&f.Status=AC&f.User=
```

# メモ

## 公式doc
- リスポーン地点。
[Iterator in std::iter \- Rust](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## バージョン 1.42.0 で使えないもの

### BTreeSet の first

unstable

### `is_sorted()` : unstable

[is_sorted \- The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/library-features/is-sorted.html)

## 2 次元配列の初期化が紛らわしい。

```rust
let mut v = vec![vec![0;M];N];
```

マクロ例：

```rust
macro_rules! vvec {
    ($val: expr; $a:expr, $b:expr) => {
        vec![vec![$val; $b]; $a]
    };
}

```

## マップの初期化・更新

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
//略
*map.entry(key).or_insert(initialValue) += delta;
```

## BinaryHeap の iter()は pop の順ではない。

それに `iter()` はヒープを消費しない。

[BinaryHeap in std::collections \- Rust](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.iter)

ソート順のイテレータに変換数`into_iter_sorted`が nightly にはある。

## jq

cargo-compete でエディタを開くために `jq` が要る。

インストール方法：
[bash \- How to run jq from gitbash in windows? \- Stack Overflow](https://stackoverflow.com/questions/53967693/how-to-run-jq-from-gitbash-in-windows)

## cargo-snippet

競プロ用スニペット管理。専用のクレートを作って、`cargo compete`を走らせるとスニペット形式に変換して出力してくれる。

[hatoo/cargo\-snippet: A snippet extrator for competitive programmers](https://github.com/hatoo/cargo-snippet)

[Rust で競技プログラミングをするときの"スニペット管理"をまじめに考える\(cargo\-snippet の紹介\) \- Qiita](https://qiita.com/hatoo@github/items/5c6814e72ddd2ecaf48f)

インストールするとき引っ掛かった。

```
[net]
git-fetch-with-cli = true
```

にするか、`CARGO_NET_GIT_FETCH_WITH_CLI`を true にすれば良い。

- [Configuration \- The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli)
- [\[Rust\] cargo install で文字化けして通信エラーが発生する │ Web 備忘録](https://webbibouroku.com/Blog/Article/cargo-install-error)
- [Cargo build failed with spurious network error · Issue \#6513 · rust\-lang/cargo](https://github.com/rust-lang/cargo/issues/6513)

VSCode では`.vscode`フォルダ内に `.code-snippets` のサフィックスを付けて json を書いてやると、ワークスペースレベルのスニペットを設定できる。ここに吐くとよい。

[Snippets in Visual Studio Code](https://code.visualstudio.com/docs/editor/userdefinedsnippets)

```
cargo snippet -tvscode > <スニペット.code-snippetsのパス>
```

## Range

`start..end` みたいなのは `std::ops::Range` の構造体で、`Iterator` と `SliceIndex` を備えている。

[Range in std::ops \- Rust](https://doc.rust-lang.org/std/ops/struct.Range.html)

インデックスアクセス `[]` は `Index` トレイトによるもので、
`SliceIndex` を備えた型を引数に取れる。

[Vec in std::vec \- Rust](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-Index%3CI%3E)

だから

```rust
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    v[2..6].reverse();
    assert_eq!(v, vec![0, 1, 5, 4, 3, 2, 6, 7]);
```

みたいなことができる。

## グラフ

何回書いたか分からない。スニペットにする。

```rust
input!{
    edges: [(Usize1, Usize1); N],
}

let mut ad = vec![vec![];N];
for (a b) in edges{ //消費してよい
    glaph[a].push(b);
    glaph[b].push(a);
}
```

## 数について

## `lower_bound` と `upper_bound`

`superslice::Ext` から使える。

[superslice::Ext \- Rust](https://docs.rs/superslice/1.0.0/superslice/trait.Ext.html)

`lower_bound(x)`: `x`より小さくない最初の要素のインデックス

`upper_bound(x)`: `x` より大きい最初の要素のインデックス

```
l  9
  10
     11
        12
        13
              14
              15
                 16
a 10 11 13 13 15
                 16
                 15
              14
              13
        12
        11
     10
u  9
```

- 必ず `l <= u`。
- 配列長 `n` に対して、`l` と `u` は最小0、最大`n`。`=n`のケースだけインデックスが範囲を超えるので注意。
- `l=u` となるのは、配列に `x` が存在しないとき。
- インデックスが `[l, u)` の範囲が `x` の存在する領域 →`u-l=(xの個数)`。
    - `equal_range` で `Range` を直接取得できる。
- 特に、`x` が `a` にちょうど一つ存在するとき、`l=(xのインデックス)`、`u=(xのインデックス+1)`
- `(-∞, a[0]]` に対して `l=0`。`(-∞, a[0])` に対して `u=0`。
- `a` の長さを `n` とする。`(a[n-1], ∞)` に対して`l=n`。`[a[n-1], ∞)` に対して `u=n`

## Union-find

```rust
use petgraph::unionfind::UnionFind;

let mut uf = UnionFind::new(10); //サイズ指定
let edges = vec![(0,1), (1,2), (2,3), (1,3)];
for &(a, b) in edges.iter(){
    if uf.equiv(a, b){
        uf.unite(a, b);
    }
}
```

- `pub fn new(n: usize) -> Self`: 初期化
- `pub fn find(&self, x: K) -> K`: 代表元を取得
- `pub fn equiv(&self, x: K, y: K) -> bool`: 同値判定
- `pub fn union(&mut self, x: K, y: K) -> bool`: 結合
- `pub fn into_labeling(self) -> Vec<K>`: 代表元の配列に変換

[UnionFind in petgraph::unionfind \- Rust](https://docs.rs/petgraph/latest/petgraph/unionfind/struct.UnionFind.html)

各連結成分のサイズ
```rust
let reps = uf.into_labeling();
let mut m = BTreeMap::<usize, usize>::new();
for r in reps{
    *m.entry(r).or_default() += 1;
}
```

## iter
### filter

イテレーターは多くの場合要素の参照を処理して、`filter` は（さらにその）参照を引数に取るため、例にあるように参照外しが2回要る。

```rust
let a = [0, 1, 2];

let mut iter = a.iter().filter(|x| **x > 1); // need two *s!

assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);
```

> Because the closure passed to filter() takes a reference, and many iterators iterate over references, this leads to a possibly confusing situation, where the type of the closure is a double reference:

[Iterator in std::iter \- Rust](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

## 桁あふれ

`vec!` とか `btreemap!` とかでサボって痛い目を見がち。`let mut ans = 0i64`とかで初期化するのが良いか？

## count_ones

[u32 \- Rust](https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones)

## 再帰でDFS 
```rust
fn dfs(i: usize, seen: &mut Vec<bool>, ad: &Vec<Vec<usize>>, other_data: &Data) -> Value{
    for &j in ad[i].iter() {
        if seen[j] {
            continue;
        }
        seen[j] = true;
        let val = dfs(v, seen, ad, other_data);
    }
}

fn main(){
    //入力
    let mut seen = vec![false; N];
    seen[0] = true; //大事
    let ans = dfs(0, &mut seen, &ad, &c);
}
```
## 無限

`std::{integer}::MAX` を無限として扱うべきではない。
$\infty+1=\infty$ を利用することがあるため。

## swap
```rust
std::mem::swap(&mut A[i], &mut A[j]);
```
借用ルールに違反するためこれはできない。

## チェス

割とよく出る。

|駒名|移動方向|メモ|
|:---:|:---:|:---:|
|♔キング|縦・横・斜め1マス||
|♕クイーン|縦・横・斜め任意マス||
|♖ルーク|縦・横任意マス||
|♗ビショップ|斜め任意マス|$x+y$ のパリティ不変|
|♘ナイト|$(x,y)→(x\pm 2, y\pm 1),(x\pm 1, y\pm 2)$|$x+y$ のパリティ変化|
|♙ポーン|略||


- [チェスのルール \- Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%81%E3%82%A7%E3%82%B9%E3%81%AE%E3%83%AB%E3%83%BC%E3%83%AB)
- [エイト・クイーン \- Wikipedia](https://ja.wikipedia.org/wiki/%E3%82%A8%E3%82%A4%E3%83%88%E3%83%BB%E3%82%AF%E3%82%A4%E3%83%BC%E3%83%B3)
- [ナイト・ツアー \- Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%8A%E3%82%A4%E3%83%88%E3%83%BB%E3%83%84%E3%82%A2%E3%83%BC)
- [E \- Bishop 2](https://atcoder.jp/contests/abc246/tasks/abc246_e)
- [E \- Queen on Grid](https://atcoder.jp/contests/abc183/tasks/abc183_e)

## Enum
列挙型を定義するとクエリの保存が楽になることがある。
```rust
pub enum Op { //operation
    Add(usize, i64),
    Shift,
    Show(usize),
}
```
みたいな。
「ラベル」を単に整数で保存するより混乱を避けられるし、パターンマッチで値が取り出せるのも良い。

その際、だいたいプリミティブ型しか値に持たないので、
```rust
#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Debug)]
```
は付けておくと良い。
