use super::{BasicGenerator, Generate};
use crate::cbor_helpers::{cbor_array, cbor_map};

pub struct EmailGenerator {
    cached_basic: Option<BasicGenerator<String>>,
}

impl Generate<String> for EmailGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn emails() -> EmailGenerator {
    EmailGenerator {
        cached_basic: Some(BasicGenerator::new(cbor_map! {"type" => "email"})),
    }
}

pub struct UrlGenerator {
    cached_basic: Option<BasicGenerator<String>>,
}

impl Generate<String> for UrlGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn urls() -> UrlGenerator {
    UrlGenerator {
        cached_basic: Some(BasicGenerator::new(cbor_map! {"type" => "url"})),
    }
}

pub struct DomainGenerator {
    max_length: usize,
    cached_basic: Option<BasicGenerator<String>>,
}

impl DomainGenerator {
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self.cached_basic = Some(compute_domain_basic(self.max_length));
        self
    }
}

impl Generate<String> for DomainGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

fn compute_domain_basic(max_length: usize) -> BasicGenerator<String> {
    BasicGenerator::new(cbor_map! {
        "type" => "domain",
        "max_length" => max_length as u64
    })
}

pub fn domains() -> DomainGenerator {
    DomainGenerator {
        max_length: 255,
        cached_basic: Some(compute_domain_basic(255)),
    }
}

#[derive(Clone, Copy)]
pub enum IpVersion {
    V4,
    V6,
}

pub struct IpAddressGenerator {
    version: Option<IpVersion>,
    cached_basic: Option<BasicGenerator<String>>,
}

impl IpAddressGenerator {
    pub fn v4(mut self) -> Self {
        self.version = Some(IpVersion::V4);
        self.cached_basic = Some(compute_ip_basic(self.version));
        self
    }

    pub fn v6(mut self) -> Self {
        self.version = Some(IpVersion::V6);
        self.cached_basic = Some(compute_ip_basic(self.version));
        self
    }
}

impl Generate<String> for IpAddressGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

fn compute_ip_basic(version: Option<IpVersion>) -> BasicGenerator<String> {
    match version {
        Some(IpVersion::V4) => BasicGenerator::new(cbor_map! {"type" => "ipv4"}),
        Some(IpVersion::V6) => BasicGenerator::new(cbor_map! {"type" => "ipv6"}),
        None => BasicGenerator::new(cbor_map! {
            "one_of" => cbor_array![
                cbor_map!{"type" => "ipv4"},
                cbor_map!{"type" => "ipv6"}
            ]
        }),
    }
}

pub fn ip_addresses() -> IpAddressGenerator {
    IpAddressGenerator {
        version: None,
        cached_basic: Some(compute_ip_basic(None)),
    }
}

pub struct DateGenerator {
    cached_basic: Option<BasicGenerator<String>>,
}

impl Generate<String> for DateGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn dates() -> DateGenerator {
    DateGenerator {
        cached_basic: Some(BasicGenerator::new(cbor_map! {"type" => "date"})),
    }
}

pub struct TimeGenerator {
    cached_basic: Option<BasicGenerator<String>>,
}

impl Generate<String> for TimeGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn times() -> TimeGenerator {
    TimeGenerator {
        cached_basic: Some(BasicGenerator::new(cbor_map! {"type" => "time"})),
    }
}

pub struct DateTimeGenerator {
    cached_basic: Option<BasicGenerator<String>>,
}

impl Generate<String> for DateTimeGenerator {
    fn generate(&self) -> String {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<String>> {
        self.cached_basic.clone()
    }
}

pub fn datetimes() -> DateTimeGenerator {
    DateTimeGenerator {
        cached_basic: Some(BasicGenerator::new(cbor_map! {"type" => "datetime"})),
    }
}
