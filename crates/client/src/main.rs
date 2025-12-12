use std::error::Error;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
    window::{PresentMode, WindowMode},
};
use bevy_asset_loader::prelude::*;
use leafwing_manifest::manifest::Manifest;

use ron::extensions::Extensions;
use ron::options::Options;

mod manifest_definition;
use manifest_definition::{ItemManifest, RawItemManifest};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .register_asset_loader(ItemAssetLoader)
        .init_asset::<RawItemManifest>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<ItemAssets>(),
        )
        .add_systems(OnEnter(GameState::Running), process_and_print_items)
        .run()
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Running,
}

#[derive(AssetCollection, Resource)]
struct ItemAssets {
    #[asset(path = "items", collection(typed))]
    manifests: Vec<Handle<RawItemManifest>>,
}

fn process_and_print_items(
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    raw_assets: Res<Assets<RawItemManifest>>,
) {
    let mut merged = RawItemManifest::default();
    for handle in &item_assets.manifests {
        if let Some(raw) = raw_assets.get(handle) {
            merged.merge_from(raw.clone());
        }
    }

    commands.queue(move |world: &mut World| -> Result {
        let item_manifest = ItemManifest::from_raw_manifest(merged, world)?;

        for (id, item) in &item_manifest.items {
            println!(
                "ID: {:?}, Name: {}, Description: {}, Value: {}, Weight: {}, Max Stack: {}",
                id, item.name, item.description, item.value, item.weight, item.max_stack
            );
        }

        world.insert_resource(item_manifest);

        Ok(())
    });
}

struct ItemAssetLoader;

impl AssetLoader for ItemAssetLoader {
    type Asset = RawItemManifest;
    type Settings = ();
    type Error = Box<dyn Error + Send + Sync + 'static>;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Construct the same RON options you used in your example
            let options = Options::default().with_default_extension(Extensions::all());

            let asset: RawItemManifest = options.from_bytes(&bytes)?;

            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["item.ron"]
    }
}
