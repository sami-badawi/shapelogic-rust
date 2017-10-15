/// Smaller image processing operations
extern crate image;


pub fn filter_operation(img: image::DynamicImage, kernel: [f32; 9]) -> image::DynamicImage {
    let filtered: image::DynamicImage = img.filter3x3(&kernel);
    filtered
}

pub fn edge_operation(img: image::DynamicImage) -> image::DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    filter_operation(img, kernel)
}

pub fn sobel_h_operation(img: image::DynamicImage) -> image::DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    filter_operation(img, kernel)
}

pub fn sobel_v_operation(img: image::DynamicImage) -> image::DynamicImage {
    let kernel = [-1.0f32, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0];
    filter_operation(img, kernel)
}

pub fn blur_operation(img: image::DynamicImage) -> image::DynamicImage {
    let kernel = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    filter_operation(img, kernel)
}
