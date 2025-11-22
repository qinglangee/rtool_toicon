use std::fs::File;
use image::ImageReader;
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};

/// 将 PNG 文件转换为 ICO 文件
/// 但是有些不带透明通道的 PNG 图片转换后的 ICO 文件在 Windows 上显示会失败
pub fn png2ico(png_file: &str, ico_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 读取 PNG 图片
    let img = ImageReader::open(png_file)?.decode()?;
    let rgba_img = img.to_rgba8();

    // 2. 创建 ICO 图像
    let icon_image = IconImage::from_rgba_data(rgba_img.width(), rgba_img.height(), rgba_img.into_raw());

    // 3. 创建 ICO 目录并添加图像
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);

    // 4. 写入 ICO 文件
    icon_dir.write(File::create(ico_file)?)?;

    Ok(())
}