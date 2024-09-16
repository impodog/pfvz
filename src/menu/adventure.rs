use crate::prelude::*;
use egui_extras::{Column, TableBuilder};

pub(super) struct MenuAdventurePlugin;

impl Plugin for MenuAdventurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::MenuStates::Adventure), (init_adventure,));
        app.add_systems(
            Update,
            (adventure_menu, adventure_direct_start).run_if(when_state!(adventure)),
        );
    }
}

fn adventure_direct_start(mut e_level: EventWriter<level::LevelEvent>, save: Res<save::Save>) {
    if save.adventure.is_empty() {
        e_level.send(level::LevelEvent {
            index: level::LevelIndex { stage: 0, level: 0 },
        });
    }
}

#[derive(Resource, Debug)]
struct AdventureInfo {
    unlocked: BTreeMap<u32, BTreeSet<u32>>,
}

fn init_adventure(mut commands: Commands, save: Res<save::Save>) {
    let mut unlocked = BTreeMap::new();
    save.adventure.iter().for_each(|index| {
        unlocked
            .entry(index.stage)
            .or_insert(BTreeSet::default())
            .insert(index.level);
    });
    commands.insert_resource(AdventureInfo { unlocked });
}

fn adventure_menu(
    mut contexts: EguiContexts,
    mut e_level: EventWriter<level::LevelEvent>,
    mut menu: ResMut<NextState<info::MenuStates>>,
    info: Res<AdventureInfo>,
    mut is_level: Local<bool>,
    mut stage: Local<u32>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        if !*is_level {
            ui.label(super::title_text("Stage..."));

            let rows = info.unlocked.len().div_ceil(5);
            let mut iter = info.unlocked.keys();
            TableBuilder::new(ui)
                .columns(Column::auto(), 5)
                .body(|body| {
                    body.rows(40.0 * UI_ZOOM_FACTOR, rows, |mut row| {
                        for _ in 0..5 {
                            if let Some(value) = iter.next() {
                                row.col(|ui| {
                                    let button = egui::Button::new(super::medium_text(format!(
                                        "S{:<2}",
                                        value
                                    )))
                                    .wrap_mode(egui::TextWrapMode::Extend);
                                    if ui.add(button).clicked() {
                                        *stage = *value;
                                        *is_level = true;
                                    }
                                });
                            } else {
                                break;
                            }
                        }
                    });
                });

            ui.separator();
            if ui.button(super::medium_text("Back to Main Menu")).clicked() {
                menu.set(info::MenuStates::Main);
            }
        } else {
            ui.label(super::title_text("Level..."));
            if let Some(unlocked) = info.unlocked.get(&stage) {
                let rows = unlocked.len().div_ceil(5);
                let mut iter = unlocked.iter();
                TableBuilder::new(ui)
                    .columns(Column::auto(), 5)
                    .body(|body| {
                        body.rows(40.0 * UI_ZOOM_FACTOR, rows, |mut row| {
                            for _ in 0..5 {
                                if let Some(value) = iter.next() {
                                    row.col(|ui| {
                                        let button = egui::Button::new(super::medium_text(
                                            format!("{:2}-{:<3}", *stage, value),
                                        ))
                                        .wrap_mode(egui::TextWrapMode::Extend);
                                        if ui.add(button).clicked() {
                                            e_level.send(level::LevelEvent {
                                                index: level::LevelIndex {
                                                    stage: *stage,
                                                    level: *value,
                                                },
                                            });
                                            *is_level = false;
                                        }
                                    });
                                } else {
                                    break;
                                }
                            }
                        });
                    });
            }

            ui.separator();
            if ui.button(super::medium_text("Back to Stages")).clicked() {
                *is_level = false;
            }
        }
    });
}
