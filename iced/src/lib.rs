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
use core::slice::from_raw_parts_mut;
use crc::crc32::{Digest, IEEE_TABLE, IEEE, Hasher32};


pub enum Error {
    OutOfSpace,
    ValidateOutOfRange,
    CrcNotMatched,
    BadAlignment,
}

enum MemoryScan {
    EndOfMemory,
    CrcNotMatched,
}

pub struct InitStats {
    err_cnt : usize,
    unique_tags : usize,
}

pub const MAX_PAYLOAD_SIZE : usize = 0x100;

#[repr(C)]
pub struct Header {
    tag  : usize,
    sz   : usize,
    crc  : usize,
}

#[repr(C)]
union UHeader {
    header : Header,
    bytes: [u8; size_of::<Header>() * size_of::<usize>()],
}

#[derive(Copy, Clone)]
pub struct Record {
    pub tag : usize,
    pub ptr : Option<&'static Header>,
}

struct Storage {
    align : usize,
    ptr : usize,
    capacity : usize,
    size  : usize,
}

impl Storage {
    pub const fn new(start_addr : usize, align : usize, capacity : usize) -> Self {
        Self {
            align,
            ptr : start_addr,
            capacity,
            size : 0,
        }
    }

    pub fn init(&mut self, list : &mut [Record], crc : &mut impl Hasher32) -> InitStats {

        let mut stats = InitStats { err_cnt : 0, unique_tags : 0 };
        
        let mut idx = 0;
        let mut size = 0;
        while idx < self.capacity - size_of::<Header>() {
            match self.header_validate(idx, crc) {
                Ok(sz) => {
                    let header = self.header_from_idx(idx);
                    list[header.tag].ptr = Some(header);
                    idx += sz;
                    size += size;
                }
                Err(_) => {
                    idx += 1;
                }
            }
        }
        self.size = size;
        
        stats
    }
    
    pub fn update(&mut self, record : &mut Record, buf : &[u8], crc : &mut impl Hasher32) -> Result<(),()> {
        todo!()
    }

    pub fn get(&mut self, record : &Record) -> Result<&'static [u8],()> {
        todo!()
    }

    fn header_validate(&self, idx : usize, crc : &mut impl Hasher32) -> Result<usize,()> {
        let header = self.header_from_idx(idx);
        let payload_idx = idx + size_of::<Header>();
        
        // Check size in bounds
        let payload_idx_end = payload_idx + header.sz;
        if payload_idx_end > self.capacity {
            return Err(());
        }
        let total_sz = size_of::<Header>() + header.sz;

        // Calculate crc
        crc.reset();
        crc.write(self.slice_from_idx(idx, total_sz));
        let crc = crc.sum32() as usize;
        
        if crc != header.crc {
            return Err(());
        }

        Ok(total_sz)
    }
    
    fn is_slice_ffed(&self, slice : &[u8]) -> bool {
        for i in slice.iter() {
            if *i != !0 {
                return false;
            }
        }
        return true;
    }

    fn header_from_idx(&self, idx : usize) -> &'static Header {
        debug_assert!(idx % self.align == 0);
        debug_assert!(idx + size_of::<Header>() <= self.capacity);
        unsafe { transmute(self.ptr + idx) }
    }

    fn slice_from_idx(&self, idx : usize, sz : usize) -> &mut [u8] {
        debug_assert!(idx % self.align == 0);
        debug_assert!(idx + sz <= self.capacity);
        unsafe { from_raw_parts_mut( idx as *mut _ , sz) }
    }
}




///pub trait ListStorage {
///
///    type List;
///    type ListArray;
///
///    /// Scan and populate list of list saved lists
///    fn scan(&self, ll : &mut Self::ListArray) -> Result<(),()>;
///
///    /// Get first elemt of list
///    fn head(&self, list : &List) -> Result<&'static [u8],()>;
///
///    /// Get list with out first element
///    fn tail(&self, list : &List) -> Result<Option<List>, ()>;
///
///    /// Append to head of list
///    fn cons(&mut self, list : &List, buf : &[u8]) -> Result<List, ()>;
///}
///
///impl<T, const N: usize> DefaultStorage<T,N> {
///    
///    //TODO: Some kind of crc check
///    
///    fn read_meta(&self, ptr : usize) -> Result<&'static Meta,()> {
///        let raw_bytes = self.read(ptr, size_of::<Meta>())?;
///        let meta : &Meta = unsafe {
///            let ptr = raw_bytes.as_ptr();
///            // ATTENTION!
///            transmute(&*ptr)
///        };
///
///        Ok(meta)
///    }
///
///    fn read_payload(&self, ptr : usize, sz : usize) -> Result<&'static [u8], ()> {
///        self.read(ptr, sz)
///    }
///
///    fn read_meta_checked(&self, ptr : usize) -> Result<&'static Meta,()> {
///        self.read_meta(ptr)
///    }
///
///    fn mark_chunk_invalid(&mut self, ptr : usize, sz : usize) -> Result<(),()> {
///        self.fill(ptr, sz, 0x00)
///    }
///
///}
///
///impl<T, const N: usize>  ListStorage for DefaultStorage<T,N> {
///
///    type List = List;
///    type ListArray = [List; N];
///
///    /// Scan and populate list of list saved lists
///    fn scan(&self, ll : &mut Self::ListArray) -> Result<(),()> {
///        // Scan from zero to memory capacity or end early if 0xFFFFFF
///        let mut iter = 0;
///        // Find 
///        while iter < self.capacity() {
///            // TODO: some error handling
///            let meta = match self.read_meta_checked(iter) {
///                Ok(meta) => {
///                    debug_assert!(ll[meta.tag].tag == meta.tag);
///                    ll[meta.tag].ptr = iter;
///                    meta
///                }
///                Err(e) => return Err(e),
///            };
///            iter += size_of::<Meta>() + meta.sz;
///        }
///
///        todo!()
///    }
///
///    /// Get first elemt of list
///    fn head(&self, list : &List) -> Result<&'static [u8],()> {
///        let meta = self.read_meta(list.ptr)?;
///        self.read_payload(list.ptr + size_of::<Meta>(), meta.sz)
///    }
///
///    /// Get list without first element
///    fn tail(&self, list : &List) -> Result<Option<List>, ()> {
///        let meta = self.read_meta(list.ptr)?;
///
///        if let 0 = meta.next {
///            Ok(None)
///        } else {
///            Ok(Some(List {
///                tag : meta.tag,
///                ptr : meta.next,
///            }))
///        }
///    }
///
///    /// Append to head of list
///    fn cons(&mut self, list : &List, buf : &[u8]) -> Result<List, ()> {
///        let next = list.ptr;
///        let sz = buf.len();
///        let crc = 0xFACEFEED;
///        let meta : UMeta = UMeta { meta : Meta{ tag : list.tag, next, sz, crc} };
///        let meta_bytes  = unsafe { meta.bytes };
///        let ptr = self.write(&meta_bytes)?;
///        let _   = self.write(&buf)?;
///
///        Ok(List {
///            tag : list.tag,
///            ptr,
///        })
///    }
///}
///
///pub struct DefaultStorage<T, const N: usize> {
///    buf : [T; N],
///    sz : usize,
///}
///
///impl<T : Sized, const N: usize>  DefaultStorage<T,N> {
///    pub const fn new(array : [T;N]) -> Self {
///        Self {
///            buf : array,
///            sz : 0,
///        }
///    }
///
///    pub fn read(&self, ptr : usize, sz : usize) -> Result<&'static [u8],()> {
///        todo!()
///    }
///
///    pub fn write(&mut self, buf : &[u8]) -> Result<usize, ()> {
///        todo!()
///    }
///
///    fn fill(&mut self, ptr : usize, sz : usize, b : u8) -> Result<(),()> {
///        todo!()
///    }
///
///    pub fn len(&self) -> usize {
///        self.sz
///    }
///
///    pub fn capacity(&self) -> usize {
///        N
///    }
///
///}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_test() {
        //let array = DefaultStorage([0u8; 10]);
        //let crc = Digest::new(crc32::IEEE);
    }


    #[test]
    fn crc32_test() {
        let mut crc = Digest::new(IEEE);
        let b = [0xA5u8];
        crc.write(&b);
        let res : u32 = crc.sum32();
        assert_eq!(res, 0x74BEB8EA);
        
        crc.reset();
        let b = [0xA5,0xA5,0xA5,0xA5];
        crc.write(&b);
        let res : u32 = crc.sum32();
        assert_eq!(res, 0xF18EB66B);
    }

}













