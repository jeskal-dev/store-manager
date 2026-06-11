use std::sync::LazyLock;

use regex::Regex;

pub static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+?[0-9\s\-]{6,19}$").unwrap());
