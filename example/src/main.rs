use encoding::Encoding;


#[derive(Encoding)]
enum Foo {
    Toto( bool, bool),
    Tata {x : bool, y: bool},
    Titi
}

#[derive(Encoding)]
struct Boo { x : bool }

fn main() {
    let a = Foo::Tata{ x : true, y:false };
    println!("{} {:?} {:?}",Foo::encoding_size().to_string(),
       a.encode(), a.likelihood(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0])
    );
}
