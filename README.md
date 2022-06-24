# リンク

- [AtCoder：競技プログラミングコンテストを開催する国内最大のサイト](https://atcoder.jp/)
- [qryxip/cargo\-compete: A Cargo subcommand for competitive programming](https://github.com/qryxip/cargo-compete)
- [2020 Update · rust\-lang\-ja/atcoder\-rust\-resources Wiki](https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update)

RustのAC解答のURL

例：
```
https://atcoder.jp/contests/abc256/submissions?f.Task=abc256_a&f.LanguageName=Rust&f.Status=AC&f.User=
```

# メモ

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