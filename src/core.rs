use std::io;
use std::process::{Command, Stdio};
pub fn process() {
    println!("Press enter to start recording");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim_end().contains("enter") {
                let child = Command::new("script")
                    .arg("log.txt")
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");
                println!("{:?}", child.stdin);
            } else {
                println!("Bye!");
            }
        }
        Err(_) => println!("Error"),
    }
}
