use crate::color::Color;

pub struct Image {
    data: Vec<Vec<Color>>,
    width: u16,
    height: u16,
}

impl Image {
    pub fn new(width: u16, height: u16, default_color: Color) -> Self {
        let mut data: Vec<Vec<Color>> = Vec::new();
        for _ in 0..height {
            let mut line: Vec<Color> = Vec::new();
            for _ in 0..width {
                line.push(default_color.clone());
            }
            data.push(line);
        }

        Image {
            data,
            width,
            height,
        }
    }
    pub fn get_color(&self, x: u16, y: u16) -> Color {
        return self.data[y as usize][x as usize].clone();
    }
    pub fn set_color(&mut self, x: u16, y: u16, color: Color) {
        return self.data[y as usize][x as usize] = color;
    }
    
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}
