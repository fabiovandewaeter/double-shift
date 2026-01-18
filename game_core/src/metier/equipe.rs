use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Equipe {
    nom: String,
    membres: HashMap<i8, u32>,
    argent: u32,
}

impl Equipe {
    pub const NOMBRE_MAX_MEMBRES: i8 = 4;

    pub fn new(nom: String) -> Self {
        Self {
            nom,
            membres: HashMap::new(),
            argent: 0,
        }
    }

    pub fn nom(&self) -> String {
        self.nom.clone()
    }

    pub fn argent(&self) -> u32 {
        self.argent
    }

    /// à partir de la position dans l'équipe
    pub fn recuperer_id_membre(&self, position: i8) -> Option<u32> {
        if position < 0 || position >= Self::NOMBRE_MAX_MEMBRES {
            return None;
        }

        self.membres.get(&position).copied()
    }

    pub fn recuperer_liste_id_membres(&self) -> Vec<u32> {
        self.membres.values().copied().collect()
    }

    /// à partir de l'id du membre
    pub fn retirer_membre(&mut self, id: u32) {
        self.membres.retain(|_, v| *v != id);
    }

    /// retourne l'id du membre remplacé (ou None) ou une erreur
    pub fn ajouter_membre_equipe(
        &mut self,
        id_perso: u32,
        position: i8,
    ) -> Result<Option<u32>, String> {
        if position < 0 || position >= Self::NOMBRE_MAX_MEMBRES {
            return Err("Position invalide".to_string());
        }

        let ancien_membre = self.membres.get(&position).copied();
        self.membres.insert(position, id_perso);

        Ok(ancien_membre)
    }
}
