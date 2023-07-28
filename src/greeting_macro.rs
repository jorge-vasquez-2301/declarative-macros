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
}
