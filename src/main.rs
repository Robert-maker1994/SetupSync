use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write};

fn run_command(id: &str) -> bool {
    println!("ID {}", id);

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

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.contains(app_id) {
        println!("{} is already installed",app_id);
        stdout.contains(app_id)
    } else {
        println!("{} is not installed", app_id);
        stdout.contains(app_id)
    }
    
}



fn print_task(task: &str) {
    println!("{}", task);
}

fn mark_done(task: &str) {
    println!("[✔] {}", task);
}

fn main() {
    let mut application_ids = HashMap::new();

    application_ids.insert(String::from("Mail"), "Proton.ProtonMail");
    application_ids.insert(String::from("Pass"), "Proton.ProtonPass");
    application_ids.insert(String::from("VPN"), "Proton.ProtonVPN");
    application_ids.insert(String::from("VS"), "Microsoft.VisualStudioCode");
    application_ids.insert(String::from("VS Code"), "Microsoft.VisualStudio.2022.Community");
    application_ids.insert(String::from("ICloud"), "AppleInc.iCloud_nzyj5cx40ttqa");
    application_ids.insert(String::from("Notion"), "Notion.Notion");
    application_ids.insert(String::from("nvm"), "CoreyButler.NVMforWindows");

    println!("Install Application set up");

    print_task("Installing Proton");
    if !is_app_installed( application_ids.get("VPN").unwrap()) {
        if run_command(application_ids.get("VPN").unwrap()) {
            mark_done("Install ProtonMail")
        }
    }

    if !is_app_installed( application_ids.get("Pass").unwrap()) {
        if run_command(application_ids.get("Pass").unwrap()) {
            mark_done("Install ProtonMail")
        }
    }

    if !is_app_installed( application_ids.get("Mail").unwrap()) {
        if run_command(application_ids.get("Mail").unwrap()) {
            mark_done("Install ProtonMail")
        }
    }
    mark_done("Finished Proton download");

    print_task("Installing IDES");
    if !is_app_installed( application_ids.get("VS").unwrap()) {
        if run_command(application_ids.get("VS").unwrap()) {
            mark_done("Installed VS Code ")
        }
    }
    mark_done("Finished IDES");

   

    print_task("Installing Dev set up");
    if !is_app_installed( application_ids.get("nvm").unwrap()) {
        if run_command(application_ids.get("nvm").unwrap()) {
            mark_done("Installed Node Version Manager ")
        }
    }
    mark_done("Finished DevSetup");

    println!("Install Application set up");

}
