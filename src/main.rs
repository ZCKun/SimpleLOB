use dat_reader::Reader;

mod dat_reader;

fn main() {
    let mut reader = Reader::new("E:/2019/DATA/dat/202104270705.dat");
    reader.read();
}
