// https://www.xarg.org/2021/07/trackball-rotation-using-quaternions/
use crate::math::{Distance, Vector3};
use glam::{vec3, Quat};

#[derive(Debug)]
pub struct Trackball {
    start_x: f32,
    start_y: f32,
    curr_x: f32,
    curr_y: f32,
    base_rot: Quat,
    active: bool,
}

fn project(x: f32, y: f32) -> Vector3 {
    if x * x + y * y <= 1.0 / 2.0 {
        Vector3::new(x, -y, (1.0 - x * x - y * y).sqrt())
    } else {
        Vector3::new(x, -y, 0.5 / (x * x + y * y).sqrt())
    }
}

impl Trackball {
    pub fn default() -> Self {
        Trackball {
            start_x: 0.,
            start_y: 0.,
            curr_x: 0.,
            curr_y: 0.,
            base_rot: Quat::IDENTITY,
            active: false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn start(&mut self, x: f32, y: f32) {
        self.start_x = x;
        self.start_y = y;
        self.curr_x = x;
        self.curr_y = y;
        self.active = true;
    }

    pub fn set_xy(&mut self, x: f32, y: f32) {
        self.curr_x = x;
        self.curr_y = y;
        println!(
            "{:?}, {:?}, {:?}, {:?}, sv: {:?}, cv: {:?}",
            self.start_x,
            self.start_y,
            self.curr_x,
            self.curr_y,
            project(self.start_x, self.start_y),
            project(self.curr_x, self.curr_y)
        );
    }

    pub fn end(&mut self, x: f32, y: f32) {
        self.set_xy(x, y);
        self.base_rot = self.view_rotation();
        self.start_x = x;
        self.start_y = y;
        self.active = false;
    }

    pub fn view_rotation(&self) -> Quat {
        if self.start_x == self.curr_x && self.start_y == self.curr_y {
            self.base_rot
        } else {
            let p = project(self.start_x, self.start_y);
            let q = project(self.curr_x, self.curr_y);
            let axis = p.cross(&q).normalize();
            let angle = p.dot(&q).acos();
            Quat::from_axis_angle(vec3(axis.x(), axis.y(), axis.z()), angle).mul_quat(self.base_rot)
        }
    }
}
