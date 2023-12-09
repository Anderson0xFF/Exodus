use exodus_common::{
    consts::*,
    graphics::device::GPU,
    net::connection::Connection,
    *,
};

use exodus_errors::ErrorKind;
use std::{os::unix::net::UnixListener, path};
use crate::client::Entity;

#[derive(Debug)]
pub struct Display {
    id:         i32,
    gpus:       Vec<GPU>,
    listener:   UnixListener,
}

impl Display {

    pub fn new() -> Result<Self, ErrorKind> {
        if !path::Path::new(EXODUS_DIRECTORY).exists() {
            std::fs::create_dir_all(EXODUS_DIRECTORY).unwrap();
            std::fs::create_dir_all(EXODUS_LOG_DIRECTORY).unwrap();
        }

        let (id, dpy) = Self::discovery_display();
        if let Err(_) = std::env::var(EXODUS_DISPLAY) {
            std::env::set_var(EXODUS_DISPLAY, id.to_string());
        }

        Self::logger_initialize(id);

        info!("Initializing display...");
        let listener: UnixListener = Self::create_display_listener(&dpy)?;
        let gpus = GPU::search_gpus()?;

        info!("Display initialized successfully.");
        Ok(Self { id, gpus, listener })
    }

    pub fn accept(&self) -> Option<Entity> {
        if let Ok((stream, _)) = self.listener.accept() {
            stream.set_nonblocking(true).unwrap();
            let conn = Connection::new(stream);
            return Some(Entity::new(conn));
        }

        None
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn gpus(&self) -> &[GPU] {
        &self.gpus
    }

    pub fn gpu(&mut self, id: i32) -> Option<&mut GPU> {
        let mut gpu = self.gpus.iter_mut().filter(|gpu| gpu.id() == id);
        gpu.next()
    }

    fn logger_initialize(id: i32) {
        let logger_dir = format!("{}/exodus-{}.log", EXODUS_LOG_DIRECTORY, id);
        if let Ok(level) = std::env::var(EXODUS_LOG) {
            let level = match level.parse::<i32>() {
                Ok(lvl) => logger::Level::from(lvl),
                Err(_) => logger::Level::Error,
            };

            logger::Logger::init(level, logger_dir.as_str());
        } else {
            logger::Logger::init(logger::Level::Debug, logger_dir.as_str());
        }
    }

    fn create_display_listener(path: &str) -> Result<UnixListener, ErrorKind> {
        if let Ok(listener) = UnixListener::bind(path) {
            listener.set_nonblocking(true).unwrap();
            return Ok(listener);
        }

        let err = ErrorKind::DISPLAY_LISTENER_FAILED;
        error!("Failed to creating display. - ErrorKind: {:?}", err);
        Err(err)
    }

    fn discovery_display() -> (i32, String) {
        let mut i = 0;
        loop {
            let path = format!("{}/exodus-{i}", EXODUS_DIRECTORY);
            if !path::Path::new(&path).exists() {
                break (i, path);
            }
            i += 1;
        }
    }

    fn dispose(&mut self) {
        debug!("Disposing display...");
        if let Ok(id) = std::env::var(EXODUS_DISPLAY) {
            if id == self.id.to_string() {
                std::env::remove_var(EXODUS_DISPLAY);
            }
        }

        let display = format!("{}/exodus-{}", EXODUS_DIRECTORY, self.id);
        if path::Path::new(&display).exists() {
            std::fs::remove_file(&display).unwrap();
        }

        for gpu in self.gpus.iter_mut() {
            gpu.dispose();
        }
        debug!("Display disposed.");
    }

}

impl Drop for Display {
    fn drop(&mut self) {
        self.dispose();
    }
}
