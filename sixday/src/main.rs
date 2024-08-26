#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

fn foo() -> Box<Point> {
    let p = Point {x: 10, y: 20};  // 这个结构体的实例创建在栈上
    Box::new(p)
}

fn foo1(p:Box<Point>) -> Box<Point>{
    p

}
fn main() {
    let _p = foo();
    println!("{:?}",_p);
    let _pp = foo1(_p);
    println!("{:?}",_pp);
}