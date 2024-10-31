use std::env;
use std::fs;

use anyhow::Result;

use smb1_tools::{Rom, RomLevel};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom_file = &args[1];
    let world = &args[2];

    let rom_data = fs::read(rom_file)?;
    let rom = Rom::new(rom_data)?;

    let level = rom.get_level(&RomLevel::from_name(world));
    println!("{:#?}", level);
    println!("objects.len = {}", level.object_data.objects.len());
    println!("enemies.len = {}", level.enemy_data.enemies.len());

    Ok(())
}
