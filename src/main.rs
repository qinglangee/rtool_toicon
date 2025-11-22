
use std::env;
use std::path::Path;

use toicon::png2ico;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取两个命令行参数: 输入 PNG 文件路径和输出 ICO 文件路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <输入 PNG 文件> [输出 ICO 文件]", args[0]);
        return Ok(());
    }
    let png_file = &args[1];
    let ico_file = if args.len() >= 3 {
        &args[2]
    } else {
        let path = Path::new(png_file);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        &format!("{}.ico", stem)
    };


    match png2ico(&png_file, &ico_file) {
        Ok(_) => println!("转换成功: {}", &ico_file),
        Err(e) => {
            eprintln!("转换失败: {}", e);
            return Err(e);
        }
    }

    Ok(())
}