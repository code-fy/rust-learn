pub fn hello_twoday(){
    print!("hello two day")
}

fn add_int_float(x:i32,y:f32) -> f32 {
    (x as f32) + y
}

#[cfg(test)]
mod test{
    use super::{add_int_float, hello_twoday};

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
