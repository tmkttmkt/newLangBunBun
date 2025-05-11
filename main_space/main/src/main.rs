use linux_codegen;
use util_lib::process_file;
use std::env;

fn main() {
    // 実行時引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    // ファイルを読み取り、translate_to_linux_nasmで変換
    match process_file(file_path, linux_codegen::translate_to_linux_nasm) {
        Ok(nasm_code) => {
            println!("Generated NASM Code:\n{}", nasm_code);
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e);
        }
    }
}