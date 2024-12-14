#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    fn new_opaque(r: f32, g: f32, b: f32) -> Self {
        Color{r, g, b, a: 1.0f32}
    }

    fn white() -> Self {
        Color{r: 1.0f32, g: 1.0f32, b: 1.0f32, a: 1.0f32}
    }

    fn black() -> Self {
        Color{r: 0.0f32, g: 0.0f32, b: 0.0f32, a: 0.0f32}
    }

    fn red() -> Self {
        Color{r: 1.0f32, g: 0.0f32, b: 0.0f32, a: 1.0f32}
    }

    fn green() -> Self {
        Color{r: 0.0f32, g: 1.0f32, b: 0.0f32, a: 1.0f32}
    }

    fn blue() -> Self {
        Color{r: 0.0f32, g: 0.0f32, b: 1.0f32, a: 1.0f32}
    }
}
