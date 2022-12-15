use core::fmt;
use std::collections::HashMap;

// impl fmt::Display for Bytes32 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         for i in self.store {
//             write!(f, "{},", i)
//         }
//
//         Ok(())
//     }
// }
// impl core::cmp::Ord for Bytes32 {
//     fn cmp(&self, other: &Bytes32) -> std::cmp::Ordering {
//         for i in 0..3 {
//             if other.store[i] > self.store[i] {
//                 return std::cmp::Ordering::Greater;
//             }
//             if other.store[i] < self.store[i] {
//                 return std::cmp::Ordering::Less;
//             }
//         }
//
//         std::cmp::Ordering::Equal
//     }
// }
//

type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap<Bytes32, Bytes32>,
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //        for (k, x) in &self.kvstore {
        //            return write!(f, "{:?} : {:?}", k, x);
        //        }
        //        Ok(())
        write!(f, "{}FUCK", 9)
    }
}
