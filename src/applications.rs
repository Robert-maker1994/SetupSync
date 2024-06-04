use std::collections::HashMap;

pub fn get_applications() -> HashMap<String, String> {
    let mut application_ids = HashMap::new();
    
    // Proton
    application_ids.insert(String::from("Mail"), String::from("Proton.ProtonMail"));
    application_ids.insert(String::from("Pass"), String::from("Proton.ProtonPass"));
    application_ids.insert(String::from("VPN"), String::from("Proton.ProtonVPN"));
    
    // IDEs
    application_ids.insert(String::from("Visual Studio Code"), String::from("Microsoft.VisualStudioCode"));
    application_ids.insert(String::from("Visual Studio"), String::from("Microsoft.VisualStudio.2022.Community"));
    
    // Apple
    application_ids.insert(String::from("Apple ICloud"), String::from("AppleInc.iCloud_nzyj5cx40ttqa"));
    
    // Notes
    application_ids.insert(String::from("Notion"), String::from("Notion.Notion"));
    
    // Development
    application_ids.insert(String::from("Node Version Manager"), String::from("CoreyButler.NVMforWindows"));
    application_ids.insert(String::from("Mongo Server"), String::from("MongoDB.Server"));


    application_ids
}