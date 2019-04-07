extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;  //head

fn main() {
    println!("Welcome to Guessing Game!");//welcome message
	let mut t=0;//reset counter
	println!("Please input your guess(1~100):");
	let secret_number = rand::thread_rng().gen_range(1,101);//random
	while t<11 {//control the rounds
		let mut guess = String::new();//input
		t = t+1;
		println!("Round {}", t);
		io::stdin().read_line(&mut guess)
			.expect("Fail to read line!");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,//check the input
		};
		println!("You guessed:{}", guess);
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You hit it!");//output the result
			break;//end this app
			}
		}
	}
}
