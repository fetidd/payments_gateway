use serde::Serialize;

use crate::{errors::Error, operation::Operation, useful};

use super::Request;

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
        let masked_pan = useful::mask_pan(&op.pan());
        Self {
            maskedpan: Some(masked_pan),
            expirydate: Some(op.expirydate().to_owned()),
            requesttypedescription: Some(op.requesttypedescription().to_owned()),
            accounttypedescription: Some(op.accounttypedescription().to_owned()),
            paymenttypedescription: Some(op.paymenttypedescription().to_owned()),
            baseamount: Some(op.baseamount()),
            currencyiso3a: Some(op.currencyiso3a().to_owned()),
            ..Default::default()
        }
    }

    pub fn error(error: &Error, req: &Request) -> Self {
        let masked_pan = useful::mask_pan(&req.pan);
        match error {
            Error::ValidationError(_) => Self {
                errormessage: Some(error.to_string()),
                maskedpan: Some(masked_pan),
                expirydate: Some(req.expirydate.clone()),
                requesttypedescription: Some(req.requesttypedescription.clone()),
                accounttypedescription: Some(req.accounttypedescription.clone()),
                paymenttypedescription: Some(req.paymenttypedescription.clone()),
                baseamount: Some(req.baseamount),
                currencyiso3a: Some(req.currencyiso3a.clone()),
                ..Default::default()
            },
            Error::FieldError(_) => Self {
                errormessage: Some(error.to_string()),
                maskedpan: Some(masked_pan),
                expirydate: Some(req.expirydate.clone()),
                requesttypedescription: Some(String::from("ERROR")),
                accounttypedescription: None,
                paymenttypedescription: None,
                baseamount: None,
                currencyiso3a: None,
                ..Default::default()
            },
        }
    }
}
