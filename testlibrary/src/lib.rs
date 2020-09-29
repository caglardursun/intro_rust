
pub fn fibonacci(i:i32) -> i32
{
    if i == 1 || i == 2
    {
        1
    }
    else{
        fibonacci(i-1) + fibonacci(i-2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(fibonacci(3),2);
    }
}
