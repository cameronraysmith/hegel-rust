use super::TestCaseData;
use ciborium::Value;
use std::marker::PhantomData;

/// A bundled schema + parse function for schema-based generation.
///
/// The lifetime `'a` ties the BasicGenerator to the generator that created it.
/// `T: 'a` is required because the parse closure returns `T`.
pub struct BasicGenerator<'a, T> {
    schema: Value,
    parse: Box<dyn Fn(Value) -> T + Send + Sync + 'a>,
    _phantom: PhantomData<fn() -> T>,
}

impl<'a, T: 'a> BasicGenerator<'a, T> {
    /// Create a new BasicGenerator from a schema and parse function.
    pub fn new<F: Fn(Value) -> T + Send + Sync + 'a>(schema: Value, f: F) -> Self {
        BasicGenerator {
            schema,
            parse: Box::new(f),
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the schema.
    pub fn schema(&self) -> &Value {
        &self.schema
    }

    /// Parse a raw CBOR value into the generated type.
    pub fn parse_raw(&self, raw: Value) -> T {
        (self.parse)(raw)
    }

    /// Generate a value by sending the schema to the server and parsing the response.
    ///
    /// This is a convenience for `self.parse_raw(data.generate_raw(self.schema()))`.
    pub fn do_draw(&self, data: &TestCaseData) -> T {
        self.parse_raw(data.generate_raw(self.schema()))
    }

    /// Transform the output type by composing a function with the parse.
    ///
    /// The resulting BasicGenerator shares the same schema but applies `f`
    /// after parsing.
    pub fn map<U: 'a, F: Fn(T) -> U + Send + Sync + 'a>(self, f: F) -> BasicGenerator<'a, U> {
        let old_parse = self.parse;
        BasicGenerator {
            schema: self.schema,
            parse: Box::new(move |raw| f(old_parse(raw))),
            _phantom: PhantomData,
        }
    }
}
