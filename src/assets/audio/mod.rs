mod items;
mod plants;
mod zombies;

pub use items::*;
pub use plants::*;
pub use zombies::*;

use crate::prelude::*;

pub(super) struct AssetsAudioPlugin;

impl Plugin for AssetsAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                items::load_items,
                zombies::load_zombies,
                plants::load_plants,
            ),
        );
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct AudioList(pub Arc<Vec<Handle<AudioSource>>>);
impl AudioList {
    fn load_result(server: &Res<AssetServer>, path: &str) -> std::io::Result<Self> {
        let mut list = Vec::new();
        for entry in std::fs::read_dir(format!("assets/{}", path))? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "ogg") {
                list.push(server.load(std::fs::canonicalize(path)?));
            }
        }
        Ok(Self(Arc::new(list)))
    }

    /// Loads a source dir of audio, or report an error when no audio file available
    pub fn load(server: &Res<AssetServer>, path: &str) -> Self {
        match Self::load_result(server, path) {
            Ok(result) => result,
            Err(err) => {
                error!("Failed to load audio source at {}: {}", path, err);
                Self(Arc::new(vec![Default::default()]))
            }
        }
    }

    /// This will panic if no audio is loaded!
    pub fn random(&self) -> Handle<AudioSource> {
        let index = rand::thread_rng().gen_range(0..self.len());
        self.get(index).unwrap().clone()
    }
}
