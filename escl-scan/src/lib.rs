/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

pub mod structs;

use reqwest::blocking::Response;
use std::fs::File;

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

    let serialized = serde_xml_rs::to_string(&scan_settings).unwrap();
    let request_body = set_xml_namespace(serialized);

    println!("Sending scan request with DPI {}...", scan_resolution);
    let scan_response = get_scan_response(scanner_base_path, request_body);

    let error = format!("Scan request failed: {:?}", scan_response);
    let location = scan_response.headers().get("location").expect(&error);
    let download_url = format!("{}/NextDocument", location.to_str().unwrap());

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

// At time of writing, serde-xml-rs doesn't support setting XML attributes.
// Tracking issue: https://github.com/RReverser/serde-xml-rs/issues/49
pub fn set_xml_namespace(xml: String) -> String {
    let xsi = "xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"";
    let pwg = "xmlns:pwg=\"http://www.pwg.org/schemas/2010/12/sm\"";
    let escl = "xmlns:escl=\"http://schemas.hp.com/imaging/escl/2011/05/03\"";

    xml.replace("<escl:ScanSettings>", &format!("<escl:ScanSettings {} {} {}>", xsi, pwg, escl))
}

pub fn get_scan_response(scanner_base_path: &str, request_body: String) -> Response {
    reqwest::blocking::Client::builder()
        .http1_title_case_headers() // Some printers respond 400 unless headers are title case.
        .build()
        .unwrap()
        .post(format!("{}/ScanJobs", &scanner_base_path).as_str())
        .body(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", request_body))
        .send()
        .unwrap()
}

pub fn download_scan(download_url: &str, destination_file: &str) {
    let mut file = { File::create(destination_file).unwrap() };

    reqwest::blocking::get(download_url).unwrap().copy_to(&mut file).unwrap();
}
