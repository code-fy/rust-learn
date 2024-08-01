fn main() {
    println!("Hello, world!");
    let a:u32 = 1;
    let first_c = String::from("fuck");
    println!("{}",first_c);
    let byte_escape = "I'm saying \"fuck\"";
    println!("{}",byte_escape);
    let byte_escape = "I'm saying \n 操！";
    println!("{}",byte_escape);
    let bytestring: &[u8; 21] = b"this is a byte string"; 
    println!("A byte string: {:?}", bytestring); 
    let escaped = b"\x52\x75\x73\x74 as bytes"; 
    println!("Some escaped bytes: {:?}", escaped); 
    let raw_bytestring = br"\u{211D} is not escaped here"; println!("{:?}", raw_bytestring);
}
