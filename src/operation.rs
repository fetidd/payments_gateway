use crate::{constant::*, errors::Error, gateway::Request};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Operation {
    requesttypedescription: String,
    accounttypedescription: String,
    paymenttypedescription: String,
    baseamount: u32,
    currencyiso3a: String,
    pan: String,
    expirydate: String,
    securitycode: String,
}

impl Operation {
    pub fn from_request(req: &Request) -> Result<Self, Error> {
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
    pub fn pan(&self) -> &str {
        self.pan.as_str()
    }
    pub fn expirydate(&self) -> &str {
        self.expirydate.as_str()
    }
    pub fn _securitycode(&self) -> &str {
        self.securitycode.as_str()
    }

    fn set_requesttypedescription(mut self, value: &str) -> Result<Self, Error> {
        if !REQUEST_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid requesttypedescription {value}"
            )))
        } else {
            self.requesttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_accounttypedescription(mut self, value: &str) -> Result<Self, Error> {
        if !ACCOUNT_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid accounttypedescription {value}"
            )))
        } else {
            self.accounttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_paymenttypedescription(mut self, value: &str) -> Result<Self, Error> {
        if !PAYMENT_TYPES.contains(&value) {
            Err(Error::FieldError(format!(
                "Invalid paymenttypedescription {value}"
            )))
        } else {
            self.paymenttypedescription = value.to_owned();
            Ok(self)
        }
    }
    fn set_baseamount(mut self, value: u32) -> Result<Self, Error> {
        self.baseamount = value;
        Ok(self)
    }
    fn set_currencyiso3a(mut self, value: &str) -> Result<Self, Error> {
        self.currencyiso3a = value.to_owned();
        Ok(self)
    }
    fn set_pan(mut self, value: &str) -> Result<Self, Error> {
        self.pan = value.to_owned();
        Ok(self)
    }
    fn set_expirydate(mut self, value: &str) -> Result<Self, Error> {
        self.expirydate = value.to_owned();
        Ok(self)
    }
    fn set_securitycode(mut self, value: &str) -> Result<Self, Error> {
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
                pan: "".into(),
                expirydate: "".into(),
                securitycode: "".into(),
            }),
        )];
        for (op, value, exp) in tests {
            assert_eq!(op.set_requesttypedescription(value), exp);
        }
    }
}
