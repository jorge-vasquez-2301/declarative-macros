pub fn greeting(name: &str, greeting: &str) -> String {
    format!("{greeting}, {name}!")
}

#[macro_export]
macro_rules! greeting {
    ($name:literal) => {
        greeting($name, "Hello")
    };
    ($name:literal,$greeting:literal) => {
        greeting($name, $greeting)
    };
    (test $name:literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returning default greeting");
        greeting($name, "Hello")
    }};
}
