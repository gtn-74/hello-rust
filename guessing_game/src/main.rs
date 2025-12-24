// use rand::Rng;
// use std::{cmp::Ordering, io};

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess)
// }

// fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::rng().random_range(1..101);

//     // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

//     // Until "you win" is outputted.
//     loop {
//         println!("Please input your guess.");

//         //　Variable to store user input
//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         // let guess: u32 = guess.trim().parse().expect("Please type a number!");
//         // Process invalid input
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         // guess
//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big!"),
//             // Ordering::Equal => println!("You win!"),
//             // Finish after making the correct prediction.
//             Ordering::Equal => {
//                 println!("You win!");
//                 break; // returnじゃなく、breakが適切らしい。loopは終わるけど後続の処理が必要な場合、break
//             }
//         }
//     }
// }

// fn main() {
//     // add
//     let sum = 5 + 10;
//     println!("sum:{}", sum);

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("{}", difference);

//     // multiplication
//     let product = 4 * 30;
//     println!("{}", product);

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("{}", quotient);

//     let floored = 2 / 3;
//     println!("{}", floored);

//     // remainder
//     let remainder = 43 % 5;
//     println!("{}", remainder);

//     // bool
//     let t = true;
//     println!("{}", t);

//     let f: bool = false;
//     println!("{}", f);

//     // タプル型
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     println!("{:?}", tup); // {:?}は、デバッグ用トレイル

//     let tup = (500, 6.4, 1);
//     let (_x, y, _z) = tup;
//     println!("{}", y);

//     let x = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;

//     println!("{}", five_hundred);
//     println!("{}", six_point_four);
//     println!("{}", one);

//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//     println!("{:?}", months[2]);

//     let a = [1, 2, 3, 4, 5];
//     println!("{:?}", a);

//     let a = [3; 5];
//     println!("{:?}", a);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // faile read.

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("{}{}", index, element);

    // 低レベル言語の多くでは、 この種のチェックは行われないため、間違った添え字を与えると、無効なメモリにアクセスできてしまいます。
}
