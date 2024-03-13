use crate::{render_params::*, JPEG_IMAGE_DATA};
use zune_core::options::DecoderOptions;
use zune_image::{image::Image, traits::OperationsTrait};
use zune_imageprocs::{
    composite::{Composite, CompositeMethod},
    crop::Crop,
    resize::{Resize, ResizeMethod},
};

pub struct ZuneImageBenchmark;

impl ZuneImageBenchmark {
    pub fn load_image() -> Image {
        Image::read(JPEG_IMAGE_DATA, DecoderOptions::default()).unwrap()
    }

    pub fn render() -> Image {
        let mut src_image = Self::load_image();
        let mut dst_image = src_image.clone();

        let crop = Crop::new(
            SRC_WIDTH as usize,
            SRC_HEIGHT as usize,
            SRC_X as usize,
            SRC_Y as usize,
        );
        crop.execute(&mut src_image).unwrap();

        let resize = Resize::new(
            DST_WIDTH as usize,
            DST_HEIGHT as usize,
            ResizeMethod::Bilinear,
        );

        resize.execute(&mut src_image).unwrap();

        let composite = Composite::try_new(
            &src_image,
            CompositeMethod::Over,
            Some((DST_X as usize, DST_Y as usize)),
            None,
        )
        .unwrap();

        composite.execute(&mut dst_image).unwrap();

        dst_image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zune_image::codecs::ImageFormat;

    fn assert_image_eq(image: &Image, ref_image_path: &str, update_ref_image: bool) {
        // encode image to jpeg
        let encoded_image = image.write_to_vec(ImageFormat::JPEG).unwrap();

        if update_ref_image {
            std::fs::write(ref_image_path, &encoded_image).unwrap();
            return;
        }

        let expected_image = std::fs::read(ref_image_path).unwrap();
        assert_eq!(expected_image, encoded_image);
    }

    #[test]
    fn test_render() {
        let rendered_image = ZuneImageBenchmark::render();

        assert_image_eq(&rendered_image, "resources/zune_image_test.jpg", false);
    }
}
