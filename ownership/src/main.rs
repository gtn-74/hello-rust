// fn main() {
// let mut s = String::from("hell");
// s.push_str(", world");
// println!("{}", s);

// let s1 = String::from("foo");
// let mut s2 = s1;
// let s2 = s1;
// s2 = String::from("foo");
// println!("{}", s2);

// let x = 5; // xがスコープに入る

// let s4 = gives_ownership();

// let s3 = takes_and_give_back(String::from("hello"));

// let (s2, len) = calclate_length(s4);

//     let mut s = String::from("rust");
//     change(&mut s);

//     // makes_copy(x); // xも関数にムーブされるが、
//     // i32はCopyなので、この後にxを使っても
//     // 大丈夫
// } // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

// fn makes_copy(some_integer: i32) {
//     // some_integerがスコープに入る
//     println!("{}", some_integer);
// } // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_give_back(a_string: String) -> String {
//     a_string
// }

// fn calclate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn main() {
// let mut s = String::from("rust");
// let res = change(&mut s);
// println!("{}", res);

// 可変な参照には大きな制約が一つあります:
// 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てないことです。こちらのコードは失敗します:
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;
// println!("{}{}", r1, r2)

fn main() {
    // スライス型：
    // コレクション：
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear();
    println!("{}", word);
}

// responseがusizeからスライス型に変わったことによりコンパイルが通らないようになった。
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// &str        // 文字列スライス
// &[i32]      // 配列スライス
// &[T]        // 一般的なスライス
// ```

// ---

// ## 要約
// ```
// スライス型 = &str

// 特徴:
// - 文字列の一部への参照
// - 元の文字列と繋がっている
// - だからコンパイラが守ってくれる
