use serde_json::Value;
use std::collections::HashMap;

pub mod common;
pub mod parameters;
pub mod security;
pub mod xml;

pub struct Swagger {
    pub swagger: String,
    pub info: Info,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub schemes: Option<Schemes>,
    pub consumes: Option<Consumes>,
    pub produces: Option<Produces>,
    pub paths: Paths,
    pub definitions: Option<Definitions>,
    pub parameters: Option<parameters::ParametersDefinitions>,
    pub responses: Option<ResponsesDefinitions>,
    pub security_definitions: Option<security::SecurityDefinitions>,
    pub security: Option<security::SecurityRequirements>,
    pub tags: Option<Tags>,
    pub external_docs: Option<common::ExternalDocumentation>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct Info {
    pub title: Option<String>,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
}

impl Scheme {
    pub fn as_str(&self) -> &str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
            Scheme::Ws => "ws",
            Scheme::Wss => "wss",
        }
    }
}
pub type Schemes = Vec<Scheme>;

pub type Consumes = Vec<String>;

pub type Produces = Vec<String>;

pub struct PathItem {
    pub ref_: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub head: Option<Operation>,
    pub options: Option<Operation>,
    pub patch: Option<Operation>,
    pub parameters: Option<parameters::ParametersOrReferences>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub type Paths = common::ExtendedHashMap;

pub struct Operation {
    pub tags: Option<Tags>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub schemes: Option<Schemes>,
    pub consumes: Option<Consumes>,
    pub produces: Option<Produces>,
    pub parameters: Option<parameters::ParametersOrReferences>,
    pub responses: Option<Responses>,
    pub security: Option<security::SecurityRequirements>,
    pub external_docs: Option<common::ExternalDocumentation>,
    pub deprecated: Option<bool>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub struct Response {
    pub description: Option<String>,
    pub schema: Option<common::Schema>,
    pub headers: Option<Headers>,
    pub examples: Option<Examples>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub type Responses = common::ExtendedHashMap;

pub type ResponsesDefinitions = HashMap<String, Response>;

pub type Examples = HashMap<String, Value>;

pub type Definitions = HashMap<String, common::Schema>;

pub type Tags = Vec<Tag>;

pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<common::ExternalDocumentation>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}

pub type Headers = HashMap<String, Header>;

pub struct Header {
    pub type_: Option<String>,
    pub description: String,
    pub format: Option<String>,
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
