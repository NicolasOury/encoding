pub use encoding_derive::*;

pub trait Encoding {
   fn encoding_size () -> usize;
}

impl Encoding for bool {
   fn encoding_size () -> usize {
       2
   }
}

impl Encoding for () {
    fn encoding_size() -> usize {
        0
    }
}

impl <T:Encoding> Encoding for &T {
    fn encoding_size () -> usize {
        <T as Encoding>::encoding_size()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
