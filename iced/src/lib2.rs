


pub enum Records {
    Name,
    Calib,
    Hello,
}



struct Record<T> {
    idx : usize,
    val : T,

}

use core::ops::Range;

pub enum MemoryError {
    OutOfSpace,
    OutOfRange,
    BadAlignment,
    CrcNotMatched,
}

pub trait Memory {
    type Error;
    fn new(addr_range : Range<usize>) -> Self;
    fn write(&mut self, offset : usize, buf : &[u8]) -> Result<usize, Self::Error>;
    fn read(&self, offset : usize) -> Result<&[u8], Self::Error>;
}

pub struct Storage<M> {
    name  : Record,
    calib : Record,
    hello : Record,
    wr    : usize,
    mem   : M,
}

// generate_storage!(
//  name : &str,
//  calib : u32,
//  hello : u8,
// )

// [ptr][tag][bytes][crc]
impl<M : Memory> Storage<M> {
    pub fn name(&self) -> Result<&[u8],M::Error> {
        self.mem.read(self.name.idx)
    }

    pub fn set_name(&mut self, value : &[u8]) -> Result<(), M::Error> {
        self.mem.write(self.name.idx, value)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
