#[macro_use]
extern crate lazy_static;

pub mod libdmtx;
mod specifications;
pub mod security;
pub mod document;
use document::ErrorDocument;
pub mod serialize;
mod pack_order;
pub use pack_order::PackOrder;
use std::str::from_utf8;

use document::Document;

pub fn data_to_document(data: &[u8]) -> Result<Document, ErrorDocument> {
    document::decode(from_utf8(data).expect("Error"))
}

pub fn data_to_json(data: &[u8]) -> String{
    serialize::json::serialize_pretty(data_to_document(data))
}

pub fn image_to_document(pxls: &[u8], width: u32, height: u32, format: PackOrder) -> Result<Document, ErrorDocument> {
    let data  = libdmtx::dmtx_read(pxls, width, height, format.into());

    document::decode(from_utf8(&data).expect("Error"))
}

pub fn image_to_json(pxls: &[u8], width: u32, height: u32, format: PackOrder) -> String {
    serialize::json::serialize_pretty(image_to_document(pxls, width, height, format))
}

mod lib_c;
pub use lib_c::*;