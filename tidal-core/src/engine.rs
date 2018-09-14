// #[derive(Debug)]
// pub struct Engine<R>
// where
//     R: Renderer,
// {
//     pub renderer: R,
//     pub interpreter: Interpreter,
// }
//
// impl<R> Engine<R>
// where
//     R: Renderer,
// {
//     pub fn initialize(&mut self) -> InterpretResult<()> {
//         self.interpreter.run_init(&mut InterpretContext {
//             renderer: &mut self.renderer,
//         })
//     }
//
//     pub fn run(&mut self) -> InterpretResult<()> {
//         self.interpreter.run_frame(&mut InterpretContext {
//             renderer: &mut self.renderer,
//         })
//     }
// }
