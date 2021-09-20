use std::collections::HashMap;

use crate::{
    table::{cell::Cell, Table},
    utils::multi_lines,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Json {
    Object(serde_json::Map<String, serde_json::Value>),
    Value(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct Data {
    data: Vec<Json>,
    sort_key: Option<String>,
}

impl Data {
    fn new(data: Vec<Json>) -> Self {
        Self {
            data,
            sort_key: None,
        }
    }

    pub fn from(s: &str) -> Result<Self> {
        serde_json::from_str::<Vec<Json>>(s)
            .map(Self::new)
            .context("unsupported format")
    }

    pub fn set_sort_key(&mut self, s: Option<String>) -> &mut Self {
        self.sort_key = s;
        self
    }

    fn keys(&self) -> Vec<String> {
        self.data
            .get(0)
            .map(|x| match x {
                Json::Object(obj) => obj.keys().map(|x| x.clone()).collect(),
                _ => vec![],
            })
            .unwrap_or_default()
    }

    fn sorted_data(&self) -> Vec<Json> {
        let sort_keys = self
            .sort_key
            .clone()
            .map(|x| x.split(".").map(String::from).collect::<Vec<_>>());

        if let Some(keys) = sort_keys {
            let mut data = self.data.clone();
            data.sort_by_cached_key(|x| {
                let val = keys.iter().fold(x.clone(), |val, key| match val {
                    Json::Object(obj) => obj
                        .get(key)
                        .map(|v| match v {
                            serde_json::Value::Object(o) => Json::Object(o.clone()),
                            _ => Json::Value(v.clone()),
                        })
                        .unwrap_or(Json::Value(serde_json::Value::default())),
                    _ => val,
                });

                match val {
                    Json::Object(_) => String::new(),
                    // NOTE: Numbers are sorted by strings, so fill them with 0s.
                    Json::Value(serde_json::Value::Number(n)) => format!("{: >099}", n),
                    Json::Value(v) => v.to_string(),
                }
            });
            data
        } else {
            self.data.clone()
        }
    }

    fn value_strings(&self) -> Vec<Vec<String>> {
        self.sorted_data()
            .iter()
            .map(|x| match x {
                Json::Object(obj) => self
                    .keys()
                    .iter()
                    .map(|k| obj.get(k.as_str()).map(|x| x.clone()))
                    .collect::<Vec<_>>(),
                Json::Value(serde_json::Value::Array(arr)) => {
                    arr.clone().iter().map(|x| Some(x.clone())).collect()
                }
                Json::Value(val) => vec![Some(val.clone())],
            })
            .map(|x| {
                x.iter()
                    .map(|x| match x {
                        Some(serde_json::Value::String(s)) => String::from(s),
                        Some(serde_json::Value::Bool(b)) => b.to_string(),
                        Some(serde_json::Value::Number(n)) => n.to_string(),
                        Some(serde_json::Value::Null) => String::from("null"),
                        None => String::from("undefined"),
                        _ => String::from("..."),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn nested_fields(&self) -> Vec<(String, Self)> {
        let nested_fields = {
            let data = self.sorted_data();
            let filtered_data = data.iter().filter_map(|x| match x {
                Json::Object(obj) => Some(obj),
                _ => None,
            });
            let nested_fields = filtered_data.map(|x| {
                x.keys()
                    .zip(x.values())
                    .filter(|(_, x)| match x {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => true,
                        _ => false,
                    })
                    .collect::<Vec<_>>()
            });
            let mut map: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
            nested_fields.for_each(|xs| {
                xs.into_iter().for_each(|(k, v)| {
                    let mut vec = if let Some(vec) = map.get(k) {
                        vec.clone()
                    } else {
                        Vec::new()
                    };

                    vec.push(v.clone());
                    map.insert(k.clone(), vec);
                });
            });
            map
        };

        nested_fields
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    Self::new(
                        v.iter()
                            .map(|x| match x {
                                serde_json::Value::Object(o) => Json::Object(o.clone()),
                                _ => Json::Value(x.clone()),
                            })
                            .collect(),
                    ),
                )
            })
            .collect()
    }
}

impl Into<Table<String>> for Data {
    fn into(self) -> Table<String> {
        let mut table = Table::new();
        let keys = self.keys();
        let values = multi_lines(self.value_strings());

        if !keys.is_empty() {
            let title = keys.into_iter().map(|x| Cell::new(x)).collect::<Vec<_>>();
            table.set_header(Some(title));
        }
        values
            .into_iter()
            .map(|xs| xs.into_iter().map(Cell::new).collect::<Vec<_>>())
            .for_each(|row| table.push_row(row));

        table
    }
}
