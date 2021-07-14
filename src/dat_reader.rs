use core::slice;
use std::{array, fs::File, io::Read, mem, slice::from_raw_parts_mut};

#[repr(packed(1))]
pub struct Header {
    total_len: i16,
    data_type: i32,
    data_len: i16
}

#[repr(packed(8))]
pub struct SZSEL2_Status {
    mdt_time: u64,
    symbol: [char; 40],
    symbol_source: [char; 5],
    time: i64,
    financial_status: [char; 8],
    crd_buy_status: char,
    crd_sell_status: char,
    subscribe_status: char,
    redemption_status: char,
    purchasing_staus: char,
    stock_divi_status: char,
    putable_status: char,
    exercise_status: char,
    gold_purchase: char,
    gold_redemption: char,
    accepted_status: char,
    release_status: char,
    canc_stock_divi_status: char,
    pledge_status: char,
    remove_pledge: char,
    vote_status: char,
    stock_pledge_repo: char,
    divide_status: char,
    merger_status: char
}


pub struct Reader {
    file: File
}

impl Reader {
    pub fn new(fp: &str) -> Reader {
        Self {
            file: File::open(fp).expect(
                format!("read file {} failed", fp).as_str()
            )
        }
    }

    fn read_head(&mut self, head: &mut Header, head_size: usize) {
        unsafe {
            let head_slice = slice::from_raw_parts_mut(head as *mut _ as *mut u8, head_size);
            self.file.read_exact(head_slice).unwrap();
        }
    }

    pub fn read(&mut self) {
        let mut head: Header = unsafe { mem::zeroed() };
        let head_size = mem::size_of::<Header>();

        self.read_head(&mut head, head_size);
        println!("total_size:{}\ndata_type:{}\ndata_size:{}", {head.total_len}, {head.data_type}, {head.data_len});

        let mut status: SZSEL2_Status = unsafe {mem::zeroed()};
        // let status_size = mem::size_of::<Header>();
        unsafe {
            let data_slice = slice::from_raw_parts_mut(&mut status as *mut _ as *mut u8, head.data_len as usize);
            self.file.read_exact(data_slice).unwrap();
        }

        // let s: String = status.symbol.iter().collect();
        let s: String = status.symbol.iter().collect();
        println!("symbol:{}, time:{}", s.as_str(), status.mdt_time);

    }
}

