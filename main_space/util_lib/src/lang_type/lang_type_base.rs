pub trait LangType {
    fn get_type_name(&self) -> &'static str;
}
