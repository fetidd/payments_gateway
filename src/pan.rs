use std::fmt::Debug;

use crate::{useful, errors::{GwResult, Error}, constant::*};

#[derive(Clone, PartialEq)]
pub struct Pan {
    pan: String
}

impl Pan {
    pub fn new(pan: &str) -> GwResult<Pan> {
        if pan.len() < MIN_PAN_LENGTH || pan.len() > MAX_PAN_LENGTH {
            Err(Error::ValidationError(format!("pan is invalid length: {}", pan.len())))
        } else {
            Ok(Pan{pan: pan.to_string()})
        }
    }

    pub fn get_masked(&self) -> String {
        useful::mask_pan(&self.pan)
    }
}

impl Debug for Pan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_masked())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tests = [
            ("", Err(Error::ValidationError("pan is invalid length: 0".into()))),
            ("3400000000000000", Ok(Pan { pan: String::from("3400000000000000") })),
            ("3400000000000000123456789", Err(Error::ValidationError("pan is invalid length: 25".into()))),
        ];
        for (pan, exp) in tests {
            assert_eq!(exp, Pan::new(pan));
        }
    }

    #[test]
    fn test_mask_pan() {
        let tests = [
            ("3400000000000000", "3400********0000"),
        ];
        for (pan, exp) in tests {
            let pan = Pan::new(pan);
            assert_eq!(exp, pan.unwrap().get_masked());
        }
    }
}
