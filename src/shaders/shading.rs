use crate::shaders::traits::*;

pub struct GouraudShading;
pub struct PhongShading;

impl Shader for GouraudShading {}

impl Shader for PhongShading {}
