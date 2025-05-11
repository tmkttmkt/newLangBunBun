use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

/// 指定されたファイルパスからファイルを読み取り、処理関数を適用します。
/// 処理関数はコンパイル、アセンブル、またはその他の変換を行うことができます。
pub fn process_file<F>(file_path: &str, process: F) -> io::Result<String>
where
    F: Fn(&str) -> String,
{
    // 相対パスの場合、2つ上位のディレクトリを基準にする
    let absolute_path = if Path::new(file_path).is_relative() {
        let mut base_path = std::env::current_dir()?.join("..").join("..");
        base_path.push(file_path);
        base_path
    } else {
        PathBuf::from(file_path)
    };

    // ファイルの内容を読み取る
    let content = fs::read_to_string(&absolute_path).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            Error::new(
                ErrorKind::NotFound,
                format!("File not found: {}", absolute_path.display()),
            )
        } else {
            e
        }
    })?;
    
    // 読み取った内容に処理関数を適用する
    let result = process(&content);
    
    // 処理結果を返す
    Ok(result)
}

/// ヘルパー関数（デバッグ用）
pub fn helper_function() {
    println!("これはユーティリティ関数です。");
}
