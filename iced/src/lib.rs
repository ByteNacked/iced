#![feature(const_generics)]


use core::ops::Range;

pub enum MemoryError {
    OutOfSpace,
    OutOfRange,
    BadAlignment,
    CrcNotMatched,
}

pub trait Storage {
    type Error;
    //fn new(addr_range : Range<usize>) -> Self;
    fn write(&mut self, uid : usize, buf : &[u8]);
    fn read(&self, uid : usize, buf : &mut [u8]) -> usize;
}

pub trait ListStorage {
    /// Scan and populate list of list saved lists
    fn scan(&self, ll : &mut [usize]);
    // Get adress register
    fn car(&self, list : usize) -> usize;
    // Get data register
    fn cdr(&self, list : usize) -> &'static [u8];
    // Get tag register
    fn ctr(&self, list : usize) -> u16;
    // Get crc register
    fn ccr(&self, list : usize) -> u16;
    // Appned to head of list
    fn cons(&mut self, list : usize, tag : u16, crc : u16, data : &[u8]) -> Result<usize, ()>;
}

struct DefaultStorage {
    m : [u8;0x100],
}

impl ListStorage for DefaultStorage {
    fn scan(&self, ll : &mut [usize]) {
        todo!()
    }

    fn car(&self, list : usize) -> usize {
        todo!()
    }

    fn cdr(&self, list : usize) -> &'static [u8] {
        todo!()
    }

    fn ctr(&self, list : usize) -> u16 {
        todo!()
    }

    fn ccr(&self, list : usize) -> u16 {
        todo!()
    }

    fn cons(&mut self, list : usize, tag : u16, crc : u16, data : &[u8]) -> Result<usize, ()> {
        todo!()
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
