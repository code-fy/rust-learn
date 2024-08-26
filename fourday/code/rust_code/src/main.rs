fn main(){
    let p1 = Point{x:1,y:1};
    let p2 = Point{x:2,y:2};
    let p3 = p1.add(p2);
    assert_eq!(p3.x,3);
    assert_eq!(p3.y,3);

    let p1 = Point{x:1,y:1};
    let delta = 2;
    let p3 = p1.add(delta);
    assert_eq!(p3.x,3);
    assert_eq!(p3.y,3);
}



fn iter_test(){
    let mut a = [1,2,3];
    let mut an_iter = a.iter();
    assert_eq!(Some(&1),an_iter.next());
    assert_eq!(Some(&2),an_iter.next());
    assert_eq!(Some(&3),an_iter.next());
    assert_eq!(None,an_iter.next());

    let mut an_iter = a.iter_mut();
    assert_eq!(Some(&mut 1),an_iter.next());
    assert_eq!(Some(&mut 2),an_iter.next());
    assert_eq!(Some(&mut 3),an_iter.next());
    assert_eq!(None,an_iter.next());

    let mut an_iter = a.into_iter();
    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None,an_iter.next());
    println!("{:?}",a);

}

fn iter2_test() {
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];    // 一个整数数组

    let mut an_iter = a.iter();  // 转换成第一种迭代器
    
    assert_eq!(Some(&"1".to_string()), an_iter.next());
    assert_eq!(Some(&"2".to_string()), an_iter.next());
    assert_eq!(Some(&"3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();  // 转换成第二种迭代器
    
    assert_eq!(Some(&mut "1".to_string()), an_iter.next());
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter();  // 转换成第三种迭代器，并消耗掉a
    
    assert_eq!(Some("1".to_string()), an_iter.next());
    assert_eq!(Some("2".to_string()), an_iter.next());
    assert_eq!(Some("3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // println!("{:?}", a);
}

fn test3_iter(){
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];

    for item in &a{
        println!("{}",item);
    }
    for item in &mut a{
        println!("{}",item);
    }
    for item in a{
        println!("{}",item);
    }

    // println1("{?}",a)
}

fn owner_test(){
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");
    let v = vec![s1,s2,s3,s4];
    let a = &v[0];
    println!("{}",a);
}

fn owner_test2(){
    let s1 = 1;
    let s2 = 2;
    let s3 = 3;
    let s4 = 4;
    let v = vec![s1,s2,s3,s4];
    let a = v[0];
    println!("{}",a);
}

fn owner_test3(){
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");
    let v = vec![s1,s2,s3,s4];

    for item in &v{
        println!("{}",item);
    }
    println!("{:?}",v);
}

// struct Point<T>{
//     x:T,
//     y:T,
// }

// struct Foo;

// fn print<T:std::fmt::Display>(p:Point<T>){
//     println!("Point {},{}", p.x,p.y);
// }

trait Add<T>{
    type Output;
    fn add(self,rhs:T) -> Self::Output;
}


struct Point {
    
    x:i32,
    y:i32,
}

impl Add<Point> for Point{
    type Output = Self;
    fn add(self,rhs:Point) -> Self::Output{
        Point{
            x:self.x + rhs.x,
            y:self.y+rhs.y,
        }
    }

}


impl Add<i32> for Point{
    type Output = Self;
    fn add(self,rhs:i32) -> Self::Output {
        Point {
            x:self.x+rhs,
            y:self.y+rhs,
        }
    }
}