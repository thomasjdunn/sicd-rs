use sicd_rs::{read_unknown_sicd, SicdVersion};
use std::path::Path;
fn main() {
    let sicd_file = Path::new(file!()).join("../../../example.nitf").canonicalize().unwrap();
    let sicd = read_unknown_sicd(&sicd_file);
    print!("{:?}", sicd.version);
    
    
    
}
