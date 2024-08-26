use std::io; // prelude

fn main() {
    println!("Hello, world!");
    println!("菜蔬");
    print!("猜测一个数 \n");
    let foo = 1;
    let bar = foo;
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法获取");
    println!("your guess number:{}",guess);
}
