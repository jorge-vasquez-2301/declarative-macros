#![feature(trace_macros)]
#![feature(log_syntax)]

use declarative_macros::greeting;
use declarative_macros::greeting_macro::greeting;

fn main() {
    trace_macros!(true);
    let _greet = greeting!("Sam", "Heya");
    let _greet_with_default = greeting!("Sam");
    let _greet_with_default = greeting!(test "Sam");
    trace_macros!(false);
}
