pub fn hello_twoday(){
    print!("hello two day")
}

fn add_int_float(x:i32,y:f32) -> f32 {
    (x as f32) + y
}

fn copys(n:i32){
    let s1 =String::from("fuck");
    let s2 = s1.clone();
    println!("{}",n)
}

fn takes_ownership(some_string :String) {
    println!("{}",some_string)
}

fn borrow_fn(){
    let x = 5;
    let y = &x;
    let s = String::from("fuck");
    let s1 = &s;
    println!("{}",*s1);
    assert_eq!(s,*s1);
    assert_eq!(x,*y);
}

fn calculate_length(s: &mut String) {
    s.len();
    s.insert_str(1, " you")
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[cfg(test)]
mod test{
    use crate::twoday::twoday::{change, copys};

    use super::{add_int_float, borrow_fn, calculate_length, hello_twoday, takes_ownership};

    #[test]
    fn test_calculate_length(){
        let mut s = String::from("fuck");
        calculate_length(&mut s);
        println!("{}",s);
        change(&mut s);
        println!("{}",s);
    }

    #[test]
    fn test_borrow_fn(){
        borrow_fn();
    }

    #[test]
    fn test_takes_ownership(){
        let s = String::from("fuck");
        takes_ownership(s);
        // println!("{}",s)
        let n = 5;
        copys(n);
        println!("{}",n)
    }
    #[test]
    fn test_hello_twoday(){
        hello_twoday();
    }
    #[test]
    fn test_add_int_float(){
        let r = add_int_float(66, 0.66);
        println!("add_int_float result :{}",r)
    }

}
