// mod print;
// This declaration will look for a file named 'strings.rs' or 'strings/mod.rs' and
// will insert its content inside a module named 'strings' under this scope
mod strings;
// single line comments
/*
 multi line comments
 */
/// <p>html comments</p>

fn main() {
    // let x = 5 + 5;
    // println!("Is `x` 10 or 100? x = {}", x)
    strings::print();
}
