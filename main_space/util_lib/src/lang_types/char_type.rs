use super::lang_type_base::LangType;

pub struct Char {
    pub name: String, // 変数名
    pub value: char,
}

impl LangType for Char {
    fn get_type_name(&self) -> &'static str {
        "Char"
    }
}
