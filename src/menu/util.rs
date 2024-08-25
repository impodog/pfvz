use crate::prelude::*;

pub(super) fn small_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(30.0 * UI_ZOOM_FACTOR)
}

pub(super) fn medium_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(40.0 * UI_ZOOM_FACTOR)
}

pub(super) fn large_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(50.0 * UI_ZOOM_FACTOR)
}

pub(super) fn title_text(str: impl Into<String>) -> egui::RichText {
    egui::RichText::new(str).size(60.0 * UI_ZOOM_FACTOR)
}

pub(super) fn medium_edit(str: &mut String) -> egui::TextEdit {
    let font_id = egui::FontId {
        family: egui::FontFamily::Proportional,
        size: 40.0,
    };
    egui::TextEdit::singleline(str).font(font_id)
}
