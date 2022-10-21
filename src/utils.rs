use iced::widget::image::Handle;

/// The image path to use when an item has no defined image path
const NO_IMAGE_PATH: &str = "_none.jpg";

/// Helper function to get an image handle provided a filename
/// TODO: Cache handles
pub fn get_handle(name: &Option<String>) -> Handle {
    Handle::from_path(format!(
        "images/{}",
        name.clone().unwrap_or(NO_IMAGE_PATH.to_string())
    ))
}

/// Helper function to convert pence to price string
pub fn format_price(input: u32) -> String {
    let pounds = input / 100;
    let pence = input - (pounds * 100);

    format!("£{0}.{1:0<2}", pounds, pence)
}
