use std::sync::Arc;

use eframe::egui::style::{Spacing, WidgetVisuals};
use eframe::egui::{FontId, Margin, Rect, Rounding, Stroke, Style, Ui, Vec2, Visuals};
use eframe::epaint::Shadow;

pub struct Zoom {
    zoom: f32,
    clip_rect: Option<Rect>,
}

impl Zoom {
    #[must_use]
    pub fn new(zoom: f32) -> Self {
        Self {
            zoom,
            clip_rect: None,
        }
    }

    #[must_use]
    pub fn clip_rect(mut self, clip_rect: Rect) -> Self {
        self.clip_rect = Some(clip_rect);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let style = ui.style().clone();
        let scaled = style.scaled(self.zoom);

        ui.set_style(scaled);
        let r = add_contents(ui);
        ui.set_style(style);

        r
    }
}

trait Scale {
    fn scale(&mut self, factor: f32);

    fn scaled(&self, factor: f32) -> Self
    where
        Self: Clone,
    {
        let mut scaled = self.clone();
        scaled.scale(factor);
        scaled
    }
}

impl Scale for Vec2 {
    fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl Scale for Margin {
    fn scale(&mut self, factor: f32) {
        self.left *= factor;
        self.right *= factor;
        self.top *= factor;
        self.bottom *= factor;
    }
}

impl Scale for Rounding {
    fn scale(&mut self, factor: f32) {
        self.ne *= factor;
        self.nw *= factor;
        self.se *= factor;
        self.sw *= factor;
    }
}

impl Scale for Stroke {
    fn scale(&mut self, factor: f32) {
        self.width *= factor;
    }
}

impl Scale for Shadow {
    fn scale(&mut self, factor: f32) {
        self.extrusion *= factor.clamp(0.4, 1.);
    }
}

impl Scale for WidgetVisuals {
    fn scale(&mut self, factor: f32) {
        self.bg_stroke.scale(factor);
        self.fg_stroke.scale(factor);
        self.rounding.scale(factor);
        self.expansion *= factor.clamp(0.4, 1.);
    }
}

impl Scale for Style {
    fn scale(&mut self, factor: f32) {
        if let Some(ov_font_id) = &mut self.override_font_id {
            ov_font_id.size *= factor;
        }

        for text_style in self.text_styles.values_mut() {
            text_style.size *= factor;
        }

        self.spacing.item_spacing.scale(factor);
        self.spacing.window_margin.scale(factor);
        self.spacing.button_padding.scale(factor);
        self.spacing.indent *= factor;
        self.spacing.interact_size.scale(factor);
        self.spacing.slider_width *= factor;
        self.spacing.text_edit_width *= factor;
        self.spacing.icon_width *= factor;
        self.spacing.icon_width_inner *= factor;
        self.spacing.icon_spacing *= factor;
        self.spacing.tooltip_width *= factor;
        self.spacing.combo_height *= factor;
        self.spacing.scroll_bar_width *= factor;

        self.interaction.resize_grab_radius_side *= factor;
        self.interaction.resize_grab_radius_corner *= factor;

        self.visuals.widgets.noninteractive.scale(factor);
        self.visuals.widgets.inactive.scale(factor);
        self.visuals.widgets.hovered.scale(factor);
        self.visuals.widgets.active.scale(factor);
        self.visuals.widgets.open.scale(factor);

        self.visuals.selection.stroke.scale(factor);

        self.visuals.resize_corner_size *= factor;
        self.visuals.text_cursor.scale(factor);
        self.visuals.clip_rect_margin *= factor;
        self.visuals.window_rounding.scale(factor);
        self.visuals.window_shadow.scale(factor);
        self.visuals.popup_shadow.scale(factor);
    }
}
