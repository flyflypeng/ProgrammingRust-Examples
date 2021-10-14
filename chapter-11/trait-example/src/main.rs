use std::fs::File;
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello World\n")?;
    out.flush()
}

fn say_generic_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello World\n")?;
    out.flush()
}

fn main() {
    let mut local_file = File::create("hello.txt").expect("error");
    say_hello(&mut local_file).expect("say file hello error");

    let mut bytes = vec![];
    say_hello(&mut bytes).expect("say bytes hello error");
    assert_eq!(bytes, b"Hello World\n");

    let mut new_bytes = vec![];
    say_generic_hello(&mut new_bytes).expect("say bytes generic hello error");
    assert_eq!(new_bytes, b"Hello World\n");
}
