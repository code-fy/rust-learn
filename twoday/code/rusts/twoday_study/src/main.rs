struct User {
    active:bool,
    username:String,
    email:String,
    price:u64,
}

struct Origin {
    or_id:u32,
    or_rank:u32,
    members : Vec<User>,
}

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut a: u32 = 10;
    let b = &mut a;
    *b = 30;
    // a = 20;
    
    // println!("{}", a);

    let s:String = "I am a badman.".to_string();
    let a_slice : &str = &s[0..5];
    let another_String = a_slice.to_string();
    // println!("{}",another_String);


    let s = String::from("如斯塔你好");
    let char_vec : Vec<char> = s.chars().collect();
    // println!("{:?}", char_vec);

    for ch in s.chars(){
        // println!("{:?}", ch);
    }
    let active = true; 
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user1 = User { active, username, email, price: 1, };

    let email = user1.email;
    // println!("{:?}", user1)
    // println!("{:?}", user1) // 这一句无法通过编译
    let rect1 = Rectangle{
        width:30,
        height:50,
    };
    println!("The area of the rectangle is {} square pixels.",rect1.area());

}