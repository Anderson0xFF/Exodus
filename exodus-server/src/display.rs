use exodus_common::{consts::{EXODUS_DIRECTORY, EXODUS_LOG}, logger, info, net::connection::Connection, error, debug, memory::Allocator};
use exodus_errors::ErrorKind;
use std::{os::unix::net::UnixListener, path};
use crate::{client::Entity, device::GPU};

#[derive(Debug)]
pub struct Display {
    id:         i32,
    listener:   UnixListener,
    gpus:       Vec<GPU>,
    allocator:  Allocator,
}

impl Display {

    pub fn new(cache: usize) -> Result<Self, ErrorKind> {
        if !path::Path::new(EXODUS_DIRECTORY).exists() {
            std::fs::create_dir_all(EXODUS_DIRECTORY).unwrap();
        }

        let (id, dpy) = Self::discovery_display();

        let loggerfile = format!("exodus-display-{}.log", id);
        if let Ok(level) = std::env::var(EXODUS_LOG) {
            let level = match level.parse::<i32>() {
                Ok(lvl) => logger::Level::from(lvl),
                Err(_) => logger::Level::Error,
            };

            logger::Logger::initialize(id, level, Some(&loggerfile));
        } else {
            logger::Logger::initialize(id, logger::Level::Info, Some(&loggerfile));
        }


        info!("Initializing display...");
        let listener: UnixListener = Self::create_display_listener(&dpy)?;

        let gpus = GPU::enumerate_gpus()?;

        info!("Display initialized successfully.");
        Ok(Self { id, listener, allocator: Allocator::with_capacity(cache), gpus })
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

    pub fn dispose(&mut self) {
        debug!("Disposing display...");

        let display = format!("{}/exodus-{}", EXODUS_DIRECTORY, self.id);
        if path::Path::new(&display).exists() {
            std::fs::remove_file(&display).unwrap();
        }

        self.gpus.iter_mut().for_each(|gpu| gpu.dispose());
        
        debug!("Display disposed.");
    }

    pub fn get_gpu(&self, id: i32) -> Option<&GPU> {
        self.gpus.iter().find(|gpu| gpu.id() == id)
    }

    pub fn get_gpu_mut(&mut self, id: i32) -> Option<&mut GPU> {
        self.gpus.iter_mut().find(|gpu| gpu.id() == id)
    }

    pub fn gpus(&self) -> &[GPU] {
        self.gpus.as_ref()
    }

    pub fn gpus_mut(&mut self) -> &mut Vec<GPU> {
        &mut self.gpus
    }
    
}

impl Drop for Display {
    fn drop(&mut self) {
        self.dispose();
    }
}
