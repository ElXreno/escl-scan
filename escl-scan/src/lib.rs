extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

pub mod structs;

use reqwest::blocking::Response;
use std::fs::File;
use std::io::copy;

pub fn scan(scanner_base_path: &str, scan_resolution: i16, destination_file: &str) {
    println!("Getting scanner capabilities...");
    let scanner_capabilities = get_scanner_capabilities(&scanner_base_path);

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
        x_resolution: scan_resolution,
        y_resolution: scan_resolution,
    };

    let request_body = serde_xml_rs::to_string(&scan_settings).unwrap();

    println!("Sending scan request with DPI {}...", scan_resolution);
    let scan_response = get_scan_response(scanner_base_path, request_body);

    let download_url = format!(
        "{}/NextDocument",
        scan_response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
    );

    println!("Downloading output file to {}...", destination_file);
    download_scan(&download_url, destination_file);
}

pub fn get_scanner_capabilities(scanner_base_path: &str) -> structs::ScannerCapabilities {
    let scanner_capabilities_response =
        reqwest::blocking::get(&format!("{}/ScannerCapabilities", scanner_base_path))
            .unwrap()
            .text()
            .unwrap();

    let scanner_capabilities: structs::ScannerCapabilities =
        serde_xml_rs::from_str(&scanner_capabilities_response).unwrap();

    scanner_capabilities
}

pub fn get_scan_response(scanner_base_path: &str, request_body: String) -> Response {
    let client = reqwest::blocking::Client::new();

    client
        .post(format!("{}/ScanJobs", &scanner_base_path).as_str())
        .body(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            request_body
        ))
        .send()
        .unwrap()
}

pub fn download_scan(download_url: &str, destination_file: &str) {
    let mut file = { File::create(destination_file).unwrap() };

    let mut response = reqwest::blocking::get(download_url).unwrap();
    copy(&mut response, &mut file).unwrap();
}
