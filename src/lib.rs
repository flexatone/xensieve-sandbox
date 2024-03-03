
use xensieve::Sieve;

// mod util;
// mod parser;


// #[derive(Clone, Debug)]
// pub(crate) struct Residual {
//     modulus: u64,
//     shift: u64,
// }


// impl Residual {

//     pub(crate) fn new(modulus: u64, mut shift: u64) -> Self {
//         if modulus == 0 {
//             shift = 0;
//         } else {
//             shift %= modulus;
//         }
//         Self{modulus: modulus, shift: shift}
//     }
// }

pub fn sieve_to_str(value: String) -> String {
    Sieve::new(&value).to_string()
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let s = sieve_to_str("3@0".to_string());
        assert_eq!(s, "Sieve{3@0}");
    }
}

