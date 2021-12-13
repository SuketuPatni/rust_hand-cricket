use std::io;
use rand::Rng;
mod user;
mod system;
// mod tie;

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

const ODDEVE_TUP:[&str;2] = ["even", "odd"];

fn random() -> u32 {
    rand::thread_rng().gen_range(0..7)
}

fn random_bool() -> bool {
    rand::thread_rng().gen_bool(0.5)
}

fn main() {
    let _ = input("Choose heads or tails: ");

    let mut _winner = String::new();

    if random_bool() {
        // user wins toss
        _winner = input("Choose odd or eve: ");
    } else {
        // user loses toss
        let temp = random();
        _winner = String::from(ODDEVE_TUP[(temp % 2) as usize]);
        println!("I choose {}",ODDEVE_TUP[
            ((temp + 1)%2) as usize
        ]);
        
    }
    let sys_oe = random();
    let user_oe = input_num("Enter your odd-eve throw: ");
    let parity = (user_oe + sys_oe)%2;
    if ODDEVE_TUP[parity as usize] == &_winner {
        // user wins odd eve draw
        let choice = input("You won the odd-eve draw.\nChoose to bat or bowl: ");
        if choice == "bat" {
            user::batting();
        } else {
            system::batting();
        }
    }
}
