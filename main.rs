// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run


use crc::crc32::{Digest,IEEE};
use crc::CalcType;

use iced_macros::generate_storage_ty;

#[derive(Debug)]
pub enum Mode {
    InAir,
    Lifting,
    Landing,
    OnGround,
}

generate_storage_ty! {
    struct PerMap {
        name : u32,
        calib : u32,
        calib2 : u16,
        sign : u8,
        num : u8,
        cara : u8,
        flag : bool,
        barray : [bool;5],
        mode : Mode,
    }
}

fn new_params_from_array(storage_mem : &mut [u32] ) -> (usize, usize) {
    let start_addr = storage_mem.as_mut_ptr() as usize;
    let capacity = core::mem::size_of::<u32>() * storage_mem.len();

    (start_addr, capacity)
}

fn crc32_ethernet() -> impl StorageHasher32 {
    Digest::new_custom(IEEE, !0u32, 0u32, CalcType::Normal)
}


fn main() {

    let mut storage_mem = [!0u32;0x1000];
    let (start_addr, capacity) = new_params_from_array(&mut storage_mem);

    let mut storage = PerMap::new(start_addr, capacity);
    let mut crc = crc32_ethernet();
    
    storage.set_name(7u32, &mut crc).unwrap();
    storage.set_name(6u32, &mut crc).unwrap();
    storage.set_name(3u32, &mut crc).unwrap();
    storage.set_name(1u32, &mut crc).unwrap();
    storage.set_calib(777u32, &mut crc).unwrap();
    storage.set_cara(42u8, &mut crc).unwrap();
    storage.set_cara(42u8, &mut crc).unwrap();
    storage.set_cara(42u8, &mut crc).unwrap();
    storage.set_flag(true, &mut crc).unwrap();
    storage.set_flag(false, &mut crc).unwrap();
    storage.set_barray([false, true, false, true, true], &mut crc).unwrap();
    storage.set_barray([false; 5], &mut crc).unwrap();
    storage.set_mode(Mode::Lifting, &mut crc).unwrap();
    storage.set_mode(Mode::InAir, &mut crc).unwrap();
    
    println!("{:?}", &storage);
}
