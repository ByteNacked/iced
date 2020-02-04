#![feature(
    associated_type_bounds,
    const_fn,
    const_fn_union,
    const_generics,
    const_mut_refs,
    const_transmute,
    maybe_uninit_extra,
    maybe_uninit_ref,
    maybe_uninit_slice_assume_init,
    track_caller,
    untagged_unions
)]


use core::ops::Range;
use core::mem::transmute;
use core::mem::size_of;
pub enum MemoryError {
    OutOfSpace,
    OutOfRange,
    BadAlignment,
    CrcNotMatched,
}



#[derive(Copy, Clone)]
pub struct List {
    pub tag : usize,
    pub ptr : usize,
}

pub trait ListStorage {

    type List;
    type ListArray;

    /// Scan and populate list of list saved lists
    fn scan(&self, ll : &mut Self::ListArray) -> Result<(),()>;

    /// Get first elemt of list
    fn head(&self, list : &List) -> Result<&'static [u8],()>;

    /// Get list with out first element
    fn tail(&self, list : &List) -> Option<List>;

    /// Append to head of list
    fn cons(&mut self, list : &List, buf : &[u8]) -> Result<List, ()>;
}

#[repr(C)]
struct Meta {
    tag  : usize,
    next : usize,
    sz   : usize,
    crc  : usize,
}

#[repr(C)]
union UMeta {
    meta : Meta,
    bytes: [u8; 4 * size_of::<usize>()],
}

impl<T, const N: usize>  ListStorage for DefaultStorage<T,N> {

    type List = List;
    type ListArray = [List; N];

    /// Scan and populate list of list saved lists
    fn scan(&self, ll : &mut Self::ListArray) -> Result<(),()> {
        todo!()
    }

    /// Get first elemt of list
    fn head(&self, list : &List) -> Result<&'static [u8],()> {
        //self.read(list.ptr)
        todo!()
    }

    /// Get list without first element
    fn tail(&self, list : &List) -> Option<List> {
        let raw_bytes = self.read(list.ptr).unwrap();
        let meta : &Meta = unsafe {
            let ptr = raw_bytes.as_ptr();
            // ATTENTION!
            transmute(&*ptr)
        };
        
        todo!()
    }

    /// Append to head of list
    fn cons(&mut self, list : &List, buf : &[u8]) -> Result<List, ()> {
        let next = list.ptr;
        let sz = buf.len();
        let crc = 0;
        let meta : UMeta = UMeta { meta : Meta{ tag : list.tag, next, sz, crc} };
        let meta_bytes  = unsafe { meta.bytes };
        let ptr = self.write(&meta_bytes)?;
        let _   = self.write(&buf)?;

        Ok(List {
            tag : list.tag,
            ptr,
        })
    }
}

pub struct DefaultStorage<T, const N: usize> {
    buf : [T; N],
    sz : usize,
}

impl<T : Sized, const N: usize>  DefaultStorage<T,N> {
    pub const fn new(array : [T;N]) -> Self {
        Self {
            buf : array,
            sz : 0,
        }
    }

    pub fn read(&self, ptr : usize) -> Result<&'static [u8],()> {
        todo!()
    }

    pub fn write(&mut self, buf : &[u8]) -> Result<usize, ()> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.sz
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        let array = DefaultStorage([0u8; 10]);
    }
}
