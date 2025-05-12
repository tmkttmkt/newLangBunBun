use super::lang_type_base::{LangType, CollectionType};

pub struct Str {
    pub value: String,
}

impl LangType for Str {
    fn get_type_name(&self) -> &'static str {
        "String"
    }
}

impl CollectionType for Str {}
