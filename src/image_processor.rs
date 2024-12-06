use std::path::PathBuf;

use image::{imageops::FilterType, GenericImageView, ImageBuffer};

const INCH: f64 = 25.4;
fn convert_mm_to_px(mm: f64, dpi: f64) -> u32 {
    (mm / INCH * dpi).round() as u32
}

pub fn process_image(
    height_mm: f64,
    width_mm: f64,
    diff_mm: f64,
    dpi: f64,
    rows: u32,
    cols: u32,
    paper_x: f64,
    paper_y: f64,
    is_aspect_ratio: bool,
    input_path: &PathBuf,
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ()> {
    let width_px = convert_mm_to_px(width_mm, dpi);
    let height_px = convert_mm_to_px(height_mm, dpi);
    let diff = convert_mm_to_px(diff_mm, dpi);

    let paper_x_px = convert_mm_to_px(paper_x, dpi);
    let paper_y_px = convert_mm_to_px(paper_y, dpi);

    let Ok(img) = image::open(input_path) else {
        return Err(());
    };
    let resized_img = if is_aspect_ratio {
        img.resize(width_px, height_px, FilterType::Lanczos3)
    } else {
        img.resize_exact(width_px, height_px, FilterType::Lanczos3)
    };
    let mut new_img = ImageBuffer::new(paper_y_px, paper_x_px);
    for y in 0..rows {
        for x in 0..cols {
            for (i, j, pixel) in resized_img.pixels() {
                let new_x = x * (width_px + diff) + i;
                let new_y = y * height_px + j;
                if new_x < paper_y_px && new_y < paper_x_px {
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
