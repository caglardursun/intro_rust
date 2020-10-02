/// Fibonacci number calculator for the given index
/// ```
///use testlibrary::*;
/// 
///    fn main(){
///         let number = fibonacci(6).unwrap();
///         //or
///         let result = match fibonacci(-1)
///         {
///             Ok(value)=> value,
///             Err(e)=> 0,
///         };
///     
///    }
/// 
/// 
/// ```
/// 
pub fn fibonacci(i:i32) -> Result<i32,String>
{
    if i <= 0 
    {
        return Err("Cannot be negative".to_string());
    };

    let result =  match i {
        1 => Ok(1),
        2 => Ok(1),
        _ =>  Ok(fibonacci(i-1).unwrap() + fibonacci(i-2).unwrap()),     
    };

    
    result
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(fibonacci(3),Ok(2));
        assert_eq!(fibonacci(-1),Err("Cannot be negative".to_string()));
    }
}
