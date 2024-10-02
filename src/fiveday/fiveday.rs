pub trait road_bike {
    fn have_money(&self)->bool;
    fn deng(&self) -> String;
    
}

struct Student {
    money:i32,
    name : String,
    age : i32,
    height : f32,
    weight : f32
}

struct Boss{
    money:i32,
    name: String,
    age:i32,
    campaign:String,
    girlfriend:String
}

impl Boss {
    fn new(money:i32,name:String,age:i32,campaign:String,girlfriend:String) -> Boss{
        Boss{
            money,
            name,
            age,
            campaign,
            girlfriend
        }

    }
}

impl road_bike for Student {
    fn have_money(&self)->bool {
        if self.money >= 799{
            true
        }else {
            false
        }
    }    
    fn deng(&self) -> String {
        String::from("ao ao kuai") 
    }

}

impl road_bike for Boss {
    fn have_money(&self)->bool {
        if self.money >= 799{
            true
        }else {
            false
        }
    }  
    fn deng(&self) -> String {
        String::from("deng with my girlfriend aoao kuai")
    }
    
}

fn fitting <T: road_bike> (t : &T) -> String {

    t.deng()

}

#[cfg(test)]
mod test {
    use crate::fiveday::fiveday::road_bike;

    use super::{fitting, Boss, Student};

    #[test]

    fn test_fitting(){
        let s = Student{money:900,name:String::from("tom"),age:28,height:170.0,weight:65.0};
        let b: Boss = Boss::new(90,String::from("zhu"),30, String::from("webeye"),String::from("xiao li"));
        if b.have_money(){
            let res = fitting(&b);
            println!("nayouyigehaorena ：：：{:?}", res)

        }else{
            println!("zhuangnimane zaizhe zhaung")
        }
        
    }
}