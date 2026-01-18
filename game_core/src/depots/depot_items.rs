use std::collections::HashMap;

use crate::metier::{item::Item, personnage::Stats};

pub struct DepotItems {
    items: HashMap<u32, Item>,
    prochain_id: u32,
}

impl DepotItems {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            prochain_id: 0,
        }
    }

    pub fn creer_item(&mut self, nom: String, stats: Stats) -> u32 {
        let item = Item::new(nom, stats);
        let id_item = self.prochain_id;
        self.items.insert(id_item, item);
        self.prochain_id += 1;

        id_item
    }

    pub fn recuperer_item(&self, id: u32) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn recuperer_item_mut(&mut self, id: u32) -> Option<&mut Item> {
        self.items.get_mut(&id)
    }
}
