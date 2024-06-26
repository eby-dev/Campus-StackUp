use endoflife::rust;

use serde_json;

fn main() {
    let json_str = r#"{
        "releaseDate": "2024-05-02",
        "eol": false,
        "latest": "1.78.0",
        "latestReleaseDate": "2024-05-02",
        "lts": false
    }"#;

    let json_object = serde_json::from_str::<rust::RustSingleCycle>(json_str);

    println!(
        "{:?}",
        json_object
    );

    if let Ok(data) = json_object {
        //println!("{:#?}", data);

        let serialised_data = serde_json::to_string_pretty(&data).unwrap();

        println!("{}", serialised_data);
        
    } else {
        println!("Not able to parse to json. Invalid data format?");
    }
}