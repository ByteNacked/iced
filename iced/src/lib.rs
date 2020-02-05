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

enum MemoryScan {
    MetaV(&'static Meta),
    EndOfMemory,
    CrcNotMatched,
}

pub const MAX_PAYLOAD_SIZE : usize = 0x100;

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
    fn tail(&self, list : &List) -> Result<Option<List>, ()>;

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

impl<T, const N: usize> DefaultStorage<T,N> {
    
    //TODO: Some kind of crc check
    
    fn read_meta(&self, ptr : usize) -> Result<&'static Meta,()> {
        let raw_bytes = self.read(ptr, size_of::<Meta>())?;
        let meta : &Meta = unsafe {
            let ptr = raw_bytes.as_ptr();
            // ATTENTION!
            transmute(&*ptr)
        };

        Ok(meta)
    }

    fn read_payload(&self, ptr : usize, sz : usize) -> Result<&'static [u8], ()> {
        self.read(ptr, sz)
    }

    fn read_meta_checked(&self, ptr : usize) -> Result<&'static Meta,()> {
        self.read_meta(ptr)
    }

    fn mark_chunk_invalid(&mut self, ptr : usize, sz : usize) -> Result<(),()> {
        self.fill(ptr, sz, 0x00)
    }

}

impl<T, const N: usize>  ListStorage for DefaultStorage<T,N> {

    type List = List;
    type ListArray = [List; N];

    /// Scan and populate list of list saved lists
    fn scan(&self, ll : &mut Self::ListArray) -> Result<(),()> {
        // Scan from zero to memory capacity or end early if 0xFFFFFF
        let mut iter = 0;
        // Find 
        while iter < self.capacity() {
            // TODO: some error handling
            let meta = match self.read_meta_checked(iter) {
                Ok(meta) => {
                    debug_assert!(ll[meta.tag].tag == meta.tag);
                    ll[meta.tag].ptr = iter;
                    meta
                }
                Err(e) => return Err(e),
            };
            iter += size_of::<Meta>() + meta.sz;
        }

        todo!()
    }

    /// Get first elemt of list
    fn head(&self, list : &List) -> Result<&'static [u8],()> {
        let meta = self.read_meta(list.ptr)?;
        self.read_payload(list.ptr + size_of::<Meta>(), meta.sz)
    }

    /// Get list without first element
    fn tail(&self, list : &List) -> Result<Option<List>, ()> {
        let meta = self.read_meta(list.ptr)?;

        if let 0 = meta.next {
            Ok(None)
        } else {
            Ok(Some(List {
                tag : meta.tag,
                ptr : meta.next,
            }))
        }
    }

    /// Append to head of list
    fn cons(&mut self, list : &List, buf : &[u8]) -> Result<List, ()> {
        let next = list.ptr;
        let sz = buf.len();
        let crc = 0xFACEFEED;
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

    pub fn read(&self, ptr : usize, sz : usize) -> Result<&'static [u8],()> {
        todo!()
    }

    pub fn write(&mut self, buf : &[u8]) -> Result<usize, ()> {
        todo!()
    }

    fn fill(&mut self, ptr : usize, sz : usize, b : u8) -> Result<(),()> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.sz
    }

    pub fn capacity(&self) -> usize {
        N
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
