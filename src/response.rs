use serde::Serialize;

use crate::{errors::Error, operation::Operation, request::Request};

#[derive(Debug, Default, Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    errormessage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requesttypedescription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounttypedescription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paymenttypedescription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseamount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currencyiso3a: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maskedpan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expirydate: Option<String>,
}

impl Response {
    pub fn success(op: &Operation) -> Self {
        let maskedpan = match op.pan() {
            Some(pan) => pan.get_masked(),
            None => String::new()
        };
        Self {
            maskedpan:              Some(maskedpan),
            expirydate:             Some(op.expirydate().to_owned()),
            requesttypedescription: Some(op.requesttypedescription().to_owned()),
            accounttypedescription: Some(op.accounttypedescription().to_owned()),
            paymenttypedescription: Some(op.paymenttypedescription().to_owned()),
            baseamount:             Some(op.baseamount()),
            currencyiso3a:          Some(op.currencyiso3a().to_owned()),
            ..Default::default()
        }
    }

    pub fn error(error: &Error, _req: &Request) -> Self {
        match error {
            Error::ValidationError(_) | Error::FieldError(_) => Self {
                errormessage:           Some(error.to_string()),
                requesttypedescription: Some(String::from("ERROR")),
                ..Default::default()
            },
        }
    }
}
