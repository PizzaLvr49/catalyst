use std::path::PathBuf;

use bevy::{platform::collections::HashMap, prelude::*};
use leafwing_manifest::{
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub value: i32,
    pub weight: f32,
    pub max_stack: u8,
    pub sprite: Handle<Image>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RawItem {
    pub name: String,
    pub description: String,
    pub value: i32,
    pub weight: f32,
    pub max_stack: u8,
    pub sprite: PathBuf,
}

#[derive(Debug, Resource, PartialEq)]
pub struct ItemManifest {
    pub items: HashMap<Id<Item>, Item>,
}

#[derive(Debug, Asset, TypePath, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct RawItemManifest {
    pub items: Vec<RawItem>,
}

impl RawItemManifest {
    pub fn merge_from(&mut self, other: RawItemManifest) {
        self.items.extend(other.items);
    }
}

impl Manifest for ItemManifest {
    type Item = Item;
    type RawItem = RawItem;
    type RawManifest = RawItemManifest;
    type ConversionError = std::convert::Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn get(&self, id: Id<Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let asset_server = world.resource::<AssetServer>();

        let items: HashMap<_, _> = raw_manifest
            .items
            .into_iter()
            .map(|raw_item| {
                let sprite = asset_server.load(raw_item.sprite);
                let item = Item {
                    name: raw_item.name,
                    description: raw_item.description,
                    value: raw_item.value,
                    weight: raw_item.weight,
                    max_stack: raw_item.max_stack,
                    sprite,
                };
                (Id::from_name(&item.name), item)
            })
            .collect();

        Ok(ItemManifest { items })
    }
}
