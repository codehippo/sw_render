use crate::common::space::{WorldBox, WorldPoint, WorldSize};

pub trait Positionable {
    fn get_position(&self) -> WorldPoint;

    fn as_bounded(&self) -> Option<&dyn Bounded> {
        None
    }
}

pub trait Dimensionable {
    fn get_dimensions(&self) -> WorldSize;
}

pub trait Bounded: Positionable + Dimensionable {
    fn calculate_bounding_box(&self) -> WorldBox {
        let position = self.get_position();
        let dimensions = self.get_dimensions();
        WorldBox::new(position, position + dimensions.to_vector())
    }
}
