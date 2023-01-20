pub static REQUEST_TYPES: [&str; 2] = ["AUTH", "REFUND"];
pub static ACCOUNT_TYPES: [&str; 3] = ["ECOM", "POS", "MOTO"];
pub static PAYMENT_TYPES: [&str; 3] = ["VISA", "MASTERCARD", "AMEX"];

pub static MAX_PAN_LENGTH: usize = 19;
pub static MIN_PAN_LENGTH: usize = 14;
