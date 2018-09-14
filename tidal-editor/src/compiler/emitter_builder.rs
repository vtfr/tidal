use derive_more::From;
use tidal_core::vm::instruction::Instruction;

use crate::compiler::{CompilationResult, CompileError, Compiler, FrameName};
use crate::graph::{Constant, Input, NodeId, NodePortId};
use crate::operator::PortId;

#[derive(Copy, Clone)]
pub(crate) struct EmissionContext<'a> {
    pub frame_name: &'a FrameName,
    pub node_id: NodeId,
    pub output_id: PortId,
}

pub(crate) fn emit_children<'a, BeforeChildFn, AfterChildFn>(
    before_child: BeforeChildFn,
    after_child: AfterChildFn,
) -> impl Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> + 'a
where
    BeforeChildFn: Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> + 'a,
    AfterChildFn: Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> + 'a,
{
    move |compiler, context| {
        let node = compiler
            .graph
            .state
            .nodes
            .get(&context.node_id)
            .ok_or_else(|| CompileError::UnknownNode(context.node_id))?;

        for (_, input) in node.inputs_iter() {
            before_child(compiler, context)?;

            match &*input {
                Input::Constant(c) => {
                    let mut frame = compiler
                        .get_function_frame_mut(context.frame_name)
                        .ok_or_else(|| {
                            CompileError::UnknownFrameName(context.frame_name.to_string())
                        })?;

                    match c {
                        &Constant::F32(c) => {
                            frame.extend(core::iter::once(Instruction::PushF32(c)));
                        }
                        &Constant::I32(c) => {
                            frame.extend(core::iter::once(Instruction::PushI32(c)));
                        }
                    }
                }
                Input::Connection(children) => {
                    for child in children.iter() {
                        compiler.emit_node(EmissionContext {
                            frame_name: context.frame_name,
                            node_id: child.get_node_id(),
                            output_id: child.get_port_id(),
                        })?;
                    }
                }
            }

            after_child(compiler, context)?;
        }

        Ok(())
    }
}

pub(crate) fn emit_noop(
    compiler: &mut Compiler,
    context: EmissionContext,
) -> CompilationResult<()> {
    Ok(())
}

pub(crate) fn emit_children_simple(
) -> impl Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> {
    emit_children(emit_noop, emit_noop)
}

pub(crate) fn emit_instruction_with_initializer<InitializerFn>(
    compiler: &mut Compiler,
    context: EmissionContext,
    initializer_fn: InitializerFn,
) -> CompilationResult<()>
where
    InitializerFn: FnOnce(&mut Compiler, EmissionContext) -> CompilationResult<()>,
{
    if !compiler.initialized.contains(&context.node_id) {
        compiler.initialized.insert(context.node_id);

        initializer_fn(compiler, context)?;
    }

    let output = (context.node_id, context.output_id).into();

    let instructions = compiler.cached.get(&output).unwrap().clone();

    compiler.emit_instructions(context.frame_name, instructions.into_iter())
}

pub(crate) fn emit_instructions<'a>(
    instructions: &'a [Instruction],
) -> impl Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> + 'a {
    |compiler, context| {
        let frame = compiler
            .get_function_frame_mut(context.frame_name)
            .ok_or_else(|| CompileError::UnknownFrameName(context.frame_name.to_string()))?;

        frame.instructions.extend(instructions.into_iter());
        Ok(())
    }
}

pub(crate) fn always_emit_instruction<'a>(
    instruction: Instruction,
) -> impl Fn(&mut Compiler, EmissionContext) -> CompilationResult<()> + 'a {
    move |compiler, context| {
        let frame = compiler
            .get_function_frame_mut(context.frame_name)
            .ok_or_else(|| CompileError::UnknownFrameName(context.frame_name.to_string()))?;

        frame.instructions.extend(Some(instruction));
        Ok(())
    }
}
