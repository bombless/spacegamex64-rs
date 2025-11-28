use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 输出目录，用于存放生成的文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_data.rs");

    
    let content = fs::read_to_string("graphics/loadImg.inc").unwrap();
    let mut result = Vec::new();

    for line in content.lines() {

        // 提取 "db " 之后的内容
        let after_db = if let Some(stripped) = line.strip_prefix("db ") {
            stripped
        } else {
            continue; // 如果没有 "db " 前缀则跳过
        };

        // 按逗号分割
        for item in after_db.split(',') {
            // println!("Processing item: {}", item);
            let trimmed = item.trim();

            // 确保字符串至少有 4 个字符（例如 '0x12'）
            if trimmed.len() >= 3 {
                // 取第2和第3个字符（索引1和1）
                let c1 = trimmed.chars().nth(1).unwrap();
                let c2 = trimmed.chars().nth(2).unwrap();

                // 转换为数字（假设是十六进制字符）
                if let (Some(val1), Some(val2)) = (c1.to_digit(16), c2.to_digit(16)) {
                    let byte_val = val1 * 16 + val2;
                    result.push(byte_val as u8);
                }
            }
        }

    }
    result.extend(vec![0; 1280 * 1024 * 4 - result.len()]);

    // 将数据格式化为 Rust 代码
    let content = format!(
        "pub static GENERATED_DATA: &'static [u8; 1280 * 1024 * 4] = &{:?};\n",
        &result[..1280 * 1024 * 4]
    );

    // 写入文件
    fs::write(&dest_path, content)
        .expect("Failed to write generated data file");

    // 可选：如果源数据会变，可以标记触发重新构建
    println!("cargo:rerun-if-changed=build.rs");
    // 或者监听某个配置文件：
    // println!("cargo:rerun-if-changed=src/data.bin");
}
