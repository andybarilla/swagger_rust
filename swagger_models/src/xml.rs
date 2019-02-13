use crate::common;

pub struct XMLObject {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapper: Option<bool>,
    pub vendor_extensions: Option<common::VendorExtensions>,
}
