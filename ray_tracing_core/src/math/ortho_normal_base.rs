use crate::types::Vector3;

pub struct OrthoNormalBase {
    pub axis: [Vector3; 3],
}

impl OrthoNormalBase {
    pub fn form_w(n: &Vector3) -> OrthoNormalBase {
        let axis_z = glm::normalize(*n);
        let a = if axis_z.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let axis_y = glm::normalize(glm::cross(axis_z, a));
        let axis_x = glm::cross(axis_z, axis_y);
        OrthoNormalBase {
            axis: [axis_x, axis_y, axis_z],
        }
    }

    pub fn u(&self) -> Vector3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vector3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vector3 {
        self.axis[2]
    }

    pub fn local(&self, a: Vector3) -> Vector3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}
