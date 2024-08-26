#[derive(Debug)]
enum Shape{
    Rectangle,
    Triangle,
    Circle,
}

struct User{
    name:String,
    age:u32,
    student:bool
}

fn foo(User{
    name,
    age,
    student
}:User){
    println!("{}",name);
    println!("{}",age);
    println!("{}",student);
}

fn main() {
    let a = User{
        name:String::from("mike"),
        age:20,
        student:false,
    };
    foo(a);
    let shape_a = Shape::Triangle;
    match shape_a {
        Shape::Rectangle =>{
            println!("{:?}",Shape::Rectangle);
        }
        Shape::Triangle=>{
            println!("{:?}",Shape::Triangle);
        }
        Shape::Circle=>{
            println!("{:?}",Shape::Circle);
        }
    }
    println!("Hello, world!");
}
