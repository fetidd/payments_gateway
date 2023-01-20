use crate::errors::GwResult;

pub fn mask_pan(pan: &str) -> String {
    let mut masked_pan = String::from(pan);
    masked_pan.replace_range(4..masked_pan.len() - 4, &"*".repeat(masked_pan.len() - 8));
    masked_pan
}
