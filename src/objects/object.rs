use crate::common::space::{ModelScalar, WorldPoint, WorldScalar, WorldVector};
use crate::common::traits::Positionable;

struct Object {
    scale: WorldVector,
}

impl Positionable for Object {
    fn get_position(&self) -> WorldPoint {
        todo!()
    }
}
