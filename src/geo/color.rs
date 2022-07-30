
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

pub struct Color32 {
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
}

impl Color 
{
    pub fn new(r : f32, g : f32, b : f32, a : f32) -> Color
    {
        return Color{r, g, b, a};
    }
    pub fn to_ldr(&self) -> Color32
    {
        return Color32::new(
            (self.r.clamp(0.0, 1.0) * 255.0).floor() as u8, 
            (self.g.clamp(0.0, 1.0) * 255.0).floor() as u8, 
            (self.b.clamp(0.0, 1.0) * 255.0).floor() as u8, 
            (self.a.clamp(0.0, 1.0) * 255.0).floor() as u8, 
        )
    }
}

impl Color32 
{
    pub fn new(r : u8, g : u8, b : u8, a : u8) -> Color32
    {
        return Color32{r, g, b, a};
    }
    pub fn to_hdr(&self) -> Color
    {
        return Color::new(
            self.r as f32 / 255.0, 
            self.g as f32 / 255.0, 
            self.b as f32 / 255.0, 
            self.a as f32 / 255.0, 
        )
    }
}