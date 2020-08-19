use crate::hsl::HslColor;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub const SCALE: f32 = 255.0;

    pub fn red(self) -> u8 {
        self.0
    }

    pub fn green(self) -> u8 {
        self.1
    }

    pub fn blue(self) -> u8 {
        self.2
    }

    pub fn into_hsl(self) -> HslColor {
        self.into()
    }
}

impl From<HslColor> for RgbColor {
    fn from(hsl: HslColor) -> Self {
        let (r, g, b) = match hsl.1 {
            s if s == 0.0 => (hsl.2, hsl.2, hsl.2),
            _ => {
                let q = if hsl.2 < 0.5 {
                    hsl.2 * (1.0 + hsl.1)
                } else {
                    hsl.2 + hsl.1 - hsl.2 * hsl.1
                };
                let p = 2.0 * hsl.2 - q;
                (
                    get_rgb_from_hue(p, q, hsl.0 + 1.0 / 3.0),
                    get_rgb_from_hue(p, q, hsl.0), 
                    get_rgb_from_hue(p, q, hsl.0 - 1.0 / 3.0)
                )
            }
        };

        Self(
            (r * RgbColor::SCALE).ceil() as u8,
            (g * RgbColor::SCALE).ceil() as u8,
            (b * RgbColor::SCALE).ceil() as u8,
        )
    }
}

fn get_rgb_from_hue(p: f32, q: f32, t: f32) -> f32 {
    let nt = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };

    match nt {
        nt if nt < 1.0 / 6.0 => p + (q - p) * 6.0 * nt,
        nt if nt < 1.0 / 2.0 => q,
        nt if nt < 2.0 / 3.0 => p + (q - p) * (2.0 / 3.0 - nt) * 6.0,
        _ => p,
    }
}


#[test]
fn test_hsl_to_rgb() {
    let cases = vec![
        // Input,                             Expected
        (HslColor(0.0, 0.0, 0.0),             RgbColor(0, 0, 0)),
        (HslColor(0.0, 0.0, 0.5),             RgbColor(128, 128, 128)),
        (HslColor(180.0 / 360.0, 1.0, 0.5),   RgbColor(0, 255, 255)),
        (HslColor(30.0 / 360.0, 0.5, 0.5),    RgbColor(192, 128, 64)),
        (HslColor(90.0 / 360.0, 0.2, 0.8),    RgbColor(204, 215, 194)),
    ];

    for (hsl, rgb) in cases.into_iter() {
        assert_eq!(hsl.into_rgb(), rgb);
    }
}
