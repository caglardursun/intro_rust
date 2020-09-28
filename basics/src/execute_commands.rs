use std::process::Command;

pub fn list_directory(path:&str)
{
    let _c = Command::new("ls")
    .arg("-l")
    .arg("/home/caglar")
    .output()
    .expect("ls is not usable");

    let result = String::from_utf8(_c.stdout).expect("Not UTF8 shit!");

    println!("{}",result);
}