use crate::{
    constant::*,
    errors::{Error, GwResult},
    pan::Pan,
    request::Request,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Operation {
    requesttypedescription: String,
    accounttypedescription: String,
    paymenttypedescription: String,
    baseamount: u32,
    currencyiso3a: String,
    pan: Option<Pan>,
    expirydate: String,
    securitycode: String,
}

impl Operation {
    pub fn from_request(req: &Request) -> GwResult<Self> {
        Self::default()
            .set_accounttypedescription(&req.accounttypedescription)?
            .set_baseamount(req.baseamount)?
            .set_accounttypedescription(&req.accounttypedescription)?
            .set_paymenttypedescription(&req.paymenttypedescription)?
            .set_requesttypedescription(&req.requesttypedescription)?
            .set_currencyiso3a(&req.currencyiso3a)?
            .set_pan(&req.pan)?
            .set_expirydate(&req.expirydate)?
            .set_securitycode(&req.securitycode)
    }

    pub fn requesttypedescription(&self) -> &str {
        self.requesttypedescription.as_str()
    }
    pub fn accounttypedescription(&self) -> &str {
        self.accounttypedescription.as_str()
    }
    pub fn paymenttypedescription(&self) -> &str {
        self.paymenttypedescription.as_str()
    }
    pub fn baseamount(&self) -> u32 {
        self.baseamount
    }
    pub fn currencyiso3a(&self) -> &str {
        self.currencyiso3a.as_str()
    }
    pub fn pan(&self) -> &Option<Pan> {
        &self.pan
    }
    pub fn expirydate(&self) -> &str {
        self.expirydate.as_str()
    }
    pub fn securitycode(&self) -> &str {
        self.securitycode.as_str()
    }

    fn set_requesttypedescription(mut self, value: &str) -> GwResult<Self> {
        if !REQUEST_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid requesttypedescription {value}"
            )))
        } else {
            self.requesttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_accounttypedescription(mut self, value: &str) -> GwResult<Self> {
        if !ACCOUNT_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid accounttypedescription {value}"
            )))
        } else {
            self.accounttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_paymenttypedescription(mut self, value: &str) -> GwResult<Self> {
        if !PAYMENT_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid paymenttypedescription {value}"
            )))
        } else {
            self.paymenttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_baseamount(mut self, value: u32) -> GwResult<Self> {
        self.baseamount = value;
        Ok(self)
    }
    fn set_currencyiso3a(mut self, value: &str) -> GwResult<Self> {
        self.currencyiso3a = value.to_owned();
        Ok(self)
    }
    fn set_pan(mut self, value: &str) -> GwResult<Self> {
        self.pan = Some(Pan::new(value)?);
        Ok(self)
    }
    fn set_expirydate(mut self, value: &str) -> GwResult<Self> {
        self.expirydate = value.to_owned();
        Ok(self)
    }
    fn set_securitycode(mut self, value: &str) -> GwResult<Self> {
        self.securitycode = value.to_owned();
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_requesttypedescription() {
        let tests = [(
            Operation::default(),
            "AUTH",
            Ok(Operation {
                requesttypedescription: "AUTH".into(),
                accounttypedescription: "".into(),
                paymenttypedescription: "".into(),
                baseamount: 0u32,
                currencyiso3a: "".into(),
                pan: None,
                expirydate: "".into(),
                securitycode: "".into(),
            }),
        )];
        for (op, value, exp) in tests {
            assert_eq!(op.set_requesttypedescription(value), exp);
        }
    }

    #[test]
    fn test_set_pan() {
        let tests = [
            (
                Operation::default(),
                "4000000000000000",
                Ok(Operation {
                    requesttypedescription: "".into(),
                    accounttypedescription: "".into(),
                    paymenttypedescription: "".into(),
                    baseamount: 0u32,
                    currencyiso3a: "".into(),
                    pan: Some(Pan::new("4000000000000000").unwrap()),
                    expirydate: "".into(),
                    securitycode: "".into(),
                }),
            ),
            (
                Operation::default(),
                "40000",
                Err(Error::ValidationError("pan is invalid length: 5".into())),
            ),
        ];
        for (op, value, exp) in tests {
            assert_eq!(op.set_pan(value), exp);
        }
    }
}
