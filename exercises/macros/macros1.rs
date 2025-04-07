// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
/*
error[E0423]: expected function, found macro `my_macro`
  --> exercises/macros/macros1.rs:15:5
   |
15 |     my_macro();
   |     ^^^^^^^^ not a function
   |
help: use `!` to invoke the macro
   |
15 |     my_macro!();
   |             +

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0423`.
hint
When you call a macro, you need to add something special compared to a
regular function call. If you're stuck, take a look at what's inside
`my_macro`.
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
