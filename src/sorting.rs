use std::cmp::Ordering;

use image::Rgb;

use crate::color::HSB;

pub enum SortMethod {
    HsbHue,
    HsbSaturation,
    HsbBrightness,
    Intensity,
    Minimum
}

pub fn get_sort_func(sort: &SortMethod) -> fn(&Rgb<u8>, &Rgb<u8>) -> Ordering {
    match sort {
        SortMethod::HsbHue => sort_by_hue,
        SortMethod::HsbSaturation => sort_by_saturation,
        SortMethod::HsbBrightness => sort_by_brightness,
        SortMethod::Intensity => sort_by_intensity,
        SortMethod::Minimum => sort_by_minimum
    }
}

pub fn sort_by_hue(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {
    let a_hue = HSB::rgb_get_hue(&a[0], &a[1], &a[2]);
    let b_hue = HSB::rgb_get_hue(&b[0], &b[1], &b[2]);

    a_hue.partial_cmp(&b_hue).unwrap()
}

pub fn sort_by_saturation(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {
    let a_sat = HSB::rgb_get_saturation(&a[0], &a[1], &a[2]);
    let b_sat = HSB::rgb_get_saturation(&b[0], &b[1], &b[2]);

    a_sat.partial_cmp(&b_sat).unwrap()
}

pub fn sort_by_brightness(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {
    let a_bri = HSB::rgb_get_brightness(&a[0], &a[1], &a[2]);
    let b_bri = HSB::rgb_get_brightness(&b[0], &b[1], &b[2]);

    a_bri.partial_cmp(&b_bri).unwrap()
}

pub fn sort_by_intensity(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {
    let a_int = a[0] as u16 + a[1] as u16 + a[2] as u16;
    let b_int = b[0] as u16 + b[1] as u16 + b[2] as u16;

    a_int.cmp(&b_int)
}

pub fn sort_by_minimum(a: &Rgb<u8>, b: &Rgb<u8>) -> Ordering {

    let mut a_min = a[0];

    if a[1] < a_min {
        a_min = a[1];
    }

    if a[2] < a_min {
        a_min = a[2];
    }

    let mut b_min = b[0]; 

    if b[1] < b_min {
        b_min = b[1];
    }

    if b[2] < b_min {
        b_min = b[2];
    }

    a_min.cmp(&b_min)
}