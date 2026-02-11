use super::{BasicGenerator, Generate};
use crate::cbor_helpers::{cbor_map, cbor_serialize};

pub fn unit() -> JustGenerator<()> {
    just(())
}

pub struct JustGenerator<T> {
    value: T,
}

impl<T: Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static> Generate<T>
    for JustGenerator<T>
{
    fn generate(&self) -> T {
        self.value.clone()
    }

    fn as_basic(&self) -> Option<BasicGenerator<T>> {
        Some(BasicGenerator::new(
            cbor_map! {"const" => cbor_serialize(&self.value)},
        ))
    }
}

pub fn just<T: Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static>(
    value: T,
) -> JustGenerator<T> {
    JustGenerator { value }
}

pub struct JustAnyGenerator<T> {
    value: T,
}

impl<T: Clone + Send + Sync> Generate<T> for JustAnyGenerator<T> {
    fn generate(&self) -> T {
        self.value.clone()
    }
}
pub fn just_any<T: Clone + Send + Sync>(value: T) -> JustAnyGenerator<T> {
    JustAnyGenerator { value }
}

pub struct BoolGenerator;

impl Generate<bool> for BoolGenerator {
    fn generate(&self) -> bool {
        self.as_basic().unwrap().generate()
    }

    fn as_basic(&self) -> Option<BasicGenerator<bool>> {
        Some(BasicGenerator::new(cbor_map! {"type" => "boolean"}))
    }
}

pub fn booleans() -> BoolGenerator {
    BoolGenerator
}
