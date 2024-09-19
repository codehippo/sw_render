use crate::common::space::{ViewToClipTransform, WorldPoint, WorldToViewTransform, WorldVector};
use crate::common::traits::Positionable;
use glamour::prelude::*;

pub struct PerspectiveCamera {
    pub position: WorldPoint,
    forward: WorldVector,
    up: WorldVector,
    right: WorldVector,
    pub view_matrix: WorldToViewTransform,
    pub perspective_matrix: ViewToClipTransform,
}

// This is a right-handed, column-major, Z-negative camera (follows OpenGL conventions)
impl PerspectiveCamera {
    pub fn new(
        position: WorldPoint,
        forward: WorldVector,
        near_plane: f32,
        far_plane: f32,
        field_of_view_in_degrees: f32,
        aspect_ratio: f32,
    ) -> Self {
        let scale_factor = Self::calculate_scale(field_of_view_in_degrees);
        let aspect_corrected_scale_factor = scale_factor / aspect_ratio;

        let z_range = far_plane - near_plane;
        let z_remapping_coefficient_1 = -(far_plane + near_plane) / z_range;
        let z_remapping_coefficient_2 = -(2.0 * far_plane * near_plane) / z_range;

        let perspective_matrix = ViewToClipTransform::from_matrix_unchecked(Matrix4::from_cols(
            vec4!(aspect_corrected_scale_factor, 0.0, 0.0, 0.0),
            vec4!(0.0, scale_factor, 0.0, 0.0),
            vec4!(0.0, 0.0, z_remapping_coefficient_1, -1.0),
            vec4!(0.0, 0.0, z_remapping_coefficient_2, 0.0),
        ));

        let mut camera = Self {
            position,
            forward,
            up: WorldVector::Y,
            right: WorldVector::ZERO,
            perspective_matrix,
            view_matrix: WorldToViewTransform::IDENTITY,
        };

        camera.orthonormalize_camera_base();
        camera.update_view_matrix();

        camera
    }

    fn orthonormalize_camera_base(&mut self) {
        self.forward = self.forward.normalize();
        self.right =
            WorldVector::from(WorldVector::Y.to_vec3a().cross(self.forward.to_vec3a())).normalize();
        self.up =
            WorldVector::from(self.forward.to_vec3a().cross(self.right.to_vec3a())).normalize();
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = WorldToViewTransform::from_matrix_unchecked(Matrix4::from_cols(
            vec4!(self.right.x, self.right.y, self.right.z, 0.0),
            vec4!(self.up.x, self.up.y, self.up.z, 0.0),
            vec4!(-self.forward.x, -self.forward.y, -self.forward.z, 0.0),
            vec4!(
                -self.position.to_vector().dot(self.right),
                -self.position.to_vector().dot(self.up),
                self.position.to_vector().dot(self.forward),
                1.0
            ),
        ));
    }

    pub fn look_at_point(&mut self, target: &WorldPoint) {
        self.forward = (target - self.position).normalize();

        self.orthonormalize_camera_base();

        self.update_view_matrix();
    }

    pub fn look_at<T: Positionable>(&mut self, target: &T) {
        if let Some(target_bounded) = target.as_bounded() {
            let bounding_box = target_bounded.calculate_bounding_box();
            let center = bounding_box.min + (bounding_box.max - bounding_box.min) / 2.0;
            self.look_at_point(&center);
        } else {
            self.look_at_point(&target.get_position());
        }
    }

    fn calculate_scale(field_of_view_in_degrees: f32) -> f32 {
        (field_of_view_in_degrees.to_radians() / 2.0).tan().recip()
    }
}
