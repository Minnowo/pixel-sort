
use rgb::RGB8;

#[derive(Debug)]
pub struct HSB {
    hue: f32,
    saturation: f32,
    brightness: f32,
}

impl HSB {
    fn calculate_hue(f_r: &f32, f_g: &f32, f_b: &f32, max: &f32, min: &f32) -> f32 {
        if max == f_r {
            if f_r - min != 0f32 {
                let div = (f_g - f_b) / (f_r - min);

                // (div % 6) / 6
                let hue = (div - ((div as i32) * 6) as f32) / 6f32;

                if hue < 0f32 {
                    return hue + 1f32;
                }
                return hue;
            }
            return 0f32;
        } else if max == f_b {
            (4f32 + (f_r - f_g) / (f_b - min)) / 6f32
        } else {
            (2f32 + (f_b - f_r) / (f_g - min)) / 6f32
        }
    }

    pub fn from(r: &u8, g: &u8, b: &u8) -> HSB {
        let mut hsb = HSB {
            hue: 0f32,
            saturation: 0f32,
            brightness: 0f32,
        };

        let f_r = (*r as f32) / 255f32;
        let f_g = (*g as f32) / 255f32;
        let f_b = (*b as f32) / 255f32;

        let mut min = f_r;
        let mut max = f_r;

        if f_g < min {
            min = f_g;
        } else {
            max = f_g;
        }

        if f_b < min {
            min = f_b;
        }

        if f_b > max {
            max = f_b;
        }

        hsb.hue = HSB::calculate_hue(&f_r, &f_g, &f_b, &max, &min);

        if max != 0f32 {
            hsb.saturation = (max - min) / max;
        }

        hsb.brightness = max;

        hsb
    }

    pub fn from_rgb8(rgb: &RGB8) -> HSB {
        HSB::from(&rgb.r, &rgb.g, &rgb.b)
    }

    pub fn rgb_get_hue(r: &u8, g: &u8, b: &u8) -> f32 {
        let f_r = (*r as f32) / 255f32;
        let f_g = (*g as f32) / 255f32;
        let f_b = (*b as f32) / 255f32;

        let mut min = f_r;
        let mut max = f_r;

        if f_g < min {
            min = f_g;
        } else {
            max = f_g;
        }

        if f_b < min {
            min = f_b;
        }

        if f_b > max {
            max = f_b;
        }

        HSB::calculate_hue(&f_r, &f_g, &f_b, &max, &min)
    }

    pub fn rgb_get_saturation(r: &u8, g: &u8, b: &u8) -> f32 {
        let f_r = (*r as f32) / 255f32;
        let f_g = (*g as f32) / 255f32;
        let f_b = (*b as f32) / 255f32;

        let mut min = f_r;
        let mut max = f_r;

        if f_g < min {
            min = f_g;
        } else {
            max = f_g;
        }

        if f_b < min {
            min = f_b;
        }

        if f_b > max {
            max = f_b;
        }
        if max != 0f32 {
            return (max - min) / max;
        }

        return 0f32;
    }

    pub fn rgb_get_brightness(r: &u8, g: &u8, b: &u8) -> f32 {
        let mut max = r;

        if g > max {
            max = g;
        }

        if b > max {
            max = b;
        }
        return (*max as f32) / 255f32;
    }
}
