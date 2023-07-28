macro_rules! newtype {
    ($struct_type:ident, |$name:ident: &$internal_type:ty| $predicate:expr) => {
        struct $struct_type {
            value: $internal_type,
        }

        impl $struct_type {
            pub fn get_value(&self) -> &$internal_type {
                &self.value
            }
        }

        impl TryFrom<$internal_type> for $struct_type {
            type Error = &'static str;

            fn try_from(value: $internal_type) -> Result<Self, Self::Error> {
                let f = |$name: &$internal_type| $predicate;
                if f(&value) {
                    Ok(Self { value })
                } else {
                    Err("Predicate failed")
                }
            }
        }
    };
}

newtype!(FirstName, |str: &String| !str.is_empty());
newtype!(LastName, |str: &String| !str.is_empty());
newtype!(Age, |v: &i32| v > &0);
newtype!(Pay, |v: &i32| v > &0);

fn main() {
    todo!()
}
