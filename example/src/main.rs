use encoding::Encoding;

#[derive(Encoding)]
struct Toto (bool, bool);

fn main() {
    let a = Toto (false, true);
    println!("{} {:?}",Toto::encoding_size().to_string(),
       a.encode()
    );
}
