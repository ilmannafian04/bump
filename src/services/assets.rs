use std::str;

use log::error;

use crate::{error::Error, stores};

pub fn get_text_file(file_name: &str) -> Result<String, Error> {
    let embed = match stores::assets::Asset::get(&file_name) {
        Some(embed) => embed,
        None => {
            error!("asset not found");
            return Err(Error::NotFound);
        }
    };

    let buff = embed.data.into_owned();
    match str::from_utf8(&buff) {
        Ok(data) => Ok(data.to_owned()),
        Err(_) => {
            error!("failed to parse asset, might not be a text file");
            Err(Error::NotFound)
        }
    }
}

pub fn get_raw(file_name: &str) -> Result<Vec<u8>, Error> {
    match stores::assets::Asset::get(file_name) {
        Some(embed) => Ok(embed.data.into_owned()),
        None => {
            error!("asset not found");
            Err(Error::NotFound)
        }
    }
}
