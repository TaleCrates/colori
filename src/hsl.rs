use crate::rgb::{RgbColor};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct HslColor(pub f32, pub f32, pub f32);

impl HslColor {
    pub fn hue(self) -> f32 {
        self.0
    }

    pub fn lightness(self) -> f32 {
        self.1
    }

    pub fn saturation(self) -> f32 {
        self.2
    }

    pub fn into_rgb(self) -> RgbColor {
        self.into()
    }
}

impl From<RgbColor> for HslColor {
    fn from(rgb: RgbColor) -> Self {
        let (r, g, b) = (rgb.0 as f32 / RgbColor::SCALE, rgb.1 as f32 / RgbColor::SCALE, rgb.2 as f32 / RgbColor::SCALE);
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let mut hue = 0f32;
        let mut saturation = 0f32;
        let lightness = (max + min) / 2.0;

        if max != min {
            let d = max - min;
            saturation = if lightness > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

            hue = match max {
                m if m == r => (g - b) / d + (if g < b { 6.0 } else { 0.0 }),
                m if m == g => (b - r) / d + 2.0,
                m if m == b => (r - g) / d + 4.0,
                _ => hue,
            };

            hue /= 6.0;
        }

        Self(hue, saturation, lightness)
    }
}


#[test]
fn test_rgb_to_hsl() {
    let cases = vec![
        // Input,                             Expected (Hsl)
        (RgbColor(0, 0, 0),                   HslColor(0.0, 0.0, 0.0)),
        (RgbColor(128, 128, 128),             HslColor(0.0, 0.0, 0.5019608)),
        (RgbColor(0, 255, 255),               HslColor(180.0 / 360.0, 1.0, 0.5)),
        (RgbColor(192, 128, 64),              HslColor(0.08333334, 0.50393695, 0.5019608)),
        (RgbColor(204, 215, 194),             HslColor(0.25396827, 0.20792079, 0.8019608)),
    ];

    for (rgb, hsl) in cases.into_iter() {
        assert_eq!(rgb.into_hsl(), hsl);
    }
}
