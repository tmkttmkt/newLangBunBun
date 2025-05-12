pub trait LangType {
    fn get_type_name(&self) -> &'static str;
}

// CollectionTypeトレイトは、コレクション型の共通インターフェースを定義します。
// LangTypeを継承しているため、コレクション型もLangTypeの一部となります。
pub trait CollectionType: LangType {}
