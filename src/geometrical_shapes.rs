use std::f32::consts::PI;

use raster::{Image, Color};
use rand::Rng;

#[allow(unused)]
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> raster::Color {
        let mut seed = rand::thread_rng();
        let new_r = seed.gen_range(0..=255);
        let new_g = seed.gen_range(0..=255);
        let new_b = seed.gen_range(0..=255);
    
        raster::Color{r: new_r, g: new_g, b: new_b, a: 255}
    }
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(image_width: i32, image_height: i32) -> Self {
        let mut seed = rand::thread_rng();
        let new_y: i32 = seed.gen_range(0..image_height);
        let new_x: i32 = seed.gen_range(0..image_width);

        Point { x:new_x, y: new_y }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.x, self.y, Self::color());
    }
}

pub struct Line {
    point_1: Point,
    point_2: Point,
}

impl Line {
    pub fn new(point_1: &Point, point_2: &Point) -> Self {
        Line { 
            point_1: *point_1, 
            point_2: *point_2, 
        }
    }
    pub fn random(image_width: i32, image_height: i32) -> Self {
        Line { 
            point_1: Point::random(image_width, image_height), 
            point_2: Point::random(image_width, image_height) 
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        draw_line(image, self.point_1.x, self.point_1.y, self.point_2.x, self.point_2.y, Self::color())
    }
}

pub struct Triangle {
    point_1: Point,
    point_2: Point,
    point_3: Point
}

impl Triangle {
    pub fn new(point_1: &Point, point_2: &Point, point_3: &Point) -> Self {
        Triangle { 
            point_1: *point_1, 
            point_2: *point_2, 
            point_3: *point_3
        }    
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        draw_line(image, self.point_1.x, self.point_1.y, self.point_2.x, self.point_2.y, Self::color());
        draw_line(image, self.point_2.x, self.point_2.y, self.point_3.x, self.point_3.y, Self::color());
        draw_line(image, self.point_3.x, self.point_3.y, self.point_1.x, self.point_1.y, Self::color());
    }
}

pub struct Rectangle {
    point_1: Point,
    point_2: Point,
    point_3: Point,
    point_4: Point
}

impl Rectangle {
    pub fn new(point_1: &Point, point_2: &Point) -> Self {
        let point_3 = Point::new(point_1.x, point_2.y);
        let point_4 = Point::new(point_2.x, point_1.y);
        Rectangle { 
            point_1: *point_1, 
            point_2: *point_2,
            point_3,
            point_4 
        }    
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        draw_line(image, self.point_1.x, self.point_1.y, self.point_3.x, self.point_3.y, Self::color());
        draw_line(image, self.point_2.x, self.point_2.y, self.point_3.x, self.point_3.y, Self::color());
        draw_line(image, self.point_2.x, self.point_2.y, self.point_4.x, self.point_4.y, Self::color());
        draw_line(image, self.point_4.x, self.point_4.y, self.point_1.x, self.point_1.y, Self::color());
    }
}

pub struct Circle {
    center: Point,
    radius: i32
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle { center: *center, radius }    
    }
    pub fn random(image_width: i32, image_height: i32) -> Self {
        let mut seed = rand::thread_rng();
        let new_r = seed.gen_range(5..=20);
        Circle { center: Point::random(image_width, image_height), radius: new_r } 
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let (h, k) = (self.center.x as f32, self.center.y as f32);
        let radius = self.radius as f32;
        let mut theta = 0.0;
        let step = 0.01; // Adjust the step size as needed
        let color = Self::color();

        while theta < 2.0 * PI {
            let x = h + radius * f32::cos(theta);
            let y = k + radius * f32::sin(theta);

            // Do something with the current coordinate (x, y)
            image.set_pixel(x as i32, y as i32, color.clone());

            theta += step; // Increment theta by the step size
        }
    }
}

fn draw_line(image: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else {-dy} / 2;
    let mut err2;

    loop {
        //HANDLE ERROR

        // Set pixel
        image.set_pixel(x0, y0, color.clone());

        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }
}
