extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

mod structs;

use serde_xml_rs::{from_str, to_string};
use std::fs::File;
use std::io::copy;

fn main() {
    let scanner_base_path = "http://Printer.local.:80/eSCL";

    let scanner_capabilities_response =
        reqwest::get(&format!("{}/ScannerCapabilities", scanner_base_path))
            .unwrap()
            .text()
            .unwrap();
    let scanner_capabilities: structs::ScannerCapabilities =
        from_str(&scanner_capabilities_response).unwrap();

    let scan_settings: structs::ScanSettings = structs::ScanSettings {
        version: "2.6".to_string(),
        scan_regions: structs::ScanRegion {
            x_offset: 0,
            y_offset: 0,
            width: scanner_capabilities.platen.platen_input_caps.max_width,
            height: scanner_capabilities.platen.platen_input_caps.max_height,
            content_region_units: "escl:ThreeHundredthsOfInches".to_string(),
        },
        input_source: "Platen".to_string(),
        color_mode: "RGB24".to_string(),
        x_resolution: 75,
        y_resolution: 75,
    };

    let request_body = to_string(&scan_settings).unwrap();

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/ScanJobs", &scanner_base_path).as_str())
        .body(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            request_body
        ))
        .send()
        .unwrap();

    let mut destination_file = { File::create("output.jpg").unwrap() };

    let download_url = format!(
        "{}/NextDocument",
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
    );

    let mut response = reqwest::get(&download_url).unwrap();

    copy(&mut response, &mut destination_file).unwrap();
}
