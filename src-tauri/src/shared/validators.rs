// En crate::shared::validators.rs (o donde prefieras)

use std::cell::LazyCell;

use regex::Regex;

// Regex simple que permite números, espacios, guiones y el signo + al inicio
pub static PHONE_REGEX: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"^\+?[0-9\s\-]{6,19}$").unwrap());
