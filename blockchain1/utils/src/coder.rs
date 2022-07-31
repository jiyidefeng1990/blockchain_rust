use serde::{Serialize, Deserialize};
use bincode;

// pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>> where
//     T: Serialize, 
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8> 
    where T: Serialize {
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

// pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T> where
//     T: Deserialize<'a>, 

pub fn my_deserialize<'a,T>(bytes: &'a [u8]) -> T 
    where T: Deserialize<'a>{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

#[derive(Serialize, Deserialize,Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn coder_works() {
        use super::Point;
        use super::{my_deserialize,my_serialize};
        let point = Point{x: 0, y: 1};
        let se = my_serialize(&point);
        let de:Point = my_deserialize(&se);
        assert_eq!(de, point);
    }
}
