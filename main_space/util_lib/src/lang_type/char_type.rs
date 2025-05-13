use super::lang_type_base::LangType;

pub struct Char {
    pub value: char,
}

impl LangType for Char {
    fn get_type_name(&self) -> &'static str {
        "Char"
    }
}
