#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_xml_rs;

mod structs;

use clap::{App, AppSettings, Arg};
use reqwest::Response;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::exit;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("ip")
                .help("IP of scanner")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output file")
                .help("Destination file")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("dpi")
                .short("d")
                .long("dpi")
                .help("Scan resolution")
                .default_value("75"),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Force scan and override output file"),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let scanner_base_path = format!("http://{}:80/eSCL", ip);
    let scan_resolution: i16 = matches.value_of("dpi").unwrap().parse().unwrap();
    let destination_file = matches.value_of("output file").unwrap();

    if !matches.is_present("force") && Path::new(destination_file).exists() {
        eprintln!("Output file exists! Exiting...");
        exit(1);
    }

    scan(&scanner_base_path, scan_resolution, destination_file);

    println!("Done!");
}

fn scan(scanner_base_path: &str, scan_resolution: i16, destination_file: &str) {
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

fn get_scanner_capabilities(scanner_base_path: &str) -> structs::ScannerCapabilities {
    let scanner_capabilities_response =
        reqwest::get(&format!("{}/ScannerCapabilities", scanner_base_path))
            .unwrap()
            .text()
            .unwrap();

    let scanner_capabilities: structs::ScannerCapabilities =
        serde_xml_rs::from_str(&scanner_capabilities_response).unwrap();

    scanner_capabilities
}

fn get_scan_response(scanner_base_path: &str, request_body: String) -> Response {
    let client = reqwest::Client::new();

    client
        .post(format!("{}/ScanJobs", &scanner_base_path).as_str())
        .body(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            request_body
        ))
        .send()
        .unwrap()
}

fn download_scan(download_url: &str, destination_file: &str) {
    let mut file = { File::create(destination_file).unwrap() };

    let mut response = reqwest::get(download_url).unwrap();
    copy(&mut response, &mut file).unwrap();
}
