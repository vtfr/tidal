use std::rc::Rc;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};

use crate::renderer::Mesh;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Point3::new(0.0, 0.0, -1.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            aspect: 1920.0 / 1080.0,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }
}

impl Camera {
    pub fn to_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.fov_y),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    // Transforms
    Translate(Vector3<f32>),
    Rotate(Vector3<f32>, f32),
    RevertTransform,
    // Entities
    SetCamera(Camera),
    AddObject(Rc<Mesh>),
    ResetTransform,
}

impl From<Command> for CommandList {
    fn from(command: Command) -> Self {
        let mut list = Self::new();
        list.add(command);
        list
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub mesh: Rc<Mesh>,
    pub transform: Matrix4<f32>,
}

#[derive(Debug, Clone)]
pub struct CommandList {
    transform_stack: Vec<Matrix4<f32>>,
    pub objects: Vec<Object>,
    pub camera: Camera,
}

impl FromIterator<CommandList> for CommandList {
    fn from_iter<T: IntoIterator<Item = CommandList>>(iter: T) -> Self {
        let mut command_list = CommandList::new();
        command_list
    }
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            transform_stack: vec![Matrix4::identity()],
            objects: vec![],
            camera: Default::default(),
        }
    }

    pub fn add(&mut self, c: Command) {
        match c {
            Command::Translate(translation) => self
                .transform_stack
                .push(Matrix4::from_translation(translation)),
            Command::Rotate(_, _) => {}
            Command::RevertTransform => {
                self.transform_stack.pop();
            }
            Command::SetCamera(camera) => self.camera = camera,
            Command::AddObject(mesh) => {
                let transform = *self.transform_stack.last().unwrap();

                self.objects.push(Object { mesh, transform });
            }
            Command::ResetTransform => {
                self.transform_stack.clear();
                self.transform_stack.push(Matrix4::identity());
            }
        }
    }
}
