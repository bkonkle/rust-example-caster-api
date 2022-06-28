use async_graphql::{
    registry, ContextSelectionSet, InputType, InputValueError, InputValueResult, OutputType,
    Positioned, ServerResult, Value,
};
use async_graphql_parser::types::Field;
use fake::{Dummy, Faker};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;

/// A newtype enabling a `Dummy<Faker>` implementation for `serde_json::Value`
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Json(serde_json::Value);

impl Json {
    /// Create a new `Json` with a `Value`
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl Dummy<Faker> for Json {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
        Json(json!({}))
    }
}

#[async_trait::async_trait]
impl OutputType for Json {
    fn type_name() -> Cow<'static, str> {
        <serde_json::Value as OutputType>::type_name()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <serde_json::Value as OutputType>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        self.0.resolve(ctx, field).await
    }
}

impl InputType for Json {
    type RawValueType = <serde_json::Value as InputType>::RawValueType;

    fn type_name() -> Cow<'static, str> {
        <serde_json::Value as InputType>::type_name()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <serde_json::Value as InputType>::create_type_info(registry)
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        let value = value.unwrap_or_default();

        match value {
            Value::Null => Err(InputValueError::expected_type(value)),
            value => <serde_json::Value as InputType>::parse(Some(value))
                .map(Json::new)
                .map_err(InputValueError::propagate),
        }
    }

    fn to_value(&self) -> Value {
        <serde_json::Value as InputType>::to_value(&self.0)
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        <serde_json::Value as InputType>::as_raw_value(&self.0)
    }
}

impl From<Json> for serde_json::Value {
    fn from(value: Json) -> Self {
        value.0
    }
}

impl From<Json> for sea_orm::Value {
    fn from(json: Json) -> Self {
        if let Ok(value) = serde_json::to_value(&json.0) {
            return sea_orm::Value::Json(Some(Box::new(value)));
        }

        sea_orm::Value::Json(None)
    }
}

/// A newtype enabling a `Dummy<Faker>` implementation for `Option<serde_json::Value>`
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct JsonOption(Option<serde_json::Value>);

impl JsonOption {
    /// Create a new `JsonOption` with an `Option<Value>`
    pub fn new(opt: Option<serde_json::Value>) -> Self {
        Self(opt)
    }
}

impl Dummy<Faker> for JsonOption {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
        JsonOption(Some(json!({})))
    }
}

#[async_trait::async_trait]
impl OutputType for JsonOption {
    fn type_name() -> Cow<'static, str> {
        <Option<serde_json::Value> as OutputType>::type_name()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <Option<serde_json::Value> as OutputType>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        self.0.resolve(ctx, field).await
    }
}

impl InputType for JsonOption {
    type RawValueType = <Option<serde_json::Value> as InputType>::RawValueType;

    fn type_name() -> Cow<'static, str> {
        <Option<serde_json::Value> as InputType>::type_name()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <Option<serde_json::Value> as InputType>::create_type_info(registry)
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        let value = value.unwrap_or_default();

        match value {
            Value::Null => Err(InputValueError::expected_type(value)),
            value => <Option<serde_json::Value> as InputType>::parse(Some(value))
                .map(JsonOption::new)
                .map_err(InputValueError::propagate),
        }
    }

    fn to_value(&self) -> Value {
        <Option<serde_json::Value> as InputType>::to_value(&self.0)
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        <Option<serde_json::Value> as InputType>::as_raw_value(&self.0)
    }
}

impl From<JsonOption> for Option<serde_json::Value> {
    fn from(value: JsonOption) -> Self {
        value.0
    }
}

impl From<JsonOption> for sea_orm::Value {
    fn from(json: JsonOption) -> Self {
        if let Some(value) = json.0 {
            if let Ok(value) = serde_json::to_value(&value) {
                return sea_orm::Value::Json(Some(Box::new(value)));
            }
        }

        sea_orm::Value::Json(None)
    }
}
