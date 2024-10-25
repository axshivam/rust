use rand::Rng;
use std::io;

fn main() {
    let options = ["banana", "orange", "apple", "papaya", "grapes", "pea"];

    loop {
        let random_number = rand::thread_rng().gen_range(0..6);
        println!("Game choices fruit: {}", options[random_number]);

        println!("Please enter a guess a fruit name from {:?}", options);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

        let fruit_name = guess.trim().to_lowercase().to_string();
        println!("Selected Fruit:  {fruit_name}");

        if fruit_name.as_str() == options[random_number] {
            println!("You won");
            break;
        } else {
            println!("try again");
        }
    }
}
