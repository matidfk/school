use std::{
    fs::{read_to_string, write},
    path::Path,
};

use iced::widget::image::Handle;
use notify_rust::Notification;

/// The image path to use when an item has no defined image path
const NO_IMAGE_PATH: &str = "_none.jpg";

/// Helper function to get an image handle provided a filename
/// TODO: Cache handles
pub fn get_handle(image_path: &Option<String>) -> Handle {
    match image_path {
        Some(path) if Path::new(&format!("images/{path}")).exists() && path != "" => {
            format!("images/{path}").into()
        }
        _ => format!("images/{NO_IMAGE_PATH}").to_string().into(),
    }
}

/// Helper function to convert pence to price string
pub fn format_price(input: u32) -> String {
    let pounds = input / 100;
    let pence = input - (pounds * 100);

    format!("£{0}.{1:0<2}", pounds, pence)
}

/// Helper function to notify the user
pub fn notify(title: &str, description: &str) {
    Notification::new()
        .summary(title)
        .body(description)
        // .icon("idk")
        .show()
        .unwrap();
}

const ENCRYPTION_OFFSET: i8 = 3;

pub fn encrypt(input: &str) -> String {
    input
        .chars()
        .map(|character| {
            let mut ascii = character as u8;
            ascii = ascii.overflowing_add_signed(ENCRYPTION_OFFSET).0;
            ascii as char
        })
        .collect::<String>()
}

pub fn set_password(input: &str) {
    write(".password", encrypt(input)).unwrap();
}

pub fn get_password() -> String {
    read_to_string(".password").unwrap()
}
