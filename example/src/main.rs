use encoding::Encoding;

#[derive(Encoding)]
enum Toto<'a> {
    A {x : &'a bool},
    B {b: bool, c: bool},
    C (bool, bool)
}

fn main() {
    println!("{}",Toto::encoding_size().to_string());
}
