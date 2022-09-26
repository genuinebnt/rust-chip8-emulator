mod chip;
mod sdl_driver;

use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sdl_driver = sdl_driver::SdlDriver::new()?;

    let mut chip = chip::Chip::new();
    chip.load_rom("roms/games/Tetris [Fran Dachille, 1991].ch8");

    loop {
        let quit = sdl_driver.process_input(&mut chip.keypad);

        chip.cycle();

        sdl_driver.render(&mut chip, 15);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));

        if quit {
            break;
        }
    }

    Ok(())
}
