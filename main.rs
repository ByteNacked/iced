// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use iced::generate_storage_ty;

generate_storage_ty!(
    struct MyStorage {
        name : &str,
        calib : u32,
    }
);

fn main() {

}
