use super::{BasicGenerator, Generate};
use crate::cbor_helpers::{cbor_map, map_insert};
use ciborium::Value;

pub struct TextGenerator {
    min_size: usize,
    max_size: Option<usize>,
    cached_basic: Option<BasicGenerator<String>>,
}

fn compute_text_basic(min_size: usize, max_size: Option<usize>) -> Option<BasicGenerator<String>> {
    let mut schema = cbor_map! {
        "type" => "string",
        "min_size" => min_size as u64
    };

    if let Some(max) = max_size {
        map_insert(&mut schema, "max_size", Value::from(max as u64));
    }

    Some(BasicGenerator::new(schema))
}

impl TextGenerator {
    pub fn with_min_size(mut self, min: usize) -> Self {
        self.min_size = min;
        self.cached_basic = compute_text_basic(self.min_size, self.max_size);
        self
    }

    pub fn with_max_size(mut self, max: usize) -> Self {
        self.max_size = Some(max);
        self.cached_basic = compute_text_basic(self.min_size, self.max_size);
        self
    }
}

impl Generate<String> for TextGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn text() -> TextGenerator {
    TextGenerator {
        min_size: 0,
        max_size: None,
        cached_basic: compute_text_basic(0, None),
    }
}

pub struct RegexGenerator {
    pattern: String,
    fullmatch: bool,
    cached_basic: Option<BasicGenerator<String>>,
}

fn compute_regex_basic(pattern: &str, fullmatch: bool) -> Option<BasicGenerator<String>> {
    Some(BasicGenerator::new(cbor_map! {
        "type" => "regex",
        "pattern" => pattern,
        "fullmatch" => fullmatch
    }))
}

impl RegexGenerator {
    /// Require the entire string to match the pattern, not just contain a match.
    pub fn fullmatch(mut self) -> Self {
        self.fullmatch = true;
        self.cached_basic = compute_regex_basic(&self.pattern, self.fullmatch);
        self
    }
}

impl Generate<String> for RegexGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

/// Generate strings that contain a match for the given regex pattern.
///
/// Use `.fullmatch()` to require the entire string to match.
pub fn from_regex(pattern: &str) -> RegexGenerator {
    let fullmatch = false;
    RegexGenerator {
        cached_basic: compute_regex_basic(pattern, fullmatch),
        pattern: pattern.to_string(),
        fullmatch,
    }
}
