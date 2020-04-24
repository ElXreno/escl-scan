/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize)]
pub struct Platen {
    #[serde(rename = "PlatenInputCaps", default)]
    pub platen_input_caps: PlatenInputCaps,
}

#[derive(Default, Debug, Deserialize)]
pub struct PlatenInputCaps {
    #[serde(rename = "MinWidth", default)]
    pub min_width: u16,
    #[serde(rename = "MaxWidth", default)]
    pub max_width: i16,
    #[serde(rename = "MinHeight", default)]
    pub min_height: i16,
    #[serde(rename = "MaxHeight", default)]
    pub max_height: i16,
    #[serde(rename = "MaxScanRegions", default)]
    pub max_scan_regions: u16,
    // TODO: Make SettingProfiles
    // TODO: Make SupportedIntents
    #[serde(rename = "MaxOpticalXResolution", default)]
    pub max_optical_xresolution: u16,
    #[serde(rename = "MaxOpticalYResolution", default)]
    pub max_optical_yresolution: u16,
    #[serde(rename = "RiskyLeftMargin", default)]
    pub risky_left_margin: u16,
    #[serde(rename = "RiskyRightMargin", default)]
    pub risky_right_margin: u16,
    #[serde(rename = "RiskyTopMargin", default)]
    pub risky_top_margin: u16,
    #[serde(rename = "RiskyBottomMargin", default)]
    pub risky_bottom_margin: u16,
}

#[derive(Default, Debug, Deserialize)]
pub struct CompressionFactorSupport {
    #[serde(rename = "Min", default)]
    pub min: i8,
    #[serde(rename = "Max", default)]
    pub max: i8,
    #[serde(rename = "Normal", default)]
    pub normal: i8,
    #[serde(rename = "Step", default)]
    pub step: i8,
}

#[derive(Default, Debug, Deserialize)]
pub struct SupportedMediaTypes {
    #[serde(rename = "MediaType", default)]
    pub media_types: Vec<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct SharpenSupport {
    #[serde(rename = "Min", default)]
    pub min: i8,
    #[serde(rename = "Max", default)]
    pub max: i8,
    #[serde(rename = "Normal", default)]
    pub normal: i8,
    #[serde(rename = "Step", default)]
    pub step: i8,
}

#[derive(Default, Debug, Deserialize)]
pub struct ScannerCapabilities {
    #[serde(rename = "Version", default)]
    pub version: String,
    #[serde(rename = "MakeAndModel", default)]
    pub make_and_model: String,
    #[serde(rename = "SerialNumber", default)]
    pub serial_number: String,
    #[serde(rename = "UUID", default)]
    pub uuid: String,
    #[serde(rename = "AdminURI", default)]
    pub admin_uri: String,
    #[serde(rename = "IconURI", default)]
    pub icon_uri: String,
    #[serde(rename = "Platen", default)]
    pub platen: Platen,
    #[serde(rename = "CompressionFactorSupport", default)]
    pub compression_factor_support: CompressionFactorSupport,
    #[serde(rename = "SupportedMediaTypes", default)]
    pub supported_media_types: SupportedMediaTypes,
    #[serde(rename = "SharpenSupport", default)]
    pub sharpen_support: SharpenSupport,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "scan:ScanRegion")]
pub struct ScanRegion {
    #[serde(rename = "pwg:XOffset")]
    pub x_offset: i16,
    #[serde(rename = "pwg:YOffset")]
    pub y_offset: i16,
    #[serde(rename = "pwg:Width")]
    pub width: i16,
    #[serde(rename = "pwg:Height")]
    pub height: i16,
    #[serde(rename = "pwg:ContentRegionUnits")]
    pub content_region_units: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "scan:ScanSettings")]
pub struct ScanSettings {
    #[serde(rename = "pwg:Version")]
    pub version: String,
    #[serde(rename = "pwg:ScanRegions")]
    pub scan_regions: ScanRegion,
    #[serde(rename = "scan:InputSource")]
    pub input_source: String,
    #[serde(rename = "scan:ColorMode")]
    pub color_mode: String,
    #[serde(rename = "scan:XResolution")]
    pub x_resolution: i16,
    #[serde(rename = "scan:YResolution")]
    pub y_resolution: i16,
}
