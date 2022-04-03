

use image::DynamicImage;
use viuer::Config;

pub(crate) async fn donwload_image(url : &str) -> Option<DynamicImage> {
    let image_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    image::load_from_memory(&image_bytes).ok()
}

pub(crate) fn show_image(image : &DynamicImage) -> Option<()>{
    let config = Config {
        absolute_offset: false,
        x : 0,
        y : 0,
        width : Some(50),
        height : Some(50),
        ..Default::default()
    };
    viuer::print(image, &config).ok().map(|_| ())
}