use std::collections;

pub struct Swagger {
    pub swagger: String,
    pub info: Option<Info>,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub vendor_extensions: Option<VendorExtensions>,
}

pub struct Info {
    pub description: Option<String>,
    pub version: Option<String>,
    pub title: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
}

pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

pub struct License {
    pub vendor_extensions: Option<VendorExtensions>,
    pub name: Option<String>,
    pub url: Option<String>,
}

pub type VendorExtensions = collections::HashMap<String, String>;
