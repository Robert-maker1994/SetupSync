use dev_setup::applications::get_applications;
use dev_setup::download_application;


fn main() {
    let application_ids = get_applications();
  
    println!("Installing you're set up");
 
    for (name, id) in &application_ids {
        download_application(name, id);
    }

    println!("✔✔✔✔ Install Application set up ✔✔✔✔");
}

