use std::path::{Path, PathBuf};

use image::{imageops::FilterType, GenericImageView, ImageBuffer};

#[derive(Debug, Clone)]
pub enum PaperSize {
    // Custom(f64, f64),
    A3,
    A4,
    A5,
}

impl PaperSize {
    pub fn size(&self, is_rotate: bool) -> (f64, f64) {
        if is_rotate {
            self.rotate()
        } else {
            match self {
                // PaperSize::Custom(w, h) => (*w, *h),
                PaperSize::A3 => (297.0, 420.0),
                PaperSize::A4 => (210.0, 297.0),
                PaperSize::A5 => (148.0, 210.0),
            }
        }
    }

    fn rotate(&self) -> (f64, f64) {
        match self {
            // PaperSize::Custom(w, h) => (*w, *h),
            PaperSize::A3 => (420.0, 297.0),
            PaperSize::A4 => (297.0, 210.0),
            PaperSize::A5 => (210.0, 148.0),
        }
    }

    pub fn vec_new() -> Vec<Self> {
        vec![PaperSize::A5, PaperSize::A4, PaperSize::A3]
    }

    // pub fn custom(w: f64, h: f64) -> Self {
    //     PaperSize::Custom(w, h)
    // }
}

impl std::fmt::Display for PaperSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // PaperSize::Custom(w, h) => write!(f, "{}mm x {}mm", w, h),
            PaperSize::A5 => write!(f, "A5"),
            PaperSize::A4 => write!(f, "A4"),
            PaperSize::A3 => write!(f, "A3"),
        }
    }
}

/// 画像の設定
/// height_mm: 縦の長さ(mm)
/// width_mm: 横の長さ(mm)
/// diff_mm: 隙間(mm)
/// dpi: 解像度
/// is_aspect_ratio: アスペクト比を維持するか
/// is_rotate: 回転するか
/// rows: 縦に配する数
/// cols: 横に配する数
#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub height_mm: f64,
    pub width_mm: f64,
    pub diff_mm: f64,
    pub dpi: f64,
    pub is_aspect_ratio: bool,
    pub is_rotate: bool,
    pub rows: u32,
    pub cols: u32,
}

impl ImageConfig {
    pub fn size(&self) -> (f64, f64) {
        (self.width_mm, self.height_mm)
    }
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            height_mm: 32.0,
            width_mm: 22.0,
            diff_mm: 0.5,
            dpi: 350.0,
            is_aspect_ratio: false,
            is_rotate: false,
            rows: 5,
            cols: 8,
        }
    }
}

const INCH: f64 = 25.4;
fn convert_mm_to_px(mm: f64, dpi: f64) -> u32 {
    (mm / INCH * dpi).round() as u32
}

pub fn process_image(
    image_config: &ImageConfig,
    paiser_size: &PaperSize,
    input_path: &PathBuf,
) -> Option<PathBuf> {
    let (width_mm, height_mm) = image_config.size();
    let width_px = convert_mm_to_px(width_mm, image_config.dpi);
    let height_px = convert_mm_to_px(height_mm, image_config.dpi);
    let Ok(img) = image::open(input_path) else {
        return None;
    };
    let resized_img = if image_config.is_aspect_ratio {
        img.resize(width_px, height_px, FilterType::Lanczos3)
    } else {
        img.resize_exact(width_px, height_px, FilterType::Lanczos3)
    };

    let (paiser_width_mm, paiser_height_mm) = paiser_size.size(image_config.is_rotate);
    let paiser_width_px = convert_mm_to_px(paiser_width_mm, image_config.dpi);
    let paiser_height_px = convert_mm_to_px(paiser_height_mm, image_config.dpi);
    let mut new_img = ImageBuffer::new(paiser_width_px, paiser_height_px);

    let diff = convert_mm_to_px(image_config.diff_mm, image_config.dpi);
    for y in 0..image_config.rows {
        for x in 0..image_config.cols {
            for (i, j, pixel) in resized_img.pixels() {
                let new_y = y * height_px + j;
                let new_x = x * (width_px + diff) + i;
                if new_x < paiser_height_px && new_y < paiser_width_px {
                    new_img.put_pixel(new_x, new_y, pixel);
                }
            }
        }
    }
    save_image_with_unique_name(&new_img, input_path.parent().unwrap())
}

fn save_image_with_unique_name(
    img: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    output_path: &Path,
) -> Option<PathBuf> {
    let mut new_output_path = output_path.join("output.png");
    let mut counter = 1;

    while new_output_path.exists() {
        new_output_path = output_path.join(format!("output({}).png", counter));
        counter += 1;
    }

    if img.save(&new_output_path).is_ok() {
        Some(new_output_path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_mm_to_px() {
        let a4_width = 210.0;
        let a4_height = 297.0;

        let dpi = 350.0;
        assert_eq!(convert_mm_to_px(a4_width, dpi), 2894);
        assert_eq!(convert_mm_to_px(a4_height, dpi), 4093);

        let dpi = 300.0;
        assert_eq!(convert_mm_to_px(a4_width, dpi), 2480);
        assert_eq!(convert_mm_to_px(a4_height, dpi), 3508);

        let dpi = 200.0;
        assert_eq!(convert_mm_to_px(a4_width, dpi), 1654);
        assert_eq!(convert_mm_to_px(a4_height, dpi), 2339);
    }
}
