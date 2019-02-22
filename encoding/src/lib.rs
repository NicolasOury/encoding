pub use encoding_derive::*;

pub trait Encoding {
   fn encoding_size () -> usize;
   fn encode_into(&self, target: &mut [f64]);
   fn encode(&self) -> Vec<f64> {
       let mut v = vec![0.0; Self::encoding_size()];
       self.encode_into(v.as_mut_slice());
       return v;
   }
   fn likelihood(&self, source: &[f64]) -> f64;
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

   fn likelihood(&self, source : &[f64]) -> f64 {
       if *self { 
          source[0]
       } else {
           source[1]
       }
   }
}

impl Encoding for () {
    fn encoding_size() -> usize {
        0
    }
    fn encode_into(&self, _target: & mut [f64]) {

    }
    fn likelihood(&self, _source: &[f64]) -> f64 {
        1.0
    }
}

impl <T:Encoding> Encoding for &T {
    fn encoding_size () -> usize {
        <T as Encoding>::encoding_size()
    }
    fn encode_into(&self, target: & mut [f64]) {
        <T as Encoding>::encode_into(self, target);
    }

    fn likelihood(&self, source: &[f64]) -> f64 {
        <T as Encoding>::likelihood(self, source)
    }
}

impl <T:Encoding> Encoding for Option<T> {
    fn encoding_size () -> usize {
        <T as Encoding>::encoding_size() + 2
    }
    fn encode_into(&self, target: & mut [f64]) {
        match self  {
            None => target[0] = 1.0,
            Some(x) => {
                target[1] = 1.0;
                let target = &mut target[2..];
                x.encode_into(target);
            }
        }
    }

    fn likelihood(&self, target: & [f64] ) -> f64 {
        match self {
            None => target[0],
            Some(x) =>
               target[1] * { 
                   let target = & target[2..];
                   x.likelihood(target)
               }
        }
    }
}

array_impl!(0);
array_impl!(1);
array_impl!(2);
array_impl!(3);
array_impl!(4);
array_impl!(5);
array_impl!(6);
array_impl!(7);
array_impl!(8);
array_impl!(9);
array_impl!(10);
array_impl!(11);
array_impl!(12);
array_impl!(13);
array_impl!(14);
array_impl!(15);
array_impl!(16);
array_impl!(17);
array_impl!(18);
array_impl!(19);
array_impl!(20);
array_impl!(21);
array_impl!(22);
array_impl!(23);
array_impl!(24);
array_impl!(25);
array_impl!(26);
array_impl!(27);
array_impl!(28);
array_impl!(29);
array_impl!(30);
array_impl!(31);
array_impl!(32);
array_impl!(64);
array_impl!(128);
array_impl!(256);

impl <T1: Encoding, T2 : Encoding> Encoding for (T1, T2) {
    fn encoding_size () -> usize {
        T1::encoding_size() + T2::encoding_size()
    }

    fn encode_into(&self, target: &mut [f64]) {
        let (x1, x2) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        x1.encode_into(&mut target[0..t1_size]);
        x2.encode_into(&mut target[t1_size.. t1_size+t2_size]);
    }

    fn likelihood(&self, source: &[f64]) -> f64 {
        let (x1, x2) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        x1.likelihood(&source[0..t1_size])
        * x2.likelihood(&source[t1_size..t1_size+t2_size])
    }

}

impl <T1: Encoding, T2 : Encoding, T3: Encoding> Encoding for (T1, T2, T3) {
    fn encoding_size () -> usize {
        T1::encoding_size() + T2::encoding_size() + T3::encoding_size()
    }

    fn encode_into(&self, target: &mut [f64]) {
        let (x1, x2, x3) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        let t3_size = T3::encoding_size();
        x1.encode_into(&mut target[0..t1_size]);
        x2.encode_into(&mut target[t1_size.. t1_size+t2_size]);
        x3.encode_into(&mut target[t1_size+t2_size.. t1_size+t2_size+t3_size]);
    }

    fn likelihood(&self, source: &[f64]) -> f64 {
        let (x1, x2, x3) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        let t3_size = T3::encoding_size();
        x1.likelihood(&source[0..t1_size])
        * x2.likelihood(&source[t1_size..t1_size+t2_size])
        * x3.likelihood(&source[t1_size+t2_size..t1_size+t2_size+t3_size])
    }

}

impl <T1: Encoding, T2 : Encoding, T3: Encoding, T4: Encoding> Encoding for (T1, T2, T3, T4) {
    fn encoding_size () -> usize {
        T1::encoding_size() + T2::encoding_size() + T3::encoding_size() + T4::encoding_size()
    }

    fn encode_into(&self, target: &mut [f64]) {
        let (x1, x2, x3, x4) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        let t3_size = T3::encoding_size();
        let t4_size = T4::encoding_size();
        x1.encode_into(&mut target[0..t1_size]);
        x2.encode_into(&mut target[t1_size.. t1_size+t2_size]);
        x3.encode_into(&mut target[t1_size+t2_size.. t1_size+t2_size+t3_size]);
        x3.encode_into(&mut target[t1_size+t2_size+t3_size.. t1_size+t2_size+t3_size+t4_size]);
    }

    fn likelihood(&self, source: &[f64]) -> f64 {
        let (x1, x2, x3, x4) = self;
        let t1_size = T1::encoding_size();
        let t2_size = T2::encoding_size();
        let t3_size = T3::encoding_size();
        let t4_size = T4::encoding_size();
        x1.likelihood(&source[0..t1_size])
        * x2.likelihood(&source[t1_size..t1_size+t2_size])
        * x3.likelihood(&source[t1_size+t2_size..t1_size+t2_size+t3_size])
        * x4.likelihood(&source[t1_size+t2_size+t3_size..t1_size+t2_size+t3_size+t4_size])
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
