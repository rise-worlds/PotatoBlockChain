
#[cfg(test)]
mod tests {
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3, add_one(2));
    }
}