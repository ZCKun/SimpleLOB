use std::io::{Read, BufReader, BufRead};
use chrono::{DateTime, NaiveDateTime, Duration, Utc};
use std::fs::{OpenOptions, File};
use byteorder::{LittleEndian, ByteOrder};

use crate::{mdt_struct::SZSEL2_Quotation, observer::{Observer, Observerable}};

/// dat 头结构
#[repr(align(1))]
pub struct Header {
    // 2 bytes
    total_len: u16,
    // 4 bytes
    r#type: u32,
    // 2 bytes
    data_len: u16,
} // 8 bytes

impl Header {
    fn load<T: Read>(reader: &mut T) -> Header {
        Header {
            total_len: LittleEndian::read_u16(&Dat::read_part(reader, 2)),
            r#type: LittleEndian::read_u32(&Dat::read_part(reader, 4)),
            data_len: LittleEndian::read_u16(&Dat::read_part(reader, 2)),
        }
    }
}

pub struct Item {
    pub(crate) data_len: u16,
    pub(crate) data_type: u32,
    pub(crate) data: Vec<u8>,
}

pub struct Dat {
    buffer_reader: BufReader<File>,
    obj: Box<dyn Observer>,
}

impl Dat {

    ///
    /// # Arguments
    ///
    /// `filepath` - dat 文件路径
    /// `callback` - callback function
    pub fn new(filepath: &str, callback: Box<dyn Observer>) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .open(filepath)
            .expect("Unable to open file");

        let buffer_reader = BufReader::new(file);

        Self {
            buffer_reader,
            obj: callback
        }
    }

    pub fn read(&mut self) {
        while self.buffer_reader.fill_buf().unwrap().len() > 0 {
            let h = Header::load(&mut self.buffer_reader);
            let buf = Dat::read_part(&mut self.buffer_reader, h.data_len as i32);
            let item = Item {
                data_len: h.data_len,
                data_type: h.r#type,
                data: buf,
            };

            //self.obj.send(item)
        }
    }

    /// 从reader读取size大小的数据
    ///
    /// # Arguments
    ///
    /// `reader` - BufReader
    /// `size` - The byte size of data read
    fn read_part<T: Read>(reader: &mut T, size: i32) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size as usize);
        let mut part_reader = reader.take(size as u64);
        part_reader.read_to_end(&mut buf).unwrap();
        buf
    }

    /// timestamp 转换为 datetime
    ///
    /// # Arguments
    ///
    /// `timestamp` - 待转换的时间戳
    pub fn ts_to_datetime(timestamp: i64) -> DateTime<Utc> {
        let ts_nsec = timestamp % 1_000_000 * 1000;
        let naive = NaiveDateTime::from_timestamp(timestamp / 1_000_000, ts_nsec as u32);
        let datetime = DateTime::<Utc>::from_utc(naive, Utc);
        datetime + Duration::hours(8) // 东八区
    }

    fn on_item(item: Item) {
        todo!()
    }
}

impl Observerable for Dat {
    fn register_observer(&mut self, obj: Box<dyn Observer>) {
        todo!()
    }

    fn remove_observer(&mut self, index: usize) {
        todo!()
    }

    fn notify_observer(&self) {
        todo!()
    }
}