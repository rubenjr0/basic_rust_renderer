use nalgebra::{Matrix3, Matrix3x4, Matrix4, Translation3, Vector2, Vector3};

pub struct Camera {
    perspective: Matrix3x4<f32>,
    translation: Translation3<f32>,
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl Camera {
    pub fn new(f: f32, k_x: f32, k_y: f32) -> Self {
        let s_x = f * k_x;
        let s_y = f * k_y;
        let u_0 = k_x / 2.0;
        let v_0 = k_y / 2.0;
        let k = Matrix3::new(s_x, 0.0, u_0, 0.0, s_y, v_0, 0.0, 0.0, 1.0);
        let p = Matrix3x4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let perspective = k * p;
        let translation = Translation3::default();

        Self {
            perspective,
            translation,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.translation.vector += v;
    }

    pub fn reset(&mut self) {
        self.translation = Translation3::default();
        self.pitch = 0.0;
        self.yaw = 0.0;
        self.roll = 0.0;
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch += pitch.to_radians();
        self.yaw += yaw.to_radians();
        self.roll += roll.to_radians();
    }

    fn yaw_rotation(&self) -> Matrix3<f32> {
        let c = self.yaw.cos();
        let s = self.yaw.sin();
        Matrix3::new(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0)
    }

    fn pitch_rotation(&self) -> Matrix3<f32> {
        let c = self.pitch.cos();
        let s = self.pitch.sin();
        Matrix3::new(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c)
    }

    fn roll_rotation(&self) -> Matrix3<f32> {
        let c = self.roll.cos();
        let s = self.roll.sin();
        Matrix3::new(1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c)
    }

    fn rotation(&self) -> Matrix3<f32> {
        self.yaw_rotation() * self.pitch_rotation() * self.roll_rotation()
    }

    pub fn transformation(&self) -> Matrix4<f32> {
        let r = self.rotation();
        Matrix4::new(
            r.m11,
            r.m12,
            r.m13,
            self.translation.x,
            r.m21,
            r.m22,
            r.m23,
            self.translation.y,
            r.m31,
            r.m32,
            r.m33,
            self.translation.z,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    pub fn project(&self, p: &Vector3<f32>) -> Vector2<f32> {
        let p = self.perspective * self.transformation() * p.push(1.0);
        let p = p / p[2];
        Vector2::new(p.x, p.y)
    }
}
