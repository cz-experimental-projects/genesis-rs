use bevy::prelude::{Component, Color, Vec3};

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
    Size(Vec3),
    MaxEnergyLevel(f32),
}

#[derive(Component, Clone, Debug, Default)]
pub struct DNA {
    pub dna: Vec<Gene>
}

impl DNA {
    pub fn new(dna: Vec<Gene>) -> DNA {
        Self {
            dna
        }
    }
}
