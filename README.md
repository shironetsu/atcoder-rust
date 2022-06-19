# リンク
- [AtCoder：競技プログラミングコンテストを開催する国内最大のサイト](https://atcoder.jp/)
- [qryxip/cargo\-compete: A Cargo subcommand for competitive programming](https://github.com/qryxip/cargo-compete)
- [2020 Update · rust\-lang\-ja/atcoder\-rust\-resources Wiki](https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update)

# メモ
## バージョン 1.42.0で使えないもの
### BTreeSetのfirst

unstable

### `is_sorted()` : unstable

[is\_sorted \- The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/library-features/is-sorted.html)


## 2次元配列の初期化が紛らわしい。

```rust
let mut v = vec![vec![0;M];N];
```
良い感じのマクロ作ればいいのか？

## マップの初期化・更新
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
//略
*map.entry(key).or_insert(initialValue) += delta;
```

## BinaryHeapのiter()はpopの順ではない。

それに `iter()` はヒープを消費しない。

[BinaryHeap in std::collections \- Rust](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.iter)

ソート順のイテレータに変換数`into_iter_sorted`がnightlyにはある。

## jq
cargo-competeでエディタを開くために `jq` が要る。

インストール方法：
[bash \- How to run jq from gitbash in windows? \- Stack Overflow](https://stackoverflow.com/questions/53967693/how-to-run-jq-from-gitbash-in-windows)

## cargo-snippet

競プロ用スニペット管理。専用のクレートを作って、`cargo compete`を走らせるとスニペット形式に変換して出力してくれる。

[hatoo/cargo\-snippet: A snippet extrator for competitive programmers](https://github.com/hatoo/cargo-snippet)

[Rustで競技プログラミングをするときの"スニペット管理"をまじめに考える\(cargo\-snippetの紹介\) \- Qiita](https://qiita.com/hatoo@github/items/5c6814e72ddd2ecaf48f)

インストールするとき引っ掛かった。

```
[net]
git-fetch-with-cli = true
```

にするか、`CARGO_NET_GIT_FETCH_WITH_CLI`をtrueにすれば良い。

- [Configuration \- The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli)
- [\[Rust\] cargo install で文字化けして通信エラーが発生する │ Web備忘録](https://webbibouroku.com/Blog/Article/cargo-install-error)
- [Cargo build failed with spurious network error · Issue \#6513 · rust\-lang/cargo](https://github.com/rust-lang/cargo/issues/6513)

VSCodeでは`.vscode`フォルダ内に `.code-snippets` のサフィックスを付けてjsonを書いてやると、ワークスペースレベルのスニペットを設定できる。ここに吐くとよい。

[Snippets in Visual Studio Code](https://code.visualstudio.com/docs/editor/userdefinedsnippets)