use declarative_macros::greeting;
use declarative_macros::greeting_macro::greeting;

fn main() {
    let greet = greeting!("Sam", "Heya");
    println!("{greet}");
    let greet_with_default = greeting!("Sam");
    println!("{greet_with_default}");
}
