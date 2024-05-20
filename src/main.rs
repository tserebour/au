fn main() {

    use std::process::Command;
    
    
    Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");


    std::process::exit(0);
    
}
