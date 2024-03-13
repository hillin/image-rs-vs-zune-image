use std::ops::Deref;

use image::{imageops, GenericImage, GenericImageView, RgbImage};

use crate::{render_params::*, JPEG_IMAGE_DATA};

pub struct ImageRsBenchmark;

impl ImageRsBenchmark {
    pub fn load_image() -> RgbImage {
        image::load_from_memory(JPEG_IMAGE_DATA).unwrap().to_rgb8()
    }

    pub fn render() -> RgbImage {
        let src_image: RgbImage = Self::load_image();
        let mut dst_image = src_image.clone();

        let src_view = src_image.view(SRC_X, SRC_Y, SRC_WIDTH, SRC_HEIGHT);

        let resized_src_view = imageops::resize(
            src_view.deref(),
            DST_WIDTH as u32,
            DST_HEIGHT as u32,
            imageops::FilterType::Triangle,
        );

        dst_image
            .copy_from(&resized_src_view, DST_X, DST_Y)
            .expect("failed to render from source image");

        dst_image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::codecs::jpeg::JpegEncoder;

    fn assert_image_eq(image: &RgbImage, ref_image_path: &str, update_ref_image: bool) {
        // encode image to jpeg
        let mut encoded_image = Vec::new();
        let encoder = JpegEncoder::new_with_quality(&mut encoded_image, 80);
        image.write_with_encoder(encoder).unwrap();

        if update_ref_image {
            std::fs::write(ref_image_path, &encoded_image).unwrap();
            return;
        }

        let expected_image = std::fs::read(ref_image_path).unwrap();
        assert_eq!(expected_image, encoded_image);
    }

    #[test]
    fn test_render() {
        let rendered_image = ImageRsBenchmark::render();

        assert_image_eq(&rendered_image, "resources/image_rs_test.jpg", false);
    }
}
