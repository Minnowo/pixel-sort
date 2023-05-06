pub struct HSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl HSL {
    pub fn from_rgb_u8(r: &u8, g: &u8, b: &u8) -> HSL {
        let mut hsb = HSL {
            hue: 0_f32,
            saturation: 0_f32,
            lightness: 0_f32,
        };

        let f_r = (*r as f32) / 255_f32;
        let f_g = (*g as f32) / 255_f32;
        let f_b = (*b as f32) / 255_f32;

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

        hsb.hue = calculate_hue(&f_r, &f_g, &f_b, &max, &min);

        hsb.lightness = (max + min) / 2_f32;

        if max != min {
            hsb.saturation = (max - min) / (1_f32 - (2_f32 * hsb.lightness - 1_f32).abs())
        }

        hsb
    }

    pub fn from_rgb(rgb: [u8; 3]) -> HSL {
        HSL::from_rgb_u8(&rgb[0], &rgb[1], &rgb[2])
    }

    pub fn from_rgb_vec(rgb: Vec<u8>) -> HSL {
        HSL::from_rgb_u8(&rgb[0], &rgb[1], &rgb[2])
    }
}

fn calculate_hue(f_r: &f32, f_g: &f32, f_b: &f32, max: &f32, min: &f32) -> f32 {
    if max == f_r {
        if f_r - min == 0_f32 {
            return 0_f32;
        }
        let div = (f_g - f_b) / (f_r - min);

        // (div % 6) / 6
        let hue = (div - ((div as i32) * 6) as f32) / 6_f32;

        if hue < 0_f32 {
            return hue + 1_f32;
        }
        return hue;
    } else if max == f_b {
        (4_f32 + (f_r - f_g) / (f_b - min)) / 6_f32
    } else {
        (2_f32 + (f_b - f_r) / (f_g - min)) / 6_f32
    }
}

pub fn rgb_get_hue(r: &u8, g: &u8, b: &u8) -> f32 {
    let f_r = (*r as f32) / 255_f32;
    let f_g = (*g as f32) / 255_f32;
    let f_b = (*b as f32) / 255_f32;

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

    calculate_hue(&f_r, &f_g, &f_b, &max, &min)
}

pub fn rgb_get_saturation(r: &u8, g: &u8, b: &u8) -> f32 {
    let f_r = (*r as f32) / 255_f32;
    let f_g = (*g as f32) / 255_f32;
    let f_b = (*b as f32) / 255_f32;

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
    if max != 0_f32 {
        return (max - min) / max;
    }

    let lightness = (max + min) / 2_f32;

    if max != min {
        return (max - min) / (1_f32 - (2_f32 * lightness - 1_f32).abs());
    }

    return 0f32;
}

pub fn rgb_get_lightness(r: &u8, g: &u8, b: &u8) -> f32 {
    let f_r = (*r as f32) / 255_f32;
    let f_g = (*g as f32) / 255_f32;
    let f_b = (*b as f32) / 255_f32;

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
    let lightness = (max + min) / 2_f32;
    lightness
}