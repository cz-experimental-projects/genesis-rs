use bevy::prelude::{Component, Color};

use crate::game::components::gene::{DNA, Gene, Shape};

#[derive(Clone, Debug, Default)]
pub enum CellType {
    #[default]
    Nerve,
    Stem,
    WBC,
    RBC,
    Platelets,
    Muscle,
    Cartilage,
    Bone,
    Skin,
    Endothelial,
    Epithelial,
    Fat,
    Sex,
}

impl CellType {
    pub fn default_dna(&self) -> DNA {
        match self {
            CellType::Nerve => DNA::new(vec![Gene::Shape(Shape::Polygon(6, 1.0)), Gene::Color(Color::BEIGE)]),
            CellType::Stem => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::BEIGE)]),
            CellType::WBC => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::WHITE)]),
            CellType::RBC => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::RED)]),
            CellType::Platelets => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::rgb_u8(237, 207, 199)), Gene::Stroke(Color::rgb_u8(191, 155, 145), 5.0)]),
            CellType::Muscle => DNA::new(vec![Gene::Shape(Shape::Polygon(5, 1.0)), Gene::Color(Color::PINK)]),
            CellType::Cartilage => DNA::new(vec![Gene::Shape(Shape::Polygon(3, 1.0)), Gene::Color(Color::ANTIQUE_WHITE)]),
            CellType::Bone => DNA::new(vec![Gene::Shape(Shape::Polygon(4, 1.0)), Gene::Color(Color::WHITE)]),
            CellType::Skin => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::rgb_u8(250, 209, 185))]),
            CellType::Endothelial => DNA::new(vec![Gene::Shape(Shape::Polygon(7, 1.0)), Gene::Color(Color::rgb_u8(235, 175, 157))]),
            CellType::Epithelial => DNA::new(vec![Gene::Shape(Shape::Circle(1.0)), Gene::Color(Color::rgb_u8(242, 220, 187))]),
            CellType::Fat => DNA::new(vec![Gene::Shape(Shape::Polygon(6, 1.0)), Gene::Color(Color::YELLOW)]),
            CellType::Sex => DNA::new(vec![Gene::Shape(Shape::Polygon(10, 1.0)), Gene::Color(Color::PINK)]),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Cell {
    pub cell_type: CellType,
    pub dna: DNA,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Cells {
    pub cells: Vec<Cell>,
}

impl Cells {
    pub fn new(cells: Vec<Cell>) -> Cells {
        Self {
            cells,
        }
    }
}
