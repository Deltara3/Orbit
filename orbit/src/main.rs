mod ppu;

fn main() {
    let data = vec![0x00, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x42, 0x00, 0x03, 0xFF, 0x10, 0x00, 0xFF, 0xFF, 0xFF];
    let tile = ppu::Tile::from_raw(data);
    println!("{:?}", tile);
}
