use crate::common::space::{ScreenPoint, ScreenScalar, ScreenSpace};
use glamour::prelude::*;
use num::traits::ConstOne;

struct BarycentricPrecomputes {
    ab_edge: Vector2<ScreenSpace>,
    ac_edge: Vector2<ScreenSpace>,
    dot_ab: ScreenScalar,
    dot_ac: ScreenScalar,
    dot_ab_ac: ScreenScalar,
    cramer_denominator_inv: ScreenScalar,
}

pub struct PolygonPoints2 {
    points: [ScreenPoint; 3],
    barycentric_precomputes: BarycentricPrecomputes,
}

impl PolygonPoints2 {
    const POINT_A_IDX: usize = 0;
    const POINT_B_IDX: usize = 1;
    const POINT_C_IDX: usize = 2;

    pub fn new(points: [ScreenPoint; 3]) -> Self {
        let ab_edge = points[Self::POINT_B_IDX] - points[Self::POINT_A_IDX];
        let ac_edge = points[Self::POINT_C_IDX] - points[Self::POINT_A_IDX];
        let dot_ab = ab_edge.dot(ab_edge);
        let dot_ac = ac_edge.dot(ac_edge);
        let dot_ab_ac = ab_edge.dot(ac_edge);
        let cramer_denominator_inv = (dot_ab * dot_ac - dot_ab_ac * dot_ab_ac).recip();

        Self {
            points,
            barycentric_precomputes: BarycentricPrecomputes {
                ab_edge,
                ac_edge,
                dot_ab,
                dot_ac,
                dot_ab_ac,
                cramer_denominator_inv,
            },
        }
    }

    pub fn barycentric(&self, probe_point: ScreenPoint) -> Option<Vector3<ScreenSpace>> {
        if self
            .barycentric_precomputes
            .cramer_denominator_inv
            .is_infinite()
        {
            return None;
        };

        let ap_vec = probe_point - self.points[Self::POINT_A_IDX];

        let dot_ap_ab = ap_vec.dot(self.barycentric_precomputes.ab_edge);
        let dot_ap_ac = ap_vec.dot(self.barycentric_precomputes.ac_edge);

        let v = (self.barycentric_precomputes.dot_ac * dot_ap_ab
            - self.barycentric_precomputes.dot_ab_ac * dot_ap_ac)
            * self.barycentric_precomputes.cramer_denominator_inv;
        let w = (self.barycentric_precomputes.dot_ab * dot_ap_ac
            - self.barycentric_precomputes.dot_ab_ac * dot_ap_ab)
            * self.barycentric_precomputes.cramer_denominator_inv;
        let u = ScreenScalar::ONE - v - w;

        let uvw = Vector3::<ScreenSpace>::new(u, v, w);

        if uvw.cmpge(Vector3::<ScreenSpace>::ZERO).all() {
            Some(uvw)
        } else {
            None
        }
    }

    pub fn bounding_box(&self) -> Rect<ScreenSpace> {
        Rect::<ScreenSpace>::from_points(self.points)
    }
}
