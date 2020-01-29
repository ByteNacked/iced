// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use iced_macros::generate_storage_ty;
use iced::Storage;
use core::ops::Range;


struct MyStorage;

impl MyStorage {

    fn new(_addr_range : Range<usize>) -> Self { 
        Self {}
    }
}

impl Storage for MyStorage {

    type Error = ();

    fn write(&mut self, uid : usize, _buf : &[u8]) {
        println!("Write! uid: {}", uid);
    }
    fn read(&self, uid : usize, _buf : &mut [u8]) -> usize {
        println!("Read! uid: {}", uid);
        0
    }
}

generate_storage_ty! {
    struct PerMap {
        name : u32,
        calib : u32,
        calib2 : u16,
    }
}


fn main() {
    let mut my_map = PerMap::new(MyStorage::new(0..2));
    
    my_map.set_calib(7);
    my_map.get_calib();
}
