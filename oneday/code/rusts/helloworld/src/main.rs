fn main() {
    // println!("Hello, world!");
    // let a:u32 = 1;
    // let first_c = String::from("fuck");
    // println!("{}",first_c);
    // let byte_escape = "I'm saying \"fuck\"";
    // println!("{}",byte_escape);
    // let byte_escape = "I'm saying \n 操！";
    // println!("{}",byte_escape);
    // let bytestring: &[u8; 21] = b"this is a byte string"; 
    // println!("A byte string: {:?}", bytestring); 
    // let escaped = b"\x52\x75\x73\x74 as bytes"; 
    // println!("Some escaped bytes: {:?}", escaped); 
    // let raw_bytestring = br"\u{211D} is not escaped here"; println!("{:?}", raw_bytestring);
    let v:Vec<i32> = Vec::new();
    let v=vec![1,2,3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}",v[2]);
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is :{element}");
    }
}
