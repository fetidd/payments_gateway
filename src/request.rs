use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Request {
    pub requesttypedescription: String,
    pub accounttypedescription: String,
    pub paymenttypedescription: String,
    pub baseamount: u32,
    pub currencyiso3a: String,
    pub pan: String,
    pub expirydate: String,
    pub securitycode: String,
}
