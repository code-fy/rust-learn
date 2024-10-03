

struct Company{
    name:String,
    base:String,
}

struct Factory{
    name:String,
    base:String
}

struct Person{
    name:String,
    base:String,
    hight:i32,
    weight:i32,
    occupation:String,
    company:Company
}

pub trait Work {
    fn eat(&self)->String{
        String::from("民以食为天")
    }
    fn work(&self)->f32;
    fn play(&self)->String;   
}

impl Work for Person {

    fn work(&self)->f32 {
        10.0*3.0
    }

    fn play(&self)->String {
        String::from("good time!")
    }
    
}

fn go_work(w :&dyn Work) {
    w.eat();
    w.play();
    println!("continue study")
}


fn largest<T:PartialOrd>(list:&[T]) -> &T {

    let mut largest = &list[0];
    for i in list.iter() {
        if largest < &i {
            largest = &i;
            
        }
        
    }
    largest

}

pub trait Draw{
    fn draw(&self);
}

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,
}

impl Draw for Button {
    fn draw(&self){
        println!("impl Button")

    }
}

struct SelectBox{
    width:u32,
    height:u32,
    options:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        println!("impl SelectBox")
        
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();

        }
    }
    
}

pub struct Screen1<T:Draw>{
    pub components:Vec<T>

}

impl<T> Screen1<T> 
    where T: Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();

        }

    }
}
    


#[cfg(test)]
mod test{
    use std::fs::Permissions;

    use super::{go_work, largest, Button, Company, Factory, Person, Screen, SelectBox};

    #[test]
    fn test_replay(){
        let webeye = Company{name:String::from("webeye"),base:String::from("nanjing")};
        let jingdong = Company{name:String::from("jingdong"),base:String::from("beijing")};
        let tom = Person{name:String::from("tom"),base:String::from("nanjing"),hight:170,weight:65,company:webeye,occupation:String::from("dazade")};
        let xiaohong = Person{name:String::from("tom"),base:String::from("nanjing"),hight:170,weight:65,company:jingdong,occupation:String::from("dazade")};
        go_work(&tom);
        go_work(&xiaohong);

    }

    #[test]
    fn test_pair_sort(){
        let l=vec![1,5,8,2,4,0];
        let i = largest(&l);
        println!("{}",i)
    }

    #[test]
    fn test_trait_object(){


        let screen = Screen{
            components:vec![
                Box::new(SelectBox{
                    width:75,
                    height:10,
                    options:vec![
                        String::from("yes"),
                        String::from("")
                    ]
                }),
                Box::new(Button{
                    width:50,
                    height:10,
                    label:String::from("ok"),
                })
            ]
        };
        screen.run();

    }
    

    
    

}

