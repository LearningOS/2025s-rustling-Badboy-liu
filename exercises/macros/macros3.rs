// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

#[macro_export]
macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
mod macros {
    // 确保宏在这里定义，并且使用了 #[macro_export]
    pub use super::my_macro;
}

fn main() {
    macros::my_macro!();
}
