use std::ops::Range;

use crate::common::{ImageFile, Jpeg, RAW};
use crate::helpers::{bytes_to_string, bytes_to_u32_be};

const FORMAT_RANGE: Range<usize> = 0..16;
const VERSION_RANGE: Range<usize> = 16..20;
const IDENTIFIER_RANGE: Range<usize> = 20..28;
const MODEL_RANGE: Range<usize> = 28..60;
const OFFSET_DIRECTORY_VERSION: Range<usize> = 60..64;

const JPEG_OFFSET_START: usize = 84;
const JPEG_LENGTH_START: usize = 88;
const CFA_HEADER_OFFSET_START: usize = 92;
const CFA_HEADER_LENGTH_START: usize = 96;
const CFA_OFFSET_START: usize = 100;
const CFA_LENGTH_START: usize = 104;

pub fn parse(bytes: &[u8]) -> Option<ImageFile> {
    debug_info(bytes);

    let model = parse_model(bytes)?;
    let format = parse_format(bytes)?;
    let identifier = parse_identifier(bytes)?;
    let version = parse_version(bytes)?;
    let jpeg = parse_jpeg(bytes)?;

    Some(ImageFile {
        format,
        identifier,
        model,
        version,
        jpeg,
    })
}

fn parse_jpeg(raw_bytes: &[u8]) -> Option<Jpeg> {
    let offset = bytes_to_u32_be(raw_bytes, JPEG_OFFSET_START)? as usize;
    let length = bytes_to_u32_be(raw_bytes, JPEG_LENGTH_START)? as usize;

    let range = offset..(offset + length);

    let bytes = raw_bytes[range].to_vec();

    Some(Jpeg { bytes })
}

fn parse_cfa(raw_bytes: &[u8]) -> Option<RAW> {
    let offset = bytes_to_u32_be(raw_bytes, CFA_OFFSET_START).unwrap() as usize;
    let length = bytes_to_u32_be(raw_bytes, CFA_LENGTH_START).unwrap() as usize;

    let byte_range = offset..(offset + length);

    println!("raw range {:?}", byte_range);

    let bytes = raw_bytes[byte_range].to_vec();

    Some(RAW { bytes })
}

// TODO: remove this obviously
fn debug_info(bytes: &[u8]) {
    println!(
        "OFFSET_DIRECTORY_VERSION {:?}",
        bytes_to_string(bytes, OFFSET_DIRECTORY_VERSION)
    );
    println!(
        "JPEG_OFFSET_RANGE {:?}",
        bytes_to_u32_be(bytes, JPEG_OFFSET_START)
    );
    println!(
        "JPEG_LENGTH_RANGE {:?}",
        bytes_to_u32_be(bytes, JPEG_LENGTH_START)
    );
    println!(
        "CFA_HEADER_OFFSET_RANGE {:?}",
        bytes_to_u32_be(bytes, CFA_HEADER_OFFSET_START)
    );
    println!(
        "CFA_HEADER_LENGTH_RANGE {:?}",
        bytes_to_u32_be(bytes, CFA_HEADER_LENGTH_START)
    );
    println!(
        "CFA_OFFSET_RANGE {:?}",
        bytes_to_u32_be(bytes, CFA_OFFSET_START)
    );
    println!(
        "CFA_LENGTH_RANGE {:?}",
        bytes_to_u32_be(bytes, CFA_LENGTH_START)
    );
}

fn parse_model(bytes: &[u8]) -> Option<String> {
    let parsed = bytes_to_string(bytes, MODEL_RANGE);

    Some(parsed?.replace('\0', ""))
}

fn parse_version(bytes: &[u8]) -> Option<String> {
    bytes_to_string(bytes, VERSION_RANGE)
}

fn parse_identifier(bytes: &[u8]) -> Option<String> {
    bytes_to_string(bytes, IDENTIFIER_RANGE)
}

fn parse_format(bytes: &[u8]) -> Option<String> {
    bytes_to_string(bytes, FORMAT_RANGE)
}
