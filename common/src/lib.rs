pub use color_eyre::Result;
pub use paste::paste;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| color_eyre::install().unwrap());
}

#[macro_export]
macro_rules! boilerplate {
    {
        $($name:ident => { $($input:ident$(($($p:expr),*))? -> $value:expr),* })*
    } => {
        fn main() {
            $crate::init();
            $($({
                let input = include_str!(concat!(stringify!($input), ".txt"));
                println!(concat!("Result of ", stringify!($name), ", ", stringify!($input), ": {}"), $name(input $(, $($p),*)?));
            })*)*
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::paste;

            $($(
                paste!{
                    #[test]
                    fn [<$name _ $input>]() {
                        $crate::init();
                        let input = include_str!(concat!(stringify!($input), ".txt"));
                        assert_eq!($name(input $(, $($p),*)?), $value);
                    }
                }
            )*)*

        }
    };
}
