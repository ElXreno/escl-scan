# escl-scan

---

[![Copr build status](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan/package/escl-scan/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan)
[![Travis-CI build status](https://travis-ci.com/ElXreno/escl-scan.svg?branch=master)](https://travis-ci.com/ElXreno/escl-scan)
[![codecov](https://codecov.io/gh/ElXreno/escl-scan/branch/master/graph/badge.svg)](https://codecov.io/gh/ElXreno/escl-scan)
[![dependency status](https://deps.rs/repo/github/elxreno/escl-scan/status.svg)](https://deps.rs/repo/github/elxreno/escl-scan)

## Example:
```rust
extern crate escl_scan;
extern crate serde_xml_rs;

fn main() {
    // Define variables...
    let scanner_base_path = "192.168.2.5"; // IP or mDNS (for example "Printer.local." to printer
    let scan_resolution = 300; // DPI
    let destination_file = "output.jpg"; // I think you already know what it is for

    // Get scanner capabilities...
    let scanner_capabilities = escl_scan::get_scanner_capabilities(&scanner_base_path);

    // Create scan settings...
    let scan_settings: escl_scan::structs::ScanSettings = escl_scan::structs::ScanSettings {
        version: "2.6".to_string(),
        scan_regions: escl_scan::structs::ScanRegion {
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

    // Send scan request...
    let request_body = serde_xml_rs::to_string(&scan_settings).unwrap();
    let scan_response = escl_scan::get_scan_response(scanner_base_path, request_body);

    // Get scan url...
    let download_url = format!(
        "{}/NextDocument",
        scan_response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
    );

    // Download scan...
    escl_scan::download_scan(&download_url, destination_file);
}
```