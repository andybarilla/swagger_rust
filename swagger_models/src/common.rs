use crate::common;
use crate::xml;
use serde_json::Value;
use std::collections::HashMap;

// TODO for now just store numbers as strings
pub type Number = String;

pub type ExtendedHashMap = HashMap<String, StringOrVendorExtension>;

pub enum CollectionFormat {
    Csv,
    Ssv,
    Tsv,
    Pipes,
    Multi,
}

impl CollectionFormat {
    pub fn as_str(&self) -> &str {
        match self {
            CollectionFormat::Csv => "csv",
            CollectionFormat::Ssv => "ssv",
            CollectionFormat::Tsv => "tsv",
            CollectionFormat::Pipes => "pipes",
            CollectionFormat::Multi => "multi",
        }
    }
}

pub type VendorExtensions = HashMap<String, Value>;

pub enum StringOrVendorExtension {
    String(String),
    VendorExtension(Value),
}

pub struct Schema {
    pub ref_: Option<String>,
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
    pub multiple_of: Option<Number>,
    pub maximum: Option<Number>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<Number>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub pattern: Option<String>,
    pub max_items: Option<u32>,
    pub min_items: Option<u32>,
    pub unique_items: Option<bool>,
    pub max_properties: Option<u32>,
    pub min_properties: Option<u32>,
    pub required: Option<bool>,
    pub enum_: Option<Vec<String>>,
    pub type_: Option<String>,
    pub discriminator: Option<String>,
    pub read_only: Option<bool>,
    pub xml: Option<xml::XMLObject>,
    pub external_docs: Option<ExternalDocumentation>,
    pub vendor_extensions: Option<VendorExtensions>,
}

pub struct Items {
    pub type_: String,
    pub format: Option<String>,
    pub items: Option<Box<Items>>,
    pub collection_format: Option<CollectionFormat>,
    pub default: Option<String>,
    pub maximum: Option<common::Number>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<common::Number>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub pattern: Option<String>,
    pub max_items: Option<u32>,
    pub min_items: Option<u32>,
    pub unique_items: Option<bool>,
    pub enum_: Option<Vec<String>>,
    pub multiple_of: Option<common::Number>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
    pub vendor_extensions: Option<common::VendorExtensions>,
}
