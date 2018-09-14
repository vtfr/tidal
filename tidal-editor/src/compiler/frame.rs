use tidal_core::vm::instruction::Instruction;

#[derive(Default, Clone)]
pub(crate) struct Frame {
    pub instructions: Vec<Instruction>,
}

impl Extend<Instruction> for Frame {
    fn extend<T: IntoIterator<Item = Instruction>>(&mut self, iter: T) {
        self.instructions.extend(iter.into_iter())
    }
}

impl<'a> Extend<&'a Instruction> for Frame {
    fn extend<T: IntoIterator<Item = &'a Instruction>>(&mut self, iter: T) {
        self.instructions.extend(iter.into_iter().copied())
    }
}

impl Extend<Frame> for Frame {
    fn extend<T: IntoIterator<Item = Frame>>(&mut self, iter: T) {
        for frame in iter.into_iter() {
            self.extend(frame.instructions.into_iter())
        }
    }
}
