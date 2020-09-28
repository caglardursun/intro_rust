use std::process::{Command,Stdio};
use std::io::copy;

pub fn list_directory(path:&str)
{
    let _c = Command::new("ls")
    .arg("-l")
    .arg(path)
    .output()
    .expect("ls is not usable");

    let result = String::from_utf8(_c.stdout).expect("Not UTF8 shit!");

    println!("{}",result);
}

pub fn pipe_two_process(process_name:&str){
    let c = Command::new("ps")
                .arg("aux")
                .spawn()// executes the command as a child process
                .expect("Command didn' t run");

    let d = Command::new("grep")
                    .arg(process_name)
                    .stdout(Stdio::piped())
                    .spawn() // executes the command as a child process
                    .expect("Fucked up !");
    copy(&mut d.stdout.unwrap(),&mut c.stdin.unwrap()).unwrap(); //Coz copy returns result
}