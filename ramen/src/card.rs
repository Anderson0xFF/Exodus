use std::{fs::File, os::fd::AsRawFd, };

use crate::{errors::ErrorKind, debug, error};

#[derive(Debug)]
pub struct Card(File);

impl Card {
    pub fn open(path: &str) -> Result<Self, ErrorKind> {
        debug!("Opening card: \"{}\"", path);

        if let Ok(file) = File::open(path) {
            return Ok(Card(file));
        }

        error!("Failed to open card: \"{}\" - ErrorKind: {:?}", path, ErrorKind::RAMEN_CARD_OPEN_FAILED);
        Err(ErrorKind::RAMEN_CARD_OPEN_FAILED)
    }

    #[inline]
    pub(crate) fn fd(&self) -> i32 {
        self.0.as_raw_fd()
    }
}