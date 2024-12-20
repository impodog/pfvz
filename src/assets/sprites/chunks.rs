use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteChunks {
    pub white: Handle<Image>,
    pub transparent: Handle<Image>,
    pub pvfz: Handle<Image>,
    pub background: Handle<Image>,
    pub slot: Handle<Image>,
    pub highlight: Handle<Image>,
    pub final_wave: Handle<Image>,
    pub alert: Handle<Image>,
    pub zayb: Handle<Image>,
    pub cooldown: Handle<Image>,
    pub you_win: Handle<Image>,
    pub shovel: Handle<Image>,
    pub start: Handle<Image>,
    pub next: Handle<Image>,
    pub note1: Handle<Image>,
    pub note2: Handle<Image>,
    pub note3: Handle<Image>,
    pub note4: Handle<Image>,
    pub note5: Handle<Image>,
    pub dave: Handle<Image>,
    pub shadow: Arc<sprite::FrameArr>,
    pub cross: Handle<Image>,
    pub conveyor_mid: Handle<Image>,
    pub conveyor_left: Handle<Image>,
    pub conveyor_right: Handle<Image>,
    pub pdog: Handle<Image>,
}

pub(super) fn init_chunks(mut commands: Commands, server: Res<AssetServer>) {
    let chunks = SpriteChunks {
        white: server.load("sprites/chunks/white.png"),
        transparent: server.load("sprites/chunks/transparent.png"),
        pvfz: server.load("sprites/chunks/pfvz.png"),
        background: server.load("sprites/chunks/background.png"),
        slot: server.load("sprites/chunks/slot.png"),
        highlight: server.load("sprites/chunks/highlight.png"),
        final_wave: server.load("sprites/chunks/final.png"),
        alert: server.load("sprites/chunks/alert.png"),
        zayb: server.load("sprites/chunks/zayb.png"),
        cooldown: server.load("sprites/chunks/cooldown.png"),
        you_win: server.load("sprites/chunks/you_win.png"),
        shovel: server.load("sprites/chunks/shovel.png"),
        start: server.load("sprites/chunks/start.png"),
        next: server.load("sprites/chunks/next.png"),
        note1: server.load("sprites/chunks/note1.png"),
        note2: server.load("sprites/chunks/note2.png"),
        note3: server.load("sprites/chunks/note3.png"),
        note4: server.load("sprites/chunks/note4.png"),
        note5: server.load("sprites/chunks/note5.png"),
        dave: server.load("sprites/chunks/dave.png"),
        shadow: super::load_animation(&server, "sprites/chunks/shadow", Duration::from_secs(100)),
        cross: server.load("sprites/chunks/cross.png"),
        conveyor_mid: server.load("sprites/chunks/conveyor_mid.png"),
        conveyor_left: server.load("sprites/chunks/conveyor_left.png"),
        conveyor_right: server.load("sprites/chunks/conveyor_right.png"),
        pdog: server.load("sprites/chunks/pdog.png"),
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT)),
            color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 0.02)),
            ..Default::default()
        },
        texture: chunks.background.clone(),
        ..Default::default()
    });
    commands.insert_resource(chunks);
}
