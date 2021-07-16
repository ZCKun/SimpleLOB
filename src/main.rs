pub mod mdt_struct;
pub mod dat_reader;
pub mod data_process;
// pub mod sql;
pub mod observer;

use std::error::Error;

use crate::dat_reader::Dat;
use crate::data_process::DataProcess;


fn main() -> Result<(), Box<dyn Error>> {
    let start = chrono::offset::Local::now();
    let filepath = "D:/BaiduNetdiskdownload/dATA/dat/202107130705.dat";
    // let filepath = "/home/x2h1z/data.dat";

    let dp = DataProcess::new();
    let mut dat = Dat::new(filepath, Box::new(dp));
    dat.read();

    let end = chrono::offset::Local::now();
    let consuming = end - start;
    println!("写入完成, 耗时:{}s", consuming.num_seconds());

    Ok(())
}
