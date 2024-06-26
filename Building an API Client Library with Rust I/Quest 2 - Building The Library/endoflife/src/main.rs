use endoflife::rust;
use endoflife::request::api_request_single_rust_cycle;

fn main() {
    let rust_version = "1.78";
    let data = api_request_single_rust_cycle(rust_version).unwrap();

    println!("{:#?}", data);

    let json_str = serde_json::to_string_pretty(&data).unwrap();

    println!("{}", &json_str);
}