use std::collections::HashMap;

// TODO for now just store numbers as strings
pub type Number = String;

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
    pub parameters: Option<ParametersDefinitions>,
    pub responses: Option<ResponsesDefinitions>,
    pub security_definitions: Option<SecurityDefinitions>,
    pub security: Option<SecurityRequirements>,
    pub tags: Option<Tags>,
    pub external_docs: Option<ExternalDocumentation>,
}

pub struct Info {
    pub title: Option<String>,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

pub struct License {
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
    pub ref_: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub head: Option<Operation>,
    pub options: Option<Operation>,
    pub patch: Option<Operation>,
    pub parameters: Option<ParametersOrReferences>,
}

pub type ParametersOrReferences = Vec<ParameterOrReference>;

pub enum ParameterOrReference {
    Parameter(Parameter),
    Reference(Reference),
}

pub type Paths = HashMap<String, Path>;

pub type Requirements = HashMap<String, Vec<String>>;

pub struct Operation {
    pub tags: Option<Tags>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub schemes: Option<Schemes>,
    pub consumes: Option<Consumes>,
    pub produces: Option<Produces>,
    pub parameters: Option<ParametersOrReferences>,
    pub responses: Option<Responses>,
    pub security: Option<SecurityRequirements>,
    pub external_docs: Option<ExternalDocumentation>,
    pub deprecated: Option<bool>,
}

pub struct Response {
    pub description: Option<String>,
    pub schema: Option<Schema>,
    pub headers: Option<Headers>,
    pub examples: Option<Examples>,
}

pub type Responses = HashMap<String, Response>;

pub type ResponsesDefinitions = HashMap<String, Response>;

pub type Security = Vec<HashMap<String, Vec<String>>>;

pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}

pub struct Model {}

pub struct Property {}

pub struct Example {
    // TODO this could be anything
}

pub type Examples = HashMap<String, Example>;

pub type Definitions = HashMap<String, Schema>;

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
    pub xml: Option<XMLObject>,
    pub external_docs: Option<ExternalDocumentation>,
}

pub struct Reference {
    pub _ref: String,
}

pub struct XMLObject {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapper: Option<bool>,
}

pub type SecurityDefinitions = HashMap<String, SecurityScheme>;

pub struct SecurityScheme {
    pub type_: String,
    pub description: Option<String>,
    pub name: String,
    pub in_: String,
    pub flow: String,
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Scopes,
}

pub type Scopes = HashMap<String, String>;

pub struct ParameterBody {
    pub name: String,
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Schema,
}

pub struct ParameterOther {
    pub name: String,
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub type_: String,
    pub format: Option<String>,
    pub allow_empty_value: Option<bool>,
    pub items: Option<Items>,
    pub collection_format: Option<String>,
    pub default: Option<String>,
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
    pub enum_: Option<Vec<String>>,
    pub multiple_of: Option<Number>,
}

pub enum Parameter {
    Body(ParameterBody),
    Other(ParameterOther),
}

pub type Parameters = Vec<Parameter>;

pub type ParametersDefinitions = HashMap<String, Parameter>;

pub type VendorExtensions = HashMap<String, String>;
pub type Tags = Vec<Tag>;

pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
}

pub type Headers = HashMap<String, Header>;

pub struct Header {
    pub type_: Option<String>,
    pub description: String,
    pub format: Option<String>,
    pub items: Option<Items>,
    pub collection_format: Option<String>, // TODO enum this?
    pub default: Option<String>,
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
    pub enum_: Option<Vec<String>>,
    pub multiple_of: Option<Number>,
}

pub struct Items {
    pub type_: String,
    pub format: Option<String>,
    pub items: Option<Box<Items>>,
    pub collection_format: Option<String>, // TODO enum this?
    pub default: Option<String>,
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
    pub enum_: Option<Vec<String>>,
    pub multiple_of: Option<Number>,
}