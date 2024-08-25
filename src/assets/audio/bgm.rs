use crate::prelude::*;
use serde::Deserialize;

pub struct LayoutBgm {
    pub normal: Handle<AudioSource>,
    pub exciting: Handle<AudioSource>,
    pub begin: f64,
}
#[derive(Resource)]
pub struct AudioBgm {
    map: BTreeMap<String, Handle<AudioSource>>,
    layout: BTreeMap<level::LayoutKind, LayoutBgm>,
}
impl AudioBgm {
    pub fn get_name(&self, name: &str) -> Option<&Handle<AudioSource>> {
        self.map.get(name)
    }

    pub fn get_layout(&self, layout: level::LayoutKind) -> Option<&LayoutBgm> {
        self.layout.get(&layout)
    }
}

#[derive(Deserialize)]
struct LayoutBind {
    normal: String,
    exciting: String,
    begin: f64,
}
#[derive(Deserialize)]
struct BgmBind {
    bind: BTreeMap<String, String>,
    layout: BTreeMap<level::LayoutKind, LayoutBind>,
}

pub(super) fn load_bgm(mut commands: Commands, server: Res<AssetServer>) {
    let str =
        std::fs::read_to_string("assets/audio/bgm/bind.ron").expect("cannot load bgm bindings");
    match ron::from_str::<BgmBind>(&str) {
        Ok(bind) => {
            let BgmBind { bind, layout } = bind;
            let map = bind
                .into_iter()
                .map(|(name, src)| (name, server.load::<AudioSource>(src)))
                .collect::<BTreeMap<_, _>>();
            let layout = layout
                .into_iter()
                .filter_map(
                    |(
                        layout,
                        LayoutBind {
                            normal,
                            exciting,
                            begin,
                        },
                    )| {
                        map.get(&normal).cloned().and_then(|normal| {
                            map.get(&exciting).cloned().map(|exciting| {
                                (
                                    layout,
                                    LayoutBgm {
                                        normal,
                                        exciting,
                                        begin,
                                    },
                                )
                            })
                        })
                    },
                )
                .collect();
            commands.insert_resource(AudioBgm { map, layout });
        }
        Err(err) => {
            error!("Unable to parse assets/audio/bgm/bind.ron: {}", err);
        }
    }
}
