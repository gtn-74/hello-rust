use std::io;

fn main() {
    println!("Please input value!");
    let mut x = String::new();

    // !入力値を読み取る記述が足りない.
    // io::stdin().read_line(&mut x).expect("msg");

    // let foo = hoge(x);
    // let foo = hoge(&x);
    let foo = hoge(&mut x);

    // let number = 3;
    if foo < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let a = [1, 2, 3, 4, 5];
    // let mut index = 0;
    // while index < a {
    //     println!("{}", a[index]);

    //     index += 1;
    // }
    for element in a.iter().rev() {
        println!("{}", element);
    }
    // loop {
    //     println!("hogehogehoge");
    // }
}

// 基本的な違い
// break: ループを完全に終了する
// continue: 現在の反復をスキップして次の反復へ進む
// ループの反復: 1 → 2 → 3 → 4 → 5

// break at 3:
// 1 → 2 → 3 [終了]
//           ↓
//         ループを抜ける

// continue at 3:
// 1 → 2 → 3 [スキップ] → 4 → 5
//           ↓
//         次の反復へ

// この書き方は参照ではなく所有権を渡してるどうせ返却するなら参照だけで良い
// fn hoge(value: String) -> i32 {
// io::stdin().read_line(&mut x).expect("msg");

// !参照してるだけ
fn hoge(value: &mut String) -> i32 {
    io::stdin().read_line(value).expect("msg");
    value.trim().parse().expect("msg")
}

// &strは読み取り専用
// io::stdin().read_line(value).expect("msg");は使えない。

// &mut Stringは変更できる。
// 借用

// 引数にStringのみ。これが所有権の譲渡
// fn hoge(value: String) -> i32 {
// println!("{}", value); // !所有権はここで破棄される
// }
