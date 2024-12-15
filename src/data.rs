#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from_floats(r: f32, g: f32, b: f32) -> Self {
        Color{
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub fn white() -> Self {
        Color{r: 255, g: 255, b: 255}
    }

    pub fn black() -> Self {
        Color{r: 0, g: 0, b: 0}
    }

    pub fn red() -> Self {
        Color{r: 255, g: 0, b: 0}
    }

    pub fn green() -> Self {
        Color{r: 0, g: 255, b: 0}
    }

    pub fn blue() -> Self {
        Color{r: 0, g: 0, b: 255}
    }
}
