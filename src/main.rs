mod oneday;
mod twoday;
mod threeday;
mod fourday;
mod fiveday;
mod sixday;
mod sevenday;
mod eightday;

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "腻害";
    let english = "hello";
    let regions =[southern_germany,chinese,english];
    for r in regions.iter() {
        print!("{}",&r);
    }
}

fn add_two(){
    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;
    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a + b ) + ( c + d ) = {}", e);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以省略return
    i + j
}
struct Struct{
    e:i32
}

fn main() {
    oneday::oneday::oneday();
    twoday::twoday::hello_twoday();
    let f = 5;
    let f = f+1;
    let u=90;
    {
        let f = f*2;
        let u = 0;
        println!("the value of f is {}",f);
    }
    println!("the value of f is {}",f);
    println!("the value of u is {}",u);
    let (a,b,c,d,e);
    (a,b) = (1,2);
    [..,c,d,_,_] = [1,2,3,4,5,6];
    println!("{},{}",c,d);
    Struct {e,..} =  Struct{e:5};
    assert_eq!([1,2,3,4,5],[a,b,c,d,e]);
    let mut x = 5;
    println!("{:?}",[a,b,c,d,e]);
    println!("The value of x is :{}",x);
    x = 6;
    println!("The value of x is :{}",x);
    greet_world();
    add_two();
    println!("begin learn rust");
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
   let records = penguin_data.lines();
   for (i,record) in records.enumerate() {
       if i == 0 || record.trim().len() == 0 {
           continue;
       }
       let fields:Vec<_> = record.split(",").map(|field| field.trim()).collect();
    if cfg!(debug_assertions){
        eprintln!("debug:{:?} -> {:?}",record,fields);
    }
    let name = fields[0];
    if let Ok(length) = fields[1].parse::<f32>(){
        println!("{},{}cm",name,length);
    }
   }
}
