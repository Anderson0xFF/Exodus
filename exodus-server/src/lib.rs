#![allow(dead_code)]

extern crate drm;
extern crate gbm;

pub mod display;
pub mod client;
pub mod device;
pub mod screen;
pub mod protocol_handler;

mod framebuffer;


#[cfg(test)]
mod tests {
    use libc::rand;

    use crate::display::Display;


    #[test]
    fn rendering_direct_screen() {
        let mut display = Display::new(512).unwrap();

        let gpu = display.gpus_mut().first_mut().unwrap();
        let screen = gpu.screens_mut().first_mut().unwrap();

        let mut color = 0x00000000;
        let width = 512;
        let height = 512;
        let lenght: usize = width * height;

        for _ in 1..60
        {
            //let start = std::time::Instant::now();
            screen.clear_color(00000000);

            let mut pixels = Vec::with_capacity(lenght);

            pixels.resize(lenght, color);
            screen.rect(10, 0, width as u32, height as u32, &pixels).unwrap();
            screen.swap_buffers().unwrap();

            // rand color r
            color = unsafe { rand() as u32 } % 255 ;
            // rand color g
            color = color + (unsafe { rand() as u32 } % 255) * 0x00000100;
            // rand color b
            color = color + (unsafe { rand() as u32 } % 255) * 0x00010000;


            //let end = std::time::Instant::now();

            //let duration = end.duration_since(start);
            //let fps = 1.0 / (duration.as_secs_f64() / 1.0);
            //println!("FPS: {}", fps);
            //std::thread::sleep(std::time::Duration::from_millis(120));
        }

        //entities.clear();
        display.dispose();

    }
}

