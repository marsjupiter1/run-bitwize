#![allow(dead_code)]

#[derive(Debug)]
pub struct BVec{
    bits_per_value: u8,
    capacity: usize,
    values: Vec<u8>,
}

pub fn new() -> BVec{

    with_capacity(16)
}

pub fn with_capacity(capacity:usize)-> BVec{

    let v = Vec::with_capacity(capacity);
    BVec{
        bits_per_value: 2,
        capacity: capacity,
        values: v,
    }

}
