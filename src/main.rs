use std::io::Read;

fn read_version(txn_hx: &str)-> u32{

    let txn_bytes =  hex::decode(txn_hx).unwrap();
    let mut byte_slice  = txn_bytes.as_slice();
    let mut buff = [0; 4];

    byte_slice.read(&mut buff).unwrap();

    u32::from_le_bytes(buff)

}
fn take_ownership_sum(values: Vec<i32>)-> i32{
    let mut sum = 0;

    for v in values {
         sum += v;
    }
    sum
}

fn main() {
    let version =  read_version("0100000001abcdef");
    let vec_num = [1,23,45,5,6,13,26,78,235,3456,76].to_vec();

    let sum = take_ownership_sum(vec_num);

    println!("Sum = {}", sum);
    println!("Version {}", version);
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
   fn sum_3_len_vec(){
     let arr = [1,2,3].to_vec();
     let sum = take_ownership_sum(arr);

     assert_eq!(sum, 6);
   }

    #[test]
    fn reads_version_1() {
        let hx = "01000000";
        let v = read_version(hx);
        assert_eq!(v, 1);
    }

    #[test]
fn version_is_little_endian() {
    let hx = "02000000";
    assert_eq!(read_version(hx), 2);
}
#[test]
#[should_panic]
fn invalid_hex_panics() {
    read_version("zzzz");
}

#[test]
fn multiple_versions() {
    let cases = [
        ("01000000", 1),
        ("02000000", 2),
        ("7f000000", 127),
    ];

    for (hx, expected) in cases {
        assert_eq!(read_version(hx), expected);
    }
}

}