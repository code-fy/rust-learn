
trait Bird {
    fn quack(&self)->String;
    
}

struct Duck{}
impl Duck  {
    fn swim(&self){
        println!("look,the duck swim")
    }
    
}

struct Swan{}
impl Swan {
    fn fly(&self){
        println!("look swan fly")
    }
    
}

impl Bird for Duck {
    fn quack(&self)->String {
        String::from("duck duck")
    }
    
}

impl Bird for Swan {
    fn quack(&self)->String {
        String::from("swan swan")
    }
    
}

fn hatch_a_bird(i :i32) -> Box<dyn Bird>{
    if i == 2 {
        Box::new(Duck{})
        
    }else{
        Box::new(Swan{})
    }
}

trait Draw {
    fn draw(&self)->String;
    
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: Box<dyn Draw>) {
    x.draw();
}

trait Foo {
    fn method(&self)->String;
    
}

impl Foo for u8 {
    fn method(&self)->String {
        format!("u8 : {}",*self)
    }
    
}

impl Foo for String {
    fn method(&self)->String {
        format!("string{}",*self)
    }
    
}

fn static_dispatch<T:Foo>(t:T){
    
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(t:&dyn Foo){
    
}


#[cfg(test)]
mod test{
    use super::{draw_with_box, draw_with_ref, dynamic_dispatch, hatch_a_bird, static_dispatch, Bird, Draw, Duck, Swan};

    #[test]
    fn test_trait(){
        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8: {}", *self)
            }
        }
        
        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("f64: {}", *self)
            }
        }
        let x = 1.1f64;
        let y = 8u8;

        // draw x
        draw_with_box(Box::new(x));

    // draw y
        draw_with_ref(Box::new(y));

        println!("Success!");


        let birds:Vec<Box<dyn Bird>> = vec![
            Box::new(Duck{}),
            Box::new(Swan{})
        ];
        for bird in birds.iter(){
            println!("{}",bird.quack())
        }


        let duck = Duck{};
        duck.swim();
        let bird = hatch_a_bird(2);
        assert_eq!(bird.quack(),"duck duck");
        let bird = hatch_a_bird(1);
        assert_eq!(bird.quack(),"swan swan");
        println!("Success!")



    }
    #[test]
    fn test_static_dynamic(){
        let x = 5u8;
        let y = "Hello".to_string();
        static_dispatch(x);
        dynamic_dispatch(&y);

    }
}