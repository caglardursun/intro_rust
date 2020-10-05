///configuration conditional checks are possible through two different operators:
///
///The cfg attribute: #[cfg(...)] in attribute position
///The cfg! macro: cfg!(...) in boolean expressions
/// 
/// 
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

pub fn test_it()
{
    //Or call it from frunc. directlly. If ur platform is not linux 
    //are_you_on_linux doesn't compiled

    are_you_on_linux();
    
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}