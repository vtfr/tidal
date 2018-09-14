use std::collections::{HashMap, HashSet};

use derive_more::Display;
use nom::Parser;
use tidal_core::renderer::Vertex;
use tidal_core::vm::instruction::Instruction;
use tidal_core::vm::program::{Program, Resources};

use crate::compiler::{
    emit_children_simple, emit_instruction_with_initializer, emit_instructions, EmissionContext,
    Frame, OutputUsageTracker, ResourceAllocator,
};
use crate::graph::{Graph, NodeId, NodePortId};
use crate::operator::{Operator, PortId};

pub type CompilationResult<T> = Result<T, CompileError>;

pub struct Compiler<'a> {
    pub(crate) graph: &'a Graph,

    /// Init function frame
    pub(crate) init_fn_frame: Frame,

    /// Frame fn frame
    pub(crate) frame_fn_frame: Frame,

    pub(crate) output_usage_tracker: OutputUsageTracker,

    /// Keeps track of the next allocation address
    pub(crate) next_variable_allocation_address: usize,

    /// Keeps track of allocated resources
    pub(crate) resource_allocator: ResourceAllocator,

    /// Keeps track of initialized nodes so we don't initialize
    /// them multiple times.
    pub(crate) initialized: HashSet<NodeId>,

    /// Keeps track of the cached instructions
    pub(crate) cached: HashMap<NodePortId, Vec<Instruction>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub(crate) enum FrameName {
    #[display(fmt = "init")]
    Init,
    #[display(fmt = "frame")]
    Frame,
}

impl<'a> Compiler<'a> {
    pub fn compile(graph: &Graph) -> Result<Program, CompileError> {
        let mut compiler = Compiler::new(graph);

        compiler.emit_node(EmissionContext {
            frame_name: &FrameName::Frame,
            node_id: NodeId::root(),
            output_id: PortId::nil(),
        })?;

        // Add return to the init function
        compiler
            .init_fn_frame
            .instructions
            .extend(&[Instruction::Return]);

        // Add return to the frame function
        compiler
            .frame_fn_frame
            .instructions
            .extend(&[Instruction::Return]);

        // Calculate the frame_address
        let frame_address = compiler.init_fn_frame.instructions.len();

        let code = compiler
            .init_fn_frame
            .instructions
            .into_iter()
            .chain(compiler.frame_fn_frame.instructions.into_iter())
            .collect::<Vec<Instruction>>();

        // Resources
        let resources = Resources {
            data: compiler.resource_allocator.resources,
        };

        Ok(Program {
            resources,
            code: code.into_boxed_slice(),
            frame_address,
        })
    }

    pub(crate) fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            init_fn_frame: Default::default(),
            frame_fn_frame: Default::default(),
            output_usage_tracker: OutputUsageTracker::new(graph),
            next_variable_allocation_address: 0,
            resource_allocator: Default::default(),
            initialized: Default::default(),
            cached: Default::default(),
        }
    }

    pub(crate) fn get_function_frame_mut(&mut self, name: &FrameName) -> Option<&mut Frame> {
        match name {
            FrameName::Init => Some(&mut self.init_fn_frame),
            FrameName::Frame => Some(&mut self.frame_fn_frame),
            _ => None,
        }
    }

    pub(crate) fn emit_instructions(
        &mut self,
        name: &FrameName,
        i: impl IntoIterator<Item = Instruction>,
    ) -> CompilationResult<()> {
        let frame = self
            .get_function_frame_mut(name)
            .ok_or_else(|| CompileError::UnknownFrameName(name.to_string()))?;

        frame.instructions.extend(i.into_iter());
        Ok(())
    }

    pub(crate) fn emit_node(&mut self, context: EmissionContext) -> CompilationResult<()> {
        let node = self
            .graph
            .state
            .nodes
            .get(&context.node_id)
            .ok_or_else(|| CompileError::UnknownNode(context.node_id))?;

        match node.operator {
            Operator::Time => {
                emit_children_simple()(self, context)?;
                emit_instructions(&[Instruction::PushTime])(self, context)?;
                Ok(())
            }
            Operator::Sin => {
                emit_children_simple()(self, context)?;
                emit_instructions(&[Instruction::Sin])(self, context)?;
                Ok(())
            }
            Operator::Cos => {
                emit_children_simple()(self, context)?;
                emit_instructions(&[Instruction::Cos])(self, context)?;
                Ok(())
            }
            Operator::Remap => {
                emit_children_simple()(self, context)?;
                emit_instructions(&[Instruction::Remap])(self, context)?;
                Ok(())
            }
            Operator::Triangle => {
                emit_instruction_with_initializer(self, context, |compiler, context| {
                    let resource = compiler.resource_allocator.add_mesh(&[
                        Vertex {
                            positions: [0.0_f32, 0.6, 0.0],
                            normals: [0.0, 0.0, 0.0],
                            uv: [0.0, 0.0],
                        },
                        Vertex {
                            positions: [-0.5, -0.6, 0.0],
                            normals: [0.0, 0.0, 0.0],
                            uv: [0.0, 0.0],
                        },
                        Vertex {
                            positions: [0.5, -0.6, 0.0],
                            normals: [0.0, 0.0, 0.0],
                            uv: [0.0, 0.0],
                        },
                    ]);

                    let mesh_address = compiler.allocate_variable();

                    // Initialize
                    compiler.init_fn_frame.extend(&[
                        Instruction::PushI32(0),
                        Instruction::CreateMesh,
                        Instruction::Store(mesh_address as u8),
                    ]);

                    let mesh_port = (context.node_id, context.output_id).into();
                    compiler
                        .cached
                        .insert(mesh_port, vec![Instruction::Load(mesh_address as u8)]);

                    Ok(())
                })
            }
            Operator::Draw => {
                emit_children_simple()(self, context)?;
                self.frame_fn_frame
                    .extend(&[Instruction::SAddObjectToScene]);

                Ok(())
            }
            Operator::DrawTexture => Ok(()),
            Operator::PrincipalPass => {
                emit_children_simple()(self, context)?;
                emit_instruction_with_initializer(self, context, |compiler, context| {
                    let resource = compiler
                        .resource_allocator
                        .add_shader(include_str!("../../../tidal_player/src/shader.wgsl"));

                    let pipeline_address = compiler.allocate_variable();

                    // Initialize
                    compiler.init_fn_frame.extend(&[
                        Instruction::PushI32(resource as i32),
                        Instruction::CreateShaderModule,
                        Instruction::CreatePrincipalRenderPass,
                        Instruction::Store(pipeline_address as u8),
                    ]);

                    let output = (context.node_id, context.output_id).into();
                    compiler.cached.insert(
                        output,
                        vec![
                            Instruction::Load(pipeline_address as u8),
                            Instruction::Render,
                        ],
                    );

                    Ok(())
                })
            }
            Operator::Shader(_) => Ok(()),
            Operator::Scene => emit_children_simple()(self, context),
        }
    }

    pub(crate) fn allocate_variable(&mut self) -> usize {
        let address = self.next_variable_allocation_address;
        self.next_variable_allocation_address += 1;
        address
    }
}

#[derive(Display, Debug)]
pub enum CompileError {
    #[display(fmt = "unknown node")]
    UnknownNode(NodeId),
    #[display(fmt = "unknown frame")]
    UnknownFrameName(String),
    #[display(fmt = "unexpected frame")]
    UnexpectedFrame(String),
}
