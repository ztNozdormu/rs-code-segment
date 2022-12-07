pub trait Shape {
    fn draw(&self);
}

#[derive(Debug)]
pub enum ShapType {
    RecTangle,
    Circle,
}

pub struct RecTangle;

pub struct Circle;

impl Shape for RecTangle {
    fn draw(&self) {
        println!("我是RecTangle形状的图形");
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("我是Circle形状的图形");
    }
}

pub struct ShapeFactory;


impl ShapeFactory {
    pub fn new_shape(shape_type:&ShapType) -> Box<dyn Shape> {

        match shape_type {
            ShapType::RecTangle => Box::new(RecTangle),
            ShapType::Circle => Box::new(Circle),
        }

    }
}