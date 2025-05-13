pub trait LangType {
    fn get_type_name(&self) -> &'static str;
}

pub struct Int {
    pub value: i64,
}

impl LangType for Int {
    fn get_type_name(&self) -> &'static str {
        "Int"
    }
}

pub struct Float {
    pub value: f64,
}

impl LangType for Float {
    fn get_type_name(&self) -> &'static str {
        "Float"
    }
}

pub struct Str {
    pub value: String,
}

impl LangType for Str {
    fn get_type_name(&self) -> &'static str {
        "String"
    }
}

pub struct Array<T: LangType> {
    pub elements: Vec<T>,
}

impl<T: LangType> LangType for Array<T> {
    fn get_type_name(&self) -> &'static str {
        "Array"
    }
}
