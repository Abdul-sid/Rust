extern crate rand;
use std::io;
use rand::Rng;

fn main() {

println!("N : Normal mode");

 let mut mode = String::new();
io::stdin().read_line(&mut mode).expect("Failed to read line");

        let game_mode = String::from(mode.trim());

if game_mode == "N" || game_mode == "n" { 

loop{

	println!("1 - Rock, 2 - Paper, 3 - Scissors");

	let mut players_choice = String::new();
	let mut players_choice_int = 0;
	io::stdin().read_line(&mut players_choice).expect("Failed to read the players_choice");
	let p = String::from(players_choice.trim());
	

	if p == "1" {
		players_choice_int = 1;
    	println!("You chose Rock");
	} else if p == "2" {
		players_choice_int = 2;
    	println!("You chose Paper");
	} else if p == "3"{
		players_choice_int = 3;
    	println!("You chose Scissors");
	} else {
		println!("Error!");
	}

	let comp_choice = rand::thread_rng().gen_range(1, 4);

	match comp_choice {
		1 => println!("Computer chose Rock"),
		2 => println!("Computer chose Paper"),
		3 => println!("Computer chose Scissors"),
		_ => println!("Error!"),
	}

	let compare = (players_choice_int, comp_choice);

	match compare {
		(1, 1) => println!("It's a tie!"),
		(1, 2) => println!("You lose!"),
		(1, 3) => println!("You win!"),
		(2, 1) => println!("You win!"),
		(2, 2) => println!("It's a tie!"),
		(2, 3) => println!("You lose!"),
		(3, 1) => println!("You lose!"),
		(3, 2) => println!("You win!"),
		(3, 3) => println!("It's a tie!"),
		_ => println!("Error!"),
	}

    println!("do you wanna repeat it ? . Y for yes and N for no");

     let mut alphabet = String::new();

    io::stdin().read_line(&mut alphabet).expect("Failed to read line");

        let inpalpha = String::from(alphabet.trim());


    if inpalpha == "Y" || inpalpha == "y" {
        continue;
    }
    else if inpalpha == "N" {
        break;
    }
	else if inpalpha == "n" {
        break;
    }


}

}
}