#![allow(dead_code, unused_imports)]

use core::mem::size_of;
use core::slice::{from_raw_parts_mut, from_raw_parts};

// TODO: implement errors and error tests
// TODO: validity check on fn get

// Minimal addressing unit (and aligment)
pub type Word = u32;
// Header len in words
const HEADER_LEN : usize = size_of::<Header>() / size_of::<Word>();
// Word size in bytes
const WORD_SIZE : usize = size_of::<Word>();

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub struct Header {
    tag  : Word,
    /// Size of payload in words
    sz   : Word,
    crc  : u32,
}

#[derive(Debug)]
pub enum Error {
    OutOfFreeSpace,
    CorruptedRecordOnGet,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct RecordDesc {
    pub tag : Word,
    pub ptr : Option<&'static Header>,
}

#[derive(Debug)]
pub struct InitStats {
    words_wasted : usize,
    unique_tags  : usize,
}

pub struct Storage {
    storage : &'static mut [Word],
    current : usize,
}

impl Storage {

    pub fn new(start_addr : usize, capacity : usize) -> Self {
        assert!(start_addr % WORD_SIZE == 0);
        assert!(capacity % WORD_SIZE == 0);

        let storage = unsafe { from_raw_parts_mut(start_addr as *mut _, capacity / WORD_SIZE) };

        Self {
            storage,
            current : 0,
        }
    }
    
    /// Scan through storage memory and populate record descriptor table
    pub fn init(&mut self, list : &mut [RecordDesc], hasher : &mut impl StorageHasher32) -> InitStats {

        let mut stats = InitStats { words_wasted : 0, unique_tags : 0 };
        
        let mut idx = 0;
        let mut size = 0;
        let mut last_valid_end = 0;
        let capacity = self.storage.len();
        
        // Scanning through whole storage to find all valid records
        while idx < capacity - HEADER_LEN {
            let res = self.validate_record(idx, hasher);
            match res {
                Some(header) => {
                    assert_eq!(list[header.tag as usize].tag, header.tag, "Index in table should match tag!");
                    list[header.tag as usize].ptr = Some(header);
                    idx += HEADER_LEN + header.sz as usize;
                    last_valid_end = idx;
                }
                None => {
                    idx += 1;
                }
            }
        }
        
        // Scannig from last record end position, to determine that
        // rest flash memory wasn't already written (NOT 0xFF'ed)
        for idx in last_valid_end .. capacity {
            if !Self::is_ffed(self.storage[idx]) {
                size = idx + 1;
                stats.words_wasted += 1;
            }
        }

        self.current = size;

        // Stats
        for e in list {
            if let Some(_) = &e.ptr {
                stats.unique_tags += 1;
            }
        }
        
        stats
    }

    fn validate_record(&self, idx : usize, hasher : &mut impl StorageHasher32) -> Option<&'static Header> {
        let _tag = self.storage[idx];
        let len = self.storage[idx + 1];
        let crc = self.storage[idx + 2];

        let payload_start_idx = idx + 3;
        let payload_end_idx = payload_start_idx + len as usize;
        // Check payload slice is not out of bounds
        if payload_end_idx > self.storage.len() {
            return None;
        }
        
        // Calculate checksum
        hasher.reset();
        let header_part = &self.storage[idx .. idx + 2];
        hasher.write(header_part);
        let payload_slice = &self.storage[payload_start_idx .. payload_end_idx];
        hasher.write(payload_slice);
        
        // Compare checksums
        let calc_crc = hasher.sum();
        if crc != calc_crc {
            return None;
        }
        
        let header : &Header = unsafe { &*(self.storage[idx .. ].as_ptr() as *const _) };
        Some(header)
    }
    
    /// Update recordy entry
    pub fn update(&mut self, record : &mut RecordDesc, payload : &[Word], hasher : &mut impl StorageHasher32) -> Result<(),Error> {
        let record_len = HEADER_LEN + payload.len();
        if self.free_space_in_words() < record_len {
            return Err(Error::OutOfFreeSpace);
        }

        let record_slice = &mut self.storage[self.current .. self.current + record_len];
        let (header_slice, payload_slice) = record_slice.split_at_mut(HEADER_LEN);

        // Fill header
        header_slice[0] = record.tag;
        header_slice[1] = payload.len() as Word;

        // Copy payload
        payload_slice.copy_from_slice(payload);
        
        // Calculate and set checksum
        hasher.reset();
        hasher.write(&header_slice[0 .. 2]);
        hasher.write(payload_slice);
        let checksum = hasher.sum();
        header_slice[2] = checksum as Word;

        // Update record descriptor
        let updated_header : &Header = unsafe { &*(header_slice.as_ptr() as *const Header) };
        record.ptr = Some(updated_header);

        // Update current len
        self.current += record_len;

        Ok(())
    }
    
    /// Get record payload
    pub fn get(&mut self, record : &RecordDesc) -> Result<Option<&'static [u32]>,Error> {
        match record.ptr {
            Some(header) => {
                // Basic sanity check
                if header.tag == record.tag {
                    unsafe {
                        let header_ptr = header as *const _ as *const u32;
                        let payload_ptr = header_ptr.offset(HEADER_LEN as isize);
                        Ok(Some(from_raw_parts(payload_ptr, header.sz as usize)))
                    }
                } else {
                    Err(Error::CorruptedRecordOnGet)
                }
            },
            None => Ok(None),
        }
    }

    /// Total amount of occupied storage space in bytes
    pub fn len(&self) -> usize {
        self.current * WORD_SIZE
    }
    /// Total storage space in bytes
    pub fn capacity(&self) -> usize {
        self.storage.len() * WORD_SIZE
    }

    fn free_space_in_words(&self) -> usize {
        self.storage.len() - self.current
    }

    fn is_ffed(word : Word) -> bool {
        if word == !0 {
            return true;
        }
        return false;
    }

    fn header_from_slice(&self, _slice : &'static [u32]) -> &'static Header {
        todo!()
    }

    fn payload_from_header_slice(&self, _header : &'static Header) -> Result<&'static [u32],()> {
        todo!()
    }
}

pub trait StorageHasher32 {
    fn reset(&mut self);
    fn write(&mut self, words: &[u32]);
    fn sum(&self) -> u32;
}

#[allow(dead_code, unused_imports)]
#[cfg(test)]
mod tests {

    use super::*;
    use crc::crc32::{Digest, IEEE_TABLE, IEEE, Hasher32};
    use crc::CalcType;

    impl StorageHasher32 for Digest {
        fn reset(&mut self) {
            <Digest as Hasher32>::reset(self);
        }

        fn write(&mut self, words: &[u32]) {
            let bytes = unsafe { 
                from_raw_parts(words.as_ptr() as *const u8, words.len() * WORD_SIZE) 
            };
            <Digest as Hasher32>::write(self, bytes);
        }

        fn sum(&self) -> u32 {
            <Digest as Hasher32>::sum32(self)
        }
    }

    fn crc32_ethernet() -> impl StorageHasher32 {
        Digest::new_custom(IEEE, !0u32, 0u32, CalcType::Normal)
    }

    fn new_params_from_array(storage_mem : &mut [u32] ) -> (usize, usize) {
        let start_addr = storage_mem.as_mut_ptr() as usize;
        let capacity = WORD_SIZE * storage_mem.len();

        (start_addr, capacity)
    }

    fn new_storage(storage_mem : &mut [u32] ) -> Storage {
        let start_addr = storage_mem.as_mut_ptr() as usize;
        let capacity = WORD_SIZE * storage_mem.len();

        Storage::new(start_addr, capacity)
    }

    #[test]
    fn empty_test() {
        let mut storage_mem = [!0u32;0x100];
        let (start_addr, capacity) = new_params_from_array(&mut storage_mem[..]);
        let storage = Storage::new(start_addr, capacity);

        assert_eq!(storage.len(), 0);
        assert_eq!(storage.capacity(), capacity);
    }
    
    #[test]
    fn new_record_test() {
        let mut storage_mem = [!0u32;0x100];
        let mut storage = new_storage(&mut storage_mem[..]);
        let mut rec_desc = RecordDesc {
            tag : 1,
            ptr : None,
        };

        let rec_payload = [42u32;1];
        let mut crc32 = crc32_ethernet();
        
        storage.update(&mut rec_desc, &rec_payload, &mut crc32).unwrap();
        assert_eq!(storage.len(), (HEADER_LEN + rec_payload.len()) * WORD_SIZE );
        assert!(&rec_desc.ptr.is_some());
        
        let out_rec_payload = storage.get(&rec_desc).unwrap().unwrap();
        assert_eq!(&rec_payload, out_rec_payload);


        let mut desc_list = [
            RecordDesc {
                tag : 0,
                ptr : None,
            },
            RecordDesc {
                tag : 1,
                ptr : None,
            },
        ];
        let _stats = storage.init(&mut desc_list, &mut crc32);
        assert_eq!(&desc_list[1], &rec_desc);
        //println!("Desc list : {:#?}", &desc_list);
    }

    #[test]
    fn series_of_records_test() {
        let mut storage_mem = [!0u32;0x100];
        let mut storage = new_storage(&mut storage_mem[..]);
        let mut crc32 = crc32_ethernet();

        let mut desc_list = [
            RecordDesc {
                tag : 0,
                ptr : None,
            },
            RecordDesc {
                tag : 1,
                ptr : None,
            },
            RecordDesc {
                tag : 2,
                ptr : None,
            },
        ];

        let e0 = [!42u32; 10];
        storage.update(&mut desc_list[0], &e0, &mut crc32).unwrap();

        let e1 = [0x7777_7777; 3];
        storage.update(&mut desc_list[1], &e1, &mut crc32).unwrap();

        let e0 = [0x6666_6666; 3];
        storage.update(&mut desc_list[0], &e0, &mut crc32).unwrap();

        let e2 = [0x5555_5555; 3];
        storage.update(&mut desc_list[2], &e2, &mut crc32).unwrap();

        let e0 = [0xA5B5A5A5u32; 2];
        storage.update(&mut desc_list[0], &e0, &mut crc32).unwrap();

        let e1 = [66u32; 5];
        storage.update(&mut desc_list[1], &e1, &mut crc32).unwrap();
        
        storage.init(&mut desc_list, &mut crc32);
        assert_eq!(storage.get(&desc_list[0]).unwrap().unwrap(), &e0);
        assert_eq!(storage.get(&desc_list[1]).unwrap().unwrap(), &e1);
        assert_eq!(storage.get(&desc_list[2]).unwrap().unwrap(), &e2);
        
        //println!("Desc list : {:#?}", &desc_list);
    }

    //#[test]
    //fn crc32_test() {
    //    let mut crc = Digest::new_custom(IEEE, !0u32, 0u32, CalcType::Normal);

    //    crc.reset();
    //    let b = [0xA5u8];
    //    crc.write(&b);
    //    let res : u32 = crc.sum32();
    //    println!("\n{:x}\n", &res);
    //    assert_eq!(res, 0xA8E282D1);

    //    crc.reset();
    //    let b = [0xA5u8, 0];
    //    crc.write(&b);
    //    let res : u32 = crc.sum32();
    //    println!("\n{:x}\n", &res);
    //    assert_eq!(res, 0xA8E282D1);
    //    
    //    crc.reset();
    //    let b = [0xA5,0xA5,0xA5,0xA5];
    //    crc.write(&b);
    //    let res : u32 = crc.sum32();
    //    assert_eq!(res, 0x29928E70);
    //}

}
