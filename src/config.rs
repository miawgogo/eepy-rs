#[derive(serde::Deserialize)]
struct Config {
   timeout: u16, // in minutes
   backend: Option<String>,
   keys: Vec<Service>,
}


// The procfs libary i am using splits them into ipv4 and v6, but im just going to treat them as equal
#[derive(serde::Deserialize)]
enum NetFamilies {
    Tcp,
    Upd
}

#[derive(serde::Deserialize)]
struct Service {
    name: String,
    port: u16,
    families: Option<Vec<NetFamilies>> // Assuming all if its none
}