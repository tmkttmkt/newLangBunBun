#![allow(unused_imports)]
pub mod lang_type_base;
pub mod number_type;
pub mod char_type;
pub mod array_type;

use lang_type_base::LangType;
use number_type::{Int, Float};
use array_type::Array;
use char_type::Char; // Charをインポート

// LangTypeEnumはすべての型をまとめた列挙型です。
pub enum LangTypeEnum {
    Int { value: Int, name: String },
    Float { value: Float, name: String },
    Char { value: Char, name: String }, // Charにnameを追加
    Array { value: Box<LangTypeEnum>, name: String }, // 配列にもnameを追加
}