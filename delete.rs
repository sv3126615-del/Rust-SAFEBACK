 use crate::Command;
 use crate::Path;
 use crate::io;

pub fn delete_file() {
    println!("Enter the the path : ");

    let  mut file1_path = String::new();

    io::stdin().read_line(&mut file1_path).expect("Fail on Reading Delete input");

    let file1_path = file1_path.trim();
    
    let binding = file1_path.trim();

    let soufile = Path::new(&binding);

    let rmcmd = Command::new("rm").arg("-rf").arg(soufile).output();
    println!("Task complete 😈 : {:?}",rmcmd);
}