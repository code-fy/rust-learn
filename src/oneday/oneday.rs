// src/oneday/oneday.rs

pub fn oneday() {
    println!("one day begin");
    for i in 1..=5 {
        println!("{}",i)
        
    }
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oneday() {
        oneday(); // 调用函数进行测试
    }
}
