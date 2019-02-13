use crate::common;
use std::collections::HashMap;

pub type Security = Vec<HashMap<String, Vec<String>>>;

pub type SecurityDefinitions = HashMap<String, SecurityScheme>;

pub type Scopes = common::ExtendedHashMap;

pub type Requirements = HashMap<String, Vec<String>>;

pub struct SecurityScheme {
    pub type_: String,
    pub description: Option<String>,
    pub name: String,
    pub in_: String,
    pub flow: String,
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Scopes,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct SecurityRequirement {
    pub name: Option<String>,
    pub scopes: Option<Scopes>,
    pub requirements: Option<Requirements>,
}

pub type SecurityRequirements = Vec<SecurityRequirement>;
