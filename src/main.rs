use std::io::Read;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let path = &arguments[1];
    let mut tempf: std::fs::File = std::fs::File::open(path).expect("Error: file not find");
    let mut data = String::new();
    tempf.read_to_string(&mut data).expect("Error");
    let jsonfile: serde_json::Value = serde_yaml::from_str(&data).unwrap();
    println!("{}", serde_json::to_string_pretty(&jsonfile).unwrap());
}
