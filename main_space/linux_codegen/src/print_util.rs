use std::fs;

/// print.asmの内容を文字列として返す関数
pub fn nasm_print_function() -> String {
    // print.asmのパス（必要に応じてパスを調整してください）
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nasm/print.asm");
    fs::read_to_string(path).expect("print.asmの読み込みに失敗しました")
}
