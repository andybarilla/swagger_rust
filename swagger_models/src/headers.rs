use crate::auth;
use crate::common;
use crate::parameters;

use std::any::Any;
use std::collections::HashMap;

pub struct Swagger {
    pub swagger: String,
    pub info: Option<Info>,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub vendor_extensions: Option<common::VendorExtensions>,
    pub tags: Option<common::Tags>,
    pub schemes: Option<Schemes>,
    pub consumes: Option<Consumes>,
    pub produces: Option<Produces>,
    pub security_definitions: Option<auth::SecurityDefinitions>,
}

pub struct Info {
    pub description: Option<String>,
    pub version: Option<String>,
    pub title: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub security_requirements: Option<SecurityRequirements>,
    pub paths: Option<Paths>,
}

pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

pub struct License {
    pub vendor_extensions: Option<common::VendorExtensions>,
    pub name: Option<String>,
    pub url: Option<String>,
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
            &Scheme::Http => "http",
            &Scheme::Https => "https",
            &Scheme::Ws => "ws",
            &Scheme::Wss => "wss",
        }
    }
}

pub type Schemes = Vec<Scheme>;

pub type Consumes = Vec<String>;

pub type Produces = Vec<String>;

pub struct SecurityRequirement {
    pub name: Option<String>,
    pub scopes: Option<Scopes>,
    pub requirements: Option<Requirements>,
}

pub type SecurityRequirements = Vec<SecurityRequirement>;

pub struct Path {
    pub vendor_extensions: Option<common::VendorExtensions>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub head: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
    pub options: Option<Operation>,
    pub parameters: Option<parameters::Parameters>,
}

pub type Paths = HashMap<String, Path>;

pub type Scopes = Vec<String>;

pub type Requirements = HashMap<String, Vec<String>>;

pub struct Operation {
    pub vendor_extensions: Option<common::VendorExtensions>,
    pub tags: Option<common::Tags>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub schemes: Option<Schemes>,
    pub consumes: Option<Consumes>,
    pub produces: Option<Produces>,
    pub parameters: Option<parameters::Parameters>,
    pub responses: Option<Responses>,
    pub security: Option<Security>,
    pub external_docs: Option<ExternalDocs>,
    pub deprecated: Option<bool>,
}

pub struct Response {
    pub description: Option<String>,
    pub schema_as_model: Option<Model>,
    pub property_as_model: Option<Property>,
    pub examples: Option<Examples>,
}

pub type Responses = Vec<Response>;

pub type Security = Vec<HashMap<String, Vec<String>>>;

pub struct ExternalDocs {}

pub struct Model {}

pub struct Property {}

pub struct Example {}

pub type Examples = HashMap<String, Example>;
