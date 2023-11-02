use std::io::{self, stdin};
use rand::Rng;
fn main(){
let mut rng=rand::thread_rng();
let n1:u8=rng.gen();
println!("Enter your guess: ");
let mut secret=String::new();
io::stdin()
.read_line(&mut secret)
.expect("Failed to Read");
let guess:u8= secret.trim().parse().expect("Invalid input");
if n1<guess{
    println!("You guessed High");
}
else if n1==guess {
    println!("You guessed right!!");
    
}
else{
    println!("You guessed low!!");
}

println!("The real number was: {:?}",n1);
}