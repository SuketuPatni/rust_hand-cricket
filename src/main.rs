use std::io;
use rand::Rng;
use std::collections::HashMap;

fn input_num(prompt:&str) -> u32 {
    let mut str = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut str).expect("Failed to read line");
    str.trim().parse::<u32>().unwrap()
}

fn input(prompt:&str) -> String {
    let mut str = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut str).expect("Failed to read line");
    String::from(str.trim())
}

const ODDEVE_TUP:[&str;2] = ["odd","even"];

fn random() -> u32 {
    rand::thread_rng().gen_range(0..7)
}

fn main() {
    // let mut a = rand::thread_rng().gen_range(0..7);
    // let a = String::new();
    let a = input("Choose heads or tails: ");
    println!("{}", a);
    let toss = rand::thread_rng().gen_bool(0.5);
    if toss {
        // user wins toss
        // odd or eve
        let odd_eve = input("Choose odd or eve: ");
        let sys_oe:u32 = random();
        let user_oe = input_num("Enter your odd-eve throw: ");
        let parity = (user_oe + sys_oe)%2;
        if ODDEVE_TUP[parity as usize] == odd_eve {

        }
    }
}
