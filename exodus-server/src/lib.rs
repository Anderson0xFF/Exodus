extern crate drm;
extern crate gbm;

pub mod client;
pub mod display;
pub mod protocol_handler;


#[cfg(test)]
mod tests {

    #[test]
    fn rendering_direct_screen() {
        let mut display = super::display::Display::new().unwrap();
        let gpus = &display.gpus()[0];
        let gpu = display.gpu(gpus.id()).unwrap();
        let screens = gpu.screens();
        let screen = gpu.get_screen(screens[0].id()).unwrap();

        let length = (screen.width() * screen.height()) as usize;

        let mut background_color_1 = Vec::with_capacity(length);
        background_color_1.resize(length, 0xDE595D);

        let mut background_color_2 = Vec::with_capacity(length);
        background_color_2.resize(length, 0xDEA259);

        let mut background_color_3 = Vec::with_capacity(length);
        background_color_3.resize(length, 0x4CE0A5);

        println!("{}x{}", screen.width(), screen.height());

        for i in 0..500 {

            let instant = std::time::Instant::now();
            
            if i % 3 == 0 {
                screen.write(0, 0, screen.width(), screen.height(), &background_color_1).unwrap();
            }
            else if i % 3 == 1 {
                screen.write(0, 0, screen.width(), screen.height(), &background_color_2).unwrap();
            }
            else {
                screen.write(0, 0, screen.width(), screen.height(), &background_color_3).unwrap();
            }

            screen.swap_buffers().unwrap();

            let elapsed = instant.elapsed();

            //FPS
            let fps = 1.0 / (elapsed.as_secs() as f64 + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0));
            println!("FPS: {:.2}", fps);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        gpu.dispose();
    }
}

