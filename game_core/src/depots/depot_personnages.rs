use std::collections::HashMap;

use crate::metier::personnage::{Personnage, Stats};

#[derive(Clone)]
pub struct DepotPersonnages {
    personnages: HashMap<u32, Personnage>,
    prochain_id: u32,
}

impl DepotPersonnages {
    pub fn new() -> Self {
        Self {
            personnages: HashMap::new(),
            prochain_id: 0,
        }
    }

    pub fn creer_personnage(&mut self, nom: String, stats: Stats) -> u32 {
        let perso = Personnage::new(nom, stats);
        let id_perso = self.prochain_id;
        self.personnages.insert(id_perso, perso);
        self.prochain_id += 1;

        id_perso
    }

    pub fn recuperer_personnage(&self, id: u32) -> Option<&Personnage> {
        self.personnages.get(&id)
    }
    pub fn recuperer_personnage_mut(&mut self, id: u32) -> Option<&mut Personnage> {
        self.personnages.get_mut(&id)
    }
}
