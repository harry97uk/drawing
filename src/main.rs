mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable, Point};
use raster::{Color, Image};

fn main() {

    //create blank image
    let mut image = Image::blank(1000, 1000);

    //draw a random line
    gs::Line::new(&Point::new(250, 250), &Point::new(750, 750)).draw(&mut image);

    //draw a random point
    gs::Point::random(image.width, image.height).draw(&mut image);

    //instantiate a rectangle and draw it
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    //same for triangle
    let triangle = gs::Triangle::new (
            &gs::Point::new(500, 500),
            &gs::Point::new(250, 700),
            &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    //draw random circles
    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    //save everything onto the file
    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}