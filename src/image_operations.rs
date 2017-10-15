/// Smaller image processing operations

extern crate image;

pub fn edge_operation(img: image::DynamicImage) -> image::DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0, 8.0, -1.0,
                  -1.0, -1.0, -1.0];
    let filtered: image::DynamicImage = img.filter3x3(&kernel);
    filtered
}
