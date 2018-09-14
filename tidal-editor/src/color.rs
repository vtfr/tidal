#[macro_export]
macro_rules! rgba {
    ($color:expr) => {{
        let [r, g, b, a]: [u8; 4] = hex_literal::hex!($color);
        eframe::egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }};
}

#[macro_export]
macro_rules! rgb {
    ($color:expr) => {{
        let [r, g, b]: [u8; 3] = hex_literal::hex!($color);
        eframe::egui::Color32::from_rgb(r, g, b)
    }};
}
