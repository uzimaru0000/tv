use std::fmt::Display;

use crate::utils::Transpose;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Json {
    Object(serde_json::Map<String, serde_json::Value>),
    Value(serde_json::Value),
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    None,
    Left,
    Center,
    Right,
}

impl Align {
    pub fn new(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "l" => Self::Left,
            "left" => Self::Left,
            "c" => Self::Center,
            "center" => Self::Center,
            "r" => Self::Right,
            "right" => Self::Right,
            _ => Self::None,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    data: Vec<Json>,
    sort_key: Option<String>,
    is_plane: bool,
    align: Align,
}

impl Data {
    pub fn from(s: &str) -> Result<Self> {
        serde_json::from_str::<Vec<Json>>(s)
            .map(|x| Self {
                data: x,
                sort_key: None,
                is_plane: false,
                align: Align::None,
            })
            .context("unsupported format")
    }

    pub fn set_sort_key(&mut self, s: Option<String>) -> &mut Self {
        self.sort_key = s;
        self
    }

    pub fn set_is_plane(&mut self, p: bool) -> &mut Self {
        self.is_plane = p;
        self
    }

    pub fn set_align(&mut self, a: Align) -> &mut Self {
        self.align = a;
        self
    }

    fn keys(&self) -> Vec<String> {
        self.data
            .get(0)
            .map(|x| match x {
                Json::Object(obj) => obj.keys().map(|x| x.clone()).collect(),
                _ => vec![String::new()],
            })
            .unwrap_or_default()
    }

    fn values(&self) -> Vec<Vec<String>> {
        let keys = self.keys();

        let data = if let Some(key) = self.sort_key.clone() {
            let mut data = self.data.clone();
            data.sort_by_cached_key(|x| match x {
                Json::Object(obj) => obj
                    .get(&key)
                    .as_deref()
                    .unwrap_or(&serde_json::Value::default())
                    .to_string(),
                Json::Value(_) => serde_json::Value::default().to_string(),
            });
            data
        } else {
            self.data.clone()
        };

        data.iter()
            .map(|x| match x {
                Json::Object(obj) => keys
                    .clone()
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

    fn pads(&self) -> Vec<usize> {
        let keys = self.keys();
        self.values()
            .iter()
            .transpose()
            .zip(keys)
            .map(|(mut xs, k)| {
                xs.push(&k);
                xs.into_iter().map(|x| x.width()).max().unwrap_or_default()
            })
            .collect::<Vec<_>>()
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.keys();
        let values = self.values();
        let pads = self.pads();
        let separator = if self.is_plane { "\t" } else { "|" };
        let frame = if self.is_plane { "" } else { "|" };

        let title_str = keys
            .iter()
            .zip(pads.clone())
            .map(|(x, pad)| cell_view(x, pad, &self.align))
            .collect::<Vec<_>>()
            .join(separator);
        write!(f, "{}{}{}\n", frame, title_str, frame)?;

        if !self.is_plane {
            let border = pads
                .clone()
                .iter()
                .map(|&x| md_table_align(x, &self.align))
                .collect::<Vec<_>>()
                .join(separator);
            write!(f, "{}{}{}\n", frame, border, frame)?;
        }

        values
            .into_iter()
            .map(|xs| {
                xs.iter()
                    .zip(pads.clone())
                    .map(|(x, pad)| cell_view(x, pad, &self.align))
                    .collect::<Vec<_>>()
                    .join(separator)
            })
            .try_for_each(|x| write!(f, "{}{}{}\n", frame, x, frame))
    }
}

fn md_table_align(width: usize, align: &Align) -> String {
    match align {
        Align::None => "-".repeat(width),
        Align::Left => format!(":{}", "-".repeat(width - 1)),
        Align::Center => format!(":{}:", "-".repeat(width - 2)),
        Align::Right => format!("{}:", "-".repeat(width - 1)),
    }
}

fn cell_view(v: &String, width: usize, align: &Align) -> String {
    match align {
        Align::None | Align::Right => format!("{}{}", " ".repeat(width - v.width()), v),
        Align::Left => format!("{}{}", v, " ".repeat(width - v.width())),
        Align::Center => {
            let pad = (width - v.width()) as f32 / 2.0;
            let left_pad = pad.ceil() as usize;
            let right_pad = pad.floor() as usize;
            format!("{}{}{}", " ".repeat(left_pad), v, " ".repeat(right_pad))
        }
    }
}

#[cfg(test)]
mod tests {
    const ARRAY_OBJECT: &'static str = r#"[
            {
                "name": "test",
                "age": 10,
                "lang": "ja"
            },
            {
                "name": "uzimaru",
                "age": 23,
                "lang": "ja"
            },
            {
                "name": "hogehoge",
                "age": 21,
                "lang": "en"
            },
            {
                "name": "hugehuge",
                "age": 32,
                "lang": "en"
            }
        ]"#;

    const ARRAY_VALUE: &'static str = "[1, 2, 3, 4]";

    #[test]
    fn test_create_data_from_str_to_array_object() {
        let data = super::Data::from(ARRAY_OBJECT);
        assert!(data.is_ok());
    }

    #[test]
    fn test_create_data_from_str_to_array_value() {
        let data = super::Data::from(ARRAY_VALUE);
        assert!(data.is_ok());
    }

    #[test]
    fn test_display_array_object() {
        let actual = super::Data::from(ARRAY_OBJECT)
            .map(|x| format!("{}", x))
            .unwrap();

        let expected = r#"|age|lang|    name|
|---|----|--------|
| 10|  ja|    test|
| 23|  ja| uzimaru|
| 21|  en|hogehoge|
| 32|  en|hugehuge|
"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_display_array_value() {
        let actual = super::Data::from(ARRAY_VALUE)
            .map(|x| format!("{}", x))
            .unwrap();

        let expected = r#"| |
|-|
|1|
|2|
|3|
|4|
"#;
        assert_eq!(actual, expected);
    }
}
