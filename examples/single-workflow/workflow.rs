pub use def::WorkflowDef;
use def::*;

impl WorkflowDef {}

/// def module contains workflow's yaml definition
mod def {
    use anyhow::Result;
    use serde::de::{Error, MapAccess, Visitor};
    use serde::{Deserialize, Deserializer};
    use std::fmt::{Debug, Formatter};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkflowDef {
        id: String,
        name: String,
        actions: Vec<ActionDef>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ActionDef {
        id: Option<String>,
        #[serde(deserialize_with = "deserialize_skip", default)]
        skip: Option<InputValueDef<bool>>,
        #[serde(flatten)]
        action: ActionTypeDef,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    pub enum ActionTypeDef {
        Add {
            input: Vec<InputDef>,
        },
        Minus {
            input: Vec<InputDefWithName>,
        },
        #[serde(rename = "@parallel")]
        Parallel {
            branches: Vec<ActionDef>,
            result: Option<InputValueDef<AnyValue>>,
        },
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InputDef {
        #[serde(flatten)]
        value: InputValueDef<AnyValue>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InputDefWithName {
        name: String,
        #[serde(flatten)]
        value: InputValueDef<AnyValue>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    pub enum InputValueDef<T> {
        Raw { value: T },
        Expression { expression: String },
    }

    impl<T> From<T> for InputValueDef<T> {
        fn from(value: T) -> Self {
            Self::Raw { value }
        }
    }

    fn deserialize_input_value_def_bool<'de, D>(
        deserializer: D,
    ) -> Result<InputValueDef<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoolOrMap;

        impl<'de> Visitor<'de> for BoolOrMap {
            type Value = InputValueDef<bool>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("bool or map")
            }

            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(InputValueDef::Raw { value: v })
            }

            fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(BoolOrMap)
    }

    fn deserialize_skip<'de, D>(deserializer: D) -> Result<Option<InputValueDef<bool>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Value(
            #[serde(deserialize_with = "deserialize_input_value_def_bool")] InputValueDef<bool>,
        );

        Option::<Value>::deserialize(deserializer).map(|v| v.map(|v| v.0))
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum AnyValue {
        String(String),
        Integer(i64),
        Number(f64),
        Boolean(bool),
    }

    impl WorkflowDef {
        pub fn read_from_yaml_file(path: &str) -> Result<Self> {
            let content = std::fs::read_to_string(path)?;
            let workflow_def: WorkflowDef = serde_yaml::from_str(&content)?;
            Ok(workflow_def)
        }
    }
}
