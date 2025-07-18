use super::lang_type_base::LangType;

pub struct Array<T: LangType> {
    pub elements: Vec<T>,
}

impl<T: LangType> LangType for Array<T> {
    fn get_type_name(&self) -> &'static str {
        "Array"
    }
}
