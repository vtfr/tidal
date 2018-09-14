use eframe::egui::{InnerResponse, Ui};

pub trait ResponseWidget {
    type Response;

    fn show(self, ui: &mut Ui) -> Self::Response;
}
