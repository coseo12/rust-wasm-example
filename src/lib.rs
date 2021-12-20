use std::f64;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use image::RgbaImage;
use image::imageops::colorops::huerotate_in_place;

#[wasm_bindgen]
pub struct ImageProccessor {
    context: CanvasRenderingContext2d,
    image: RgbaImage,
}

#[wasm_bindgen]
impl ImageProccessor {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> ImageProccessor {
         let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

        let width = canvas.width();
        let height = canvas.height();

        let image_data = context.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
        let raw_image_data = image_data.data().0;

        let image = RgbaImage::from_vec(width, height, raw_image_data).unwrap();

        ImageProccessor{ context, image }
    }

    pub fn hue_roteate(&mut self) {
        huerotate_in_place(&mut self.image, 90);
    }

    pub fn redraw(&self) {
        let raw_image_data = self.image.as_ref();
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(raw_image_data), self.image.width(), self.image.height()).unwrap();
        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}
