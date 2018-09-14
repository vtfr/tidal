use std::rc::Rc;

use cgmath::{EuclideanSpace, Point3, Vector3};

use tidal_core_derive::evaluator;

use crate::interpreter::{EvaluateContext, Multiple};
use crate::renderer::{
    Camera, Command, CommandList, Mesh, MeshDescriptor, ScreenRenderPass, Vertex,
};

#[derive(Default)]
pub(crate) struct MeshEvaluator {
    mesh: Option<Rc<Mesh>>,
}

#[evaluator(impl MeshEvaluator for Mesh)]
#[output(name = "mesh")]
fn evaluate_mesh(
    #[state] state: &mut MeshEvaluator,
    #[context] ctx: &mut EvaluateContext,
) -> Rc<Mesh> {
    state
        .mesh
        .get_or_insert_with(|| {
            let mesh = ctx.renderer().create_mesh(&MeshDescriptor {
                vertices: &[
                    Vertex {
                        positions: [0.0, 0.8, 0.0],
                        normals: [0.0, 0.0, 0.0],
                        uv: [0.0, 0.0],
                    },
                    Vertex {
                        positions: [-0.5, -0.5, 0.0],
                        normals: [0.0, 0.0, 0.0],
                        uv: [0.0, 0.0],
                    },
                    Vertex {
                        positions: [0.5, -0.5, 0.0],
                        normals: [0.0, 0.0, 0.0],
                        uv: [0.0, 0.0],
                    },
                ],
            });

            Rc::new(mesh)
        })
        .clone()
}

#[derive(Debug, Default)]
pub struct SceneEvaluator {
    render_pass: Option<ScreenRenderPass>,
}

#[evaluator(impl SceneEvaluator for Scene)]
fn evaluate_scene(
    #[state] state: &mut SceneEvaluator,
    #[context] ctx: &mut EvaluateContext,
    command_list: CommandList,
) {
    let render_pass = state
        .render_pass
        .get_or_insert_with(|| ctx.renderer().create_render_pass::<ScreenRenderPass>());

    let x = ctx.interpreter_context.render_target;
    ctx.renderer().render(render_pass, &command_list, (x))
}

#[evaluator(CameraEvaluator for Camera)]
#[output(name = "commands")]
fn camera(
    mut command_list: CommandList,
    #[default(0.0, 0.0, - 1.0)] eye: Vector3<f32>,
    #[default(0.0, 0.0, 0.0)] target: Vector3<f32>,
    #[default(0.0, 1.0, 0.0)] up: Vector3<f32>,
    #[default(1.777)] aspect: f32,
    #[default(80.0)] fov_y: f32,
    #[default(0.1)] z_near: f32,
    #[default(1000.0)] z_far: f32,
) -> CommandList {
    let camera = Camera {
        eye: Point3::from_vec(eye),
        target: Point3::from_vec(target),
        up,
        aspect: aspect.max(1.0),
        fov_y: fov_y.max(1.0),
        z_near: z_near.max(0.1),
        z_far: z_far.max(1.0),
    };

    command_list.add(Command::SetCamera(camera));
    command_list
}

#[evaluator(MultiplyEvaluator for Multiply)]
#[output(name = "value")]
fn evaluate_multiply(a: f32, b: f32) -> f32 {
    a * b
}

#[evaluator(RenderTargetEvaluator for RenderTarget)]
#[output(name = "value")]
fn render_target(a: f32, b: f32) -> f32 {
    a * b
}
