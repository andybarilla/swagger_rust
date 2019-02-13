use crate::common;
use std::collections::HashMap;

pub struct ParameterBody {
    pub name: String,
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: common::Schema,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct ParameterOther {
    pub name: String,
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub type_: String,
    pub format: Option<String>,
    pub allow_empty_value: Option<bool>,
    pub items: Option<common::Items>,
    pub collection_format: Option<common::CollectionFormat>,
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

pub enum Parameter {
    Body(ParameterBody),
    Other(ParameterOther),
}

pub type Parameters = Vec<Parameter>;

pub type ParametersDefinitions = HashMap<String, Parameter>;

pub type ParametersOrReferences = Vec<ParameterOrReference>;

pub enum ParameterOrReference {
    Parameter(Box<Parameter>),
    Reference(Reference),
}

pub struct Reference {
    pub _ref: String,
}
