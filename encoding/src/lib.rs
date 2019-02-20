pub use encoding_derive::*;

pub trait Encoding {
   fn encoding_size () -> usize;
   fn encode_into(&self, target: &mut [f64]);
   fn encode(&self) -> Vec<f64> {
       let mut v = vec![0.0; Self::encoding_size()];
       self.encode_into(v.as_mut_slice());
       return v;
   }

}

impl Encoding for bool {
   fn encoding_size () -> usize {
       2
   }

   fn encode_into(&self, target: &mut [f64]) {
       if *self {
           target[0] = 1.0;
       }
       else {
           target[1] = 1.0;
       }
   }
}

impl Encoding for () {
    fn encoding_size() -> usize {
        0
    }
    fn encode_into(&self, _target: & mut [f64]) {

    }
}

impl <T:Encoding> Encoding for &T {
    fn encoding_size () -> usize {
        <T as Encoding>::encoding_size()
    }
    fn encode_into(&self, target: & mut [f64]) {
        <T as Encoding>::encode_into(self, target);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
