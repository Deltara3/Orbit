#[derive(Debug)]
pub struct Tile {
    pixels: Vec<Option<u8>>,
    colors: Vec<Color>
}

impl Tile {
    pub fn from_raw(raw: Vec<u8>) -> Self {
        let mut ptr = 0;
        let index_end = 9 + raw[8] as usize;
        let mut index_data = &raw[9..index_end];
        let mut indexes: Vec<u8> = vec![];

        let mut pixels: Vec<Option<u8>> = vec![];
        let mut colors: Vec<Color> = vec![];

        while ptr != index_data.len() {
            let byte = index_data[ptr];

            if byte == 0xff {
                for amount in 0..index_data[1] {
                    indexes.push(index_data[2])
                }

                if indexes.len() < 64 {
                    for i in indexes.len()..64 {
                        indexes.push(0);
                    }
                }

                break;
            } else {
                indexes.push(byte);
            }
        }

        ptr = 0;

        for row in &raw[0..8] {
            for bit in (0..8).rev() {
                if (row >> bit) & 1 == 1 {
                    pixels.push(Some(indexes[ptr]));
                } else {
                    pixels.push(None);
                }

                ptr += 1;
            }
        }

        for color in (index_end..raw.len()).step_by(3) {
            colors.push(Color {
                r: raw[color],
                g: raw[color + 1],
                b: raw[color + 2]
            });
        }

        Self {
            pixels: pixels,
            colors: colors
        }
    }
}

/* Placeholder until I add egui */
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}