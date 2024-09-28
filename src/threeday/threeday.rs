fn borrow_method(){
    println!("come on to fix it");
    let s = String::from("fuck");
    let s1 = &s;
    let s2 = &s;
    println!("{},{}",s1,s2);
    println!("{},{}",s1,s2); //其实是在这里结束了生命周期
    let mut s3 = &s;
    println!("{}", s3); //在这里结束了生命周期
}
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn nodangle() -> String {
    let s = String::from("hello");
    s
}

// 以上两个函数的比较，在所有权和生命周期上，dangle 返回了一个函数内部字符串的引用，内部字符串在函数执行完就已经被销毁掉，你在返回它的引用，就是空的
// nodangle 返回了内部函数的所有权给到调用者
// 引用的作用域从创建开始一直持续到他最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号

// 复合类型
#[allow(unused_variables)]
type File = String;
fn open(f: &mut File) -> bool {
    true
}
fn close(f:&mut File) -> bool{
    true
}
#[allow(dead_code)]
fn read(f:&mut File,save_to: &mut Vec<u8>) -> !{
    unimplemented!()
}


#[cfg(test)]
mod test{

    use super::{borrow_method, close, open, File,read};

    #[test]
    fn test_borrow_method(){
        borrow_method();
    }

    #[test]
    fn test_file_systeam(){
        let mut f = File::from("fuck");
        let op = open(&mut f);
        let op = close(&mut f);
        // read(&mut f,& mut vec![]);

    }
}
