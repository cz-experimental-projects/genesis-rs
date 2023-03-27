use bevy::prelude::{Component, Color};

#[derive(Clone, Debug)]
pub enum Shape {
    Circle(f32),
    Polygon(usize, f32),
}

#[derive(Component, Clone, Debug)]
pub enum Gene {
    Shape(Shape),
    Color(Color),
    Stroke(Color, f32),
}
