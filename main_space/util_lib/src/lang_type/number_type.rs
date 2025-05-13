use super::lang_type_base::LangType;

pub trait NumberType: LangType {}

pub struct Int {
    pub value: i64,
}

impl LangType for Int {
    fn get_type_name(&self) -> &'static str {
        "Int"
    }
}

impl NumberType for Int {}

pub struct Float {
    pub value: f64,
}

impl LangType for Float {
    fn get_type_name(&self) -> &'static str {
        "Float"
    }
}

impl NumberType for Float {}
