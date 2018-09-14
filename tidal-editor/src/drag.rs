use derive_more::Constructor;
use eframe::egui;
use eframe::egui::{vec2, PointerButton, Response, Vec2};

#[derive(Default, Copy, Clone)]
struct AccumulatedDragState {
    pub distance: f32,
    pub delta: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct MinimumDrag {
    minimum_distance: f32,
    pointer_button: PointerButton,
}

impl Default for MinimumDrag {
    fn default() -> Self {
        Self {
            minimum_distance: 5.0,
            pointer_button: PointerButton::Primary,
        }
    }
}

#[derive(Default)]
pub struct MinimumDragResponse {
    pub started: bool,
    pub dragged: bool,
    pub delta: Vec2,
}

impl MinimumDrag {
    pub fn pointer_button(mut self, pointer_button: PointerButton) -> Self {
        self.pointer_button = pointer_button;
        self
    }

    pub fn minimum_distance(mut self, minimum_distance: f32) -> Self {
        self.minimum_distance = minimum_distance;
        self
    }

    pub fn handle(self, r: &Response) -> MinimumDragResponse {
        fn remove_state(r: &Response) {
            r.ctx
                .data_mut(|data| data.remove_by_type::<AccumulatedDragState>())
        }

        // Prepared accumulating
        if r.drag_started_by(self.pointer_button) {
            r.ctx
                .data_mut(|data| data.insert_temp(r.id, AccumulatedDragState::default()));
        }

        // Accumulate dragging until it passes the minimum distance
        if let Some(mut state) = r
            .ctx
            .data(|data| data.get_temp::<AccumulatedDragState>(r.id))
        {
            // Drag released before accumulating enough drag. Ignore.
            if r.drag_released_by(self.pointer_button) {
                remove_state(r);
                return MinimumDragResponse::default();
            }

            // Accumulate
            state.distance += r.drag_delta().length();
            state.delta += r.drag_delta();

            r.ctx.data_mut(|data| data.insert_temp(r.id, state));

            if state.distance >= self.minimum_distance {
                remove_state(r);

                MinimumDragResponse {
                    started: true,
                    dragged: true,
                    delta: state.delta,
                }
            } else {
                MinimumDragResponse::default()
            }
        } else {
            MinimumDragResponse {
                started: r.drag_started_by(self.pointer_button),
                dragged: r.dragged_by(self.pointer_button),
                delta: r.drag_delta(),
            }
        }
    }
}
