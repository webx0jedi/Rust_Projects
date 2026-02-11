use std::thread;
use std::process;
use std::time::Duration;
use std::io;

fn main() {
    println!("This is a pizza program. Please enter your favorite pizza");
    let mut pizza = String::new();

    io::stdin().read_line(&mut pizza).expect("Failed to read line.");
    let pizza = pizza.trim();

    if pizza.eq_ignore_ascii_case("Pepperoni") {
	println!("Great choice!\n");
   } else if pizza.eq_ignore_ascii_case("Hawaiian") {
	println!("You are forever banned from this store.\n");
	std::process::exit(0);
    } else {
	println!("Ew. How dare you.");
    }
	//currently working as of here, next part
   thread::sleep(Duration::from_secs(1)); //sleeps for 2 secs
   println!("Now that we know what type of pizza you like, would you like to order some? (yes/no)");
   let mut answer = String::new();
   io::stdin().read_line(&mut answer)
	.expect("Please answer yes/no.");
   let answer_trimmed = answer.trim();

   if answer_trimmed == "yes" {
	println!("Amazing. Please enter a number.\n");
  } else {
       println!("Ok. Maybe another time...\n");
       std::process::exit(0);
   }
   let mut count = String::new();
   io::stdin().read_line(&mut count)
	.expect("Please give me a real answer.\n");
   println!("Great! You ordered {count} {pizza} pizzas! Have a good day.\n");
}
