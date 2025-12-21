use rand::Rng;
use std::{cmp::Ordering, io};

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess)
// }

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..101);

    // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

    // Until "you win" is outputted.
    loop {
        println!("Please input your guess.");

        //　Variable to store user input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Process invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // guess
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            // Ordering::Equal => println!("You win!"),
            // Finish after making the correct prediction.
            Ordering::Equal => {
                println!("You win!");
                break; // returnじゃなく、breakが適切らしい。loopは終わるけど後続の処理が必要な場合、break
            }
        }
    }
}
