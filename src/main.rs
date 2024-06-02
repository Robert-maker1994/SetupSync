use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn run_command(id: &str) -> bool {
    let mut binding = Command::new("winget");

    let command = binding
        .arg("install")
        .arg("--id")
        .arg(id);

    let mut child= command.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute winget list command");   

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("stdout: {}", line);
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("stderr: {}", line);
            }
        }
    }

    let status = child.wait().expect("Command wasn't running");

    status.success()
}

fn is_app_installed(app_id: &str) -> bool {
    let output = Command::new("winget")
        .arg("list")
        .arg("--id")
        .arg(app_id)
        .output()
        .expect("Failed to execute winget list command");
    let output = output;

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.contains(app_id)
}

fn download_application(name: &str, id: &str){
    println!("Checking if {} is download", name);
    if !is_app_installed( id) {
        if run_command(id) {
            
            println!("Installed {} [✔]", name);
        }
    } else {
        println!("{} is already installed [✔]", name);
    }
}

fn main() {

    let mut application_ids = HashMap::new();
    
    application_ids.insert(String::from("Mail"), "Proton.ProtonMail");
    application_ids.insert(String::from("Pass"), "Proton.ProtonPass");
    application_ids.insert(String::from("VPN"), "Proton.ProtonVPN");
    application_ids.insert(String::from("Visual Studio Code"), "Microsoft.VisualStudioCode");
    application_ids.insert(String::from("Visual Studio"), "Microsoft.VisualStudio.2022.Community");
    application_ids.insert(String::from("Apple ICloud"), "AppleInc.iCloud_nzyj5cx40ttqa");
    application_ids.insert(String::from("Notion"), "Notion.Notion");
    application_ids.insert(String::from("Node Version Manager"), "CoreyButler.NVMforWindows");

    println!("Installing you're set up");
 
   
    for (name, id) in &application_ids {
        download_application(name, id);
    }

    println!("✔✔✔✔ Install Application set up ✔✔✔✔");
}

