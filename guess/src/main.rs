extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数字游戏");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("输入你要猜的数字:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("在输入时出现问题");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你输入了: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal   => {
                println!("恭喜你，你猜到了");
                break;
            }
        }
    }
}