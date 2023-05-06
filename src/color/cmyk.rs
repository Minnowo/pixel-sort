pub struct CMYK {
   pub c: f32,
   pub m: f32,
   pub y: f32,
   pub k: f32,
}

impl CMYK {
    pub fn from_rgb_u8(r: &u8, g: &u8, b: &u8) -> CMYK {
        let mut cmyk = CMYK {
            c: 0_f32,
            m: 0_f32,
            y: 0_f32,
            k: 1_f32,
        };

        if *r == 0_u8 && *g == 0_u8 && *b == 0_u8 {
            return cmyk;
        }

        let f_r = (*r as f32) / 255f32;
        let f_g = (*g as f32) / 255f32;
        let f_b = (*b as f32) / 255f32;

        let mut max = f_r;

        if f_g > max {
            max = f_g;
        }

        if f_b > max {
            max = f_b;
        }

        cmyk.k = 1_f32 - max;
        cmyk.c = (1_f32 - f_r - cmyk.k) / (1_f32 - cmyk.k);
        cmyk.m = (1_f32 - f_g - cmyk.k) / (1_f32 - cmyk.k);
        cmyk.y = (1_f32 - f_b - cmyk.k) / (1_f32 - cmyk.k);

        cmyk
    }

    pub fn from_rgb(rgb: [u8; 3]) -> CMYK {
        CMYK::from_rgb_u8(&rgb[0], &rgb[1], &rgb[2])
    }

    pub fn from_rgb_vec(rgb: Vec<u8>) -> CMYK {
        CMYK::from_rgb_u8(&rgb[0], &rgb[1], &rgb[2])
    }
}
