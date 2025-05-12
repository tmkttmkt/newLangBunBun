use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

/// 指定されたファイルパスからファイルを読み取り、処理関数を適用します。
/// 処理関数はコンパイル、アセンブル、またはその他の変換を行うことができます。
pub fn process_file<F>(file_path: &str, process: F) -> io::Result<String>
where
    F: Fn(&str) -> String,
{
    // ファイルパスを絶対パスに変換
    let absolute_path = resolve_absolute_path(file_path)?;

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

/// 処理結果を入力ファイルと同じフォルダに `<元のファイル名>.nasm` として出力します。
pub fn write_result_to_file(file_path: &str, result: &str,extension: &str) -> io::Result<()> {
    let input_path = resolve_absolute_path(file_path)?;
    // 入力ファイルの親ディレクトリを取得
    if let Some(parent_dir) = input_path.parent() {
        // 元のファイル名から拡張子を取り除く
        let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();

        // 出力ファイル名を生成
        let output_path = parent_dir.join(format!("{}.{}", file_stem,extension));

        // 結果をファイルに書き込む
        fs::write(&output_path, result).map_err(|e| {
            Error::new(
                e.kind(),
                format!("Failed to write to file {}: {}", output_path.display(), e),
            )
        })?;

        println!("Result written to: {}", output_path.display());
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Could not determine parent directory for input file.",
        ))
    }
}

/// 相対パスを2つ上位のディレクトリを基準にした絶対パスに変換するプライベート関数
fn resolve_absolute_path(file_path: &str) -> io::Result<PathBuf> {
    if Path::new(file_path).is_relative() {
        let mut base_path = std::env::current_dir()?.join("..");
        base_path.push(file_path);
        Ok(base_path)
    } else {
        Ok(PathBuf::from(file_path))
    }
}

/// ヘルパー関数（デバッグ用）
pub fn helper_function() {
    println!("これはユーティリティ関数です。");
}
