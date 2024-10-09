


#[cfg(test)]
mod test{
    #[test]
    fn test_vec(){
        let v = vec![1,2,3,4,5,6];
        let thirds = &v[2];
        match v.get(2) {
            Some(t) => println!("{:?}",t),
            None=>println!("a")
            
        }

    }

}