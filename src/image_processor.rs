use std::path::PathBuf;

use image::{imageops::FilterType, GenericImageView, ImageBuffer};

#[derive(Debug, Clone)]
pub enum PaperSize {
    A4,
    A3,
}

impl PaperSize {
    pub fn size(&self) -> (f64, f64) {
        match self {
            PaperSize::A4 => (210.0, 297.0),
            PaperSize::A3 => (297.0, 420.0),
        }
    }

    pub fn vec_new() -> Vec<Self> {
        vec![PaperSize::A4, PaperSize::A3]
    }
}

impl std::fmt::Display for PaperSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperSize::A4 => write!(f, "A4"),
            PaperSize::A3 => write!(f, "A3"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub height_mm: f64,
    pub width_mm: f64,
    pub diff_mm: f64,
    pub dpi: f64,
    pub is_aspect_ratio: bool,
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
            is_aspect_ratio: true,
            rows: 8,
            cols: 16,
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
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ()> {
    let (width_mm, height_mm) = image_config.size();
    let width_px = convert_mm_to_px(width_mm, image_config.dpi);
    let height_px = convert_mm_to_px(height_mm, image_config.dpi);
    let Ok(img) = image::open(input_path) else {
        return Err(());
    };
    let resized_img = if image_config.is_aspect_ratio {
        img.resize(width_px, height_px, FilterType::Lanczos3)
    } else {
        img.resize_exact(width_px, height_px, FilterType::Lanczos3)
    };
    let (paiser_width_mm, paiser_height_mm) = paiser_size.size();
    let paiser_width_px = convert_mm_to_px(paiser_width_mm, image_config.dpi);
    let paiser_height_px = convert_mm_to_px(paiser_height_mm, image_config.dpi);
    let diff = convert_mm_to_px(image_config.diff_mm, image_config.dpi);
    let mut new_img = ImageBuffer::new(paiser_height_px, paiser_width_px);
    for y in 0..image_config.rows {
        for x in 0..image_config.cols {
            for (i, j, pixel) in resized_img.pixels() {
                let new_x = x * (width_px + diff) + i;
                let new_y = y * (height_px + diff) + j;
                if new_x < height_px && new_y < width_px {
                    new_img.put_pixel(new_x, new_y, pixel);
                }
            }
        }
    }
    Ok(new_img)
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
