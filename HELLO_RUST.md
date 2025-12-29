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

## 所有権が Move しない型(Copy 型)

- 数値（i32, f64 など）
- 真偽値（bool）
- 文字（char）

## ダングリングポインタ

```rs
let dog = Dog {
    name: "pochi".to_string(),
};
let cat = Cat {
    name: "tama".to_string(),
};

println!("{}{}", dog.name(), dog.speak());

!これがlogに吐けない。
println!("{:#?}", cat);
}
fn change(some_string: &mut String) {
    // mutだから不変。そのため変更できない。
    some_string.push_str(", world");
}

trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

// 猫
#[derive(Debug)] // !これがないと
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
    fn name(&self) -> &str {
        &self.name
    }
}
```
