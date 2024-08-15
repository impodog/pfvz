use crate::prelude::*;

pub(super) fn small_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(30.0 * UI_ZOOM_FACTOR)
}

pub(super) fn medium_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(40.0 * UI_ZOOM_FACTOR)
}
