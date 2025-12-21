# cargo-rust
## ビルドと実行を同時にしてくれる

```zsh
cargo run
```

## コードがコンパイルできるかを確認。実行ファイルの生成はしない。

```zsh
cargo run
```

## リリースするなら

```zsh
cargo build --release
```

```toml
[dependencies]
rand = "0.9.2"

```

##　自分で書く

- use クレート名::モジュール名::アイテム名;
- 文末の`;`。


## create new project

```zsh
cargo new project-name
```

## run project

```zsh
cargo run
```

## sorce code compile check

```zsh
cargo check
```
