use std::time::Duration;
use indexmap::IndexMap;

use crate::model::meta_object::MetaField;

pub mod meta_object;
pub mod meta_action;
pub mod meta_config;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EntryDate {
    created: Duration,
    updated: Duration,
    due_date: Duration,
    resolution_date: Duration,
}

impl Default for EntryDate {
    fn default() -> Self {
        EntryDate {
            created: Default::default(),
            updated: Default::default(),
            due_date: Default::default(),
            resolution_date: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Default for Author {
    fn default() -> Self {
        Author {
            name: "".to_string(),
            email: "".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CustomType {
    pub fields: IndexMap<String, MetaField>,
}

impl CustomType {
    pub fn from(map: IndexMap<String, String>) -> CustomType {
        let mut fields = IndexMap::new();
        for (key, value) in map {
            fields.insert(key, Self::parse_field_type(value));
        }

        CustomType {
            fields,
        }
    }

    pub fn field(&self, text: &str) -> Option<&MetaField> {
        self.fields.get(text)
    }

    fn parse_field_type(value: String) -> MetaField {
        let field = match value.as_str() {
            "Text" => {
                MetaField::Text(value)
            }
            "Title" => {
                MetaField::Title(value)
            }
            "Tagged" => {
                // todo
                let tags = vec![];
                MetaField::Tagged(tags)
            }
            "Author" => {
                let author = Author::default();
                MetaField::Author(author)
            }
            "EntryDate" => {
                let date = EntryDate::default();
                MetaField::EntryDate(date)
            }
            _ => {
                MetaField::Unknown(value)
            }
        };
        field
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::model::CustomType;
    use crate::model::meta_object::MetaField;

    #[test]
    fn custom_type() {
        let mut map = HashMap::new();
        map.insert("title".to_string(), "Title".to_string());

        let custom_type = CustomType::from(map);

        let option = custom_type.fields.get("title").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }

    #[test]
    fn keys_in_order() {
        let mut map = HashMap::new();
        map.insert("title".to_string(), "Title".to_string());
        map.insert("content".to_string(), "content".to_string());

        let custom_type = CustomType::from(map);

        assert_eq!("title", custom_type.keys[0]);
        assert_eq!("content", custom_type.keys[1]);
    }
}
