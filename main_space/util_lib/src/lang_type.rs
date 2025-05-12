pub mod types {
    pub mod lang_type_base;
    pub mod number_type;
    pub mod string_type;
    pub mod array_type;
}

use types::lang_type_base::LangType;
use types::number_type::{Int, Float};
use types::string_type::Str;
use types::array_type::Array;

// LangTypeEnumはすべての型をまとめた列挙型です。
pub enum LangTypeEnum {
    Int(Int),
    Float(Float),
    Str(Str),
    Array(Box<LangTypeEnum>), // 配列の要素もLangTypeEnumで表現
}

impl LangType for LangTypeEnum {
    fn get_type_name(&self) -> &'static str {
        match self {
            LangTypeEnum::Int(_) => "Int",
            LangTypeEnum::Float(_) => "Float",
            LangTypeEnum::Str(_) => "String",
            LangTypeEnum::Array(_) => "Array",
        }
    }
}
