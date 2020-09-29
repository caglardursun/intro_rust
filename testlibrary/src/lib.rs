
pub fn fibonacci(i:i32) -> Result<i32,String>
{

    // match i {
        
    // }

    // if i == 1 || i == 2
    // {
    //     Ok(1)
    // }
    // else{
    //     fibonacci(i-1) + fibonacci(i-2)
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(fibonacci(3),2);
    }
}
