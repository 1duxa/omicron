use crate::types::types::Entry;
use quick_xml::de::from_str;
use std::{fs::File, io::Read};
mod parser;
mod types;

fn load_entry() -> Result<Entry, Box<dyn std::error::Error>> {
    let mut file = File::open("src/index.xml").expect("Failed to open file");
    let mut xml_content = String::new();
    file.read_to_string(&mut xml_content)?;
    let res = from_str(&xml_content)?;
    Ok(res)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entry: Entry = load_entry()?;
    println!("{:#?}", entry);

    Ok(())
}
