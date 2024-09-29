struct Rectangle{
    weith :i32,
    height:i32

}

impl Rectangle {

    fn new(x:i32,y:i32)->Self{
        Rectangle{
            weith:x,
            height:y
        }

    }

    fn area(&self) -> i32{
        self.height * self.weith
    }    
}


#[cfg(test)]
mod test {
    use super::Rectangle;

    #[test]
    fn test_method(){
        let r = Rectangle::new(5, 6);
        let areas = r.area();
        println!("area is {}",areas)

    }
}