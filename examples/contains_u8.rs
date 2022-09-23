use bitstr::BitStr;

fn main() {
    let str_bitstr: &BitStr = BitStr::from(b"Rio");
    str_bitstr.contains_u8(4);
}
