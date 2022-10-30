use nalgebra::{matrix, ArrayStorage, Const, Matrix, Point3, Vector3};

pub struct ModelMatrix {
    pub matrix: Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>,
    stack: Vec<Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>>,
}

impl ModelMatrix {
    pub fn new() -> ModelMatrix {
        let matrix = matrix![1.0, 0.0, 0.0, 0.0;
                            0.0, 1.0, 0.0, 0.0;
                            0.0, 0.0, 1.0, 0.0;
                            0.0, 0.0, 0.0, 1.0];

        return ModelMatrix {
            matrix,
            stack: Vec::new(),
        };
    }

    pub fn load_identity(&mut self) {
        self.matrix = matrix![1.0, 0.0, 0.0, 0.0;
        0.0, 1.0, 0.0, 0.0;
        0.0, 0.0, 1.0, 0.0;
        0.0, 0.0, 0.0, 1.0];
    }

    pub fn add_transformation(
        &mut self,
        transform_mat: &Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>,
    ) {
        self.matrix = self.matrix * transform_mat;
    }

    pub fn add_nothing(&mut self) {
        self.add_transformation(&matrix![1.0, 0.0, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0;
            0.0, 0.0, 0.0, 1.0]);
    }

    pub fn add_translate(&mut self, x: f32, y: f32, z: f32) {
        self.add_transformation(&matrix![1.0, 0.0, 0.0, x;
            0.0, 1.0, 0.0, y;
            0.0, 0.0, 1.0, z;
            0.0, 0.0, 0.0, 1.0])
    }

    pub fn add_scale(&mut self, x: f32, y: f32, z: f32) {
        self.add_transformation(&matrix![x, 0.0, 0.0, 0.0;
            0.0, y, 0.0, 0.0;
            0.0, 0.0, z, 0.0;
            0.0, 0.0, 0.0, 1.0]);
    }

    pub fn add_rotation(&mut self, x: f32, y: f32, z: f32) {
        let x_matrix = matrix![1.0, 0.0, 0.0, 0.0;
                    0.0, f32::cos(x), -f32::sin(x), 0.0;
                    0.0, f32::sin(x), f32::cos(x), 0.0;
                    0.0, 0.0, 0.0, 1.0];
        let y_matrix = matrix![f32::cos(y), 0.0, f32::sin(y), 0.0;
                    0.0, 1.0, 0.0, 0.0;
                    -f32::sin(y), 0.0, f32::cos(y), 0.0;
                    0.0, 0.0, 0.0, 1.0];
        let z_matrix = matrix![f32::cos(z), -f32::sin(z), 0.0, 0.0;
                    f32::sin(z), f32::cos(z), 0.0, 0.0;
                    0.0, 0.0, 1.0, 0.0;
                    0.0, 0.0, 0.0, 1.0];

        self.add_transformation(&x_matrix);
        self.add_transformation(&y_matrix);
        self.add_transformation(&z_matrix);
    }

    pub fn push_stack(&mut self) {
        self.stack.push(self.matrix.clone());
    }

    pub fn pop_stack(&mut self) {
        match self.stack.pop() {
            Some(m) => self.matrix = m,
            None => (),
        }
    }
}

pub struct ViewMatrix {
    pub eye: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub n: Vector3<f32>,
}

impl ViewMatrix {
    pub fn new() -> ViewMatrix {
        ViewMatrix {
            eye: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(1.0, 0.0, 0.0),
            v: Vector3::new(0.0, 1.0, 0.0),
            n: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn look(&mut self, eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) {
        self.eye = eye;
        self.n = eye - center;
        self.u = up.cross(&self.n);
        self.v = self.n.cross(&self.u);
    }

    pub fn slide(
        &mut self,
        del_u: f32,
        del_v: f32,
        del_n: f32,
        u: Vector3<f32>,
        v: Vector3<f32>,
        n: Vector3<f32>,
    ) {
        self.eye.x += del_u * u.x + del_v * v.x + del_n * n.x;
        self.eye.y += del_u * u.y + del_v * v.y + del_n * n.y;
        self.eye.z += del_u * u.z + del_v * v.z + del_n * n.z;
    }

    pub fn roll(&mut self, angle: f32) {
        let angle = angle.to_radians();
        let t = self.u.clone();

        self.u = angle.cos() * t + angle.sin() * self.v;
        self.v = (-angle).sin() * t + angle.cos() * self.v;
    }

    pub fn yaw(&mut self, angle: f32) {
        let angle = angle.to_radians();
        let t = self.n.clone();

        self.n = angle.cos() * t + angle.sin() * self.u;
        self.u = (-angle).sin() * t + angle.cos() * self.u;
    }

    pub fn pitch(&mut self, angle: f32) {
        let angle = angle.to_radians();
        let t = self.v.clone();

        self.v = angle.cos() * t + angle.sin() * self.n;
        self.n = (-angle).sin() * t + angle.cos() * self.n;
    }

    pub fn get_matrix(&self) -> Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>> {
        let minus_eye = -self.eye;

        matrix![self.u.x, self.u.y, self.u.z, minus_eye.dot(&self.u);
                self.v.x, self.v.y, self.v.z, minus_eye.dot(&self.v);
                self.n.x, self.n.y, self.n.z, minus_eye.dot(&self.n);
                0.0, 0.0, 0.0, 1.0]
    }
}

pub struct ProjectionMatrix {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl ProjectionMatrix {
    pub fn new() -> ProjectionMatrix {
        ProjectionMatrix {
            left: -2.0,
            right: 2.0,
            bottom: -2.0,
            top: 2.0,
            near: 0.5,
            far: 10.0,
        }
    }

    pub fn set_perspective(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        let fov_r = fov.to_radians();
        self.top = near * f32::tan(fov_r / 2.0);
        self.bottom = self.top * -1.0;
        self.right = self.top * aspect_ratio;
        self.left = self.right * -1.0;
        self.near = near;
        self.far = far;
    }

    pub fn set_orthographic(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self.near = near;
        self.far = far;
    }

    pub fn get_matrix(&self) -> Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>> {
        let a = (2.0 * self.near) / (self.right - self.left);
        let b = (self.right + self.left) / (self.right - self.left);
        let c = (2.0 * self.near) / (self.top - self.bottom);
        let d = (self.top + self.bottom) / (self.top - self.bottom);
        let e = -(self.far + self.near) / (self.far - self.near);
        let f = -(2.0 * self.near * self.far) / (self.far - self.near);

        matrix![a, 0.0, b, 0.0;
                0.0, c, d, 0.0;
                0.0, 0.0, e, f;
                0.0, 0.0, -1.0, 0.0]
    }

    pub fn get_matrix_notransform(&self) -> Matrix<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>> {
        let a = (2.0 * self.near) / (self.right - self.left);
        let b = (self.right + self.left) / (self.right - self.left);
        let c = (2.0 * self.near) / (self.top - self.bottom);
        let d = (self.top + self.bottom) / (self.top - self.bottom);
        let e = -(self.far + self.near) / (self.far - self.near);

        matrix![a, 0.0, b, 0.0;
                0.0, c, d, 0.0;
                0.0, 0.0, e, 0.0;
                0.0, 0.0, 0.0, 0.0;]
    }
}
