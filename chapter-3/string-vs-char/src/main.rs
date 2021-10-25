use std::mem;

fn main() {
    let one_str = "h";

    println!("len of c is: {}", mem::size_of::<char>());
    println!(
        "len of string only contains one character: {}",
        one_str.len() * mem::size_of::<u8>()
    );
}
