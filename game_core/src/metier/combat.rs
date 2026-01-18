use crate::metier::etat_combat::EtatCombat;

pub struct Combat {
    recompense: u32,
    est_tour_joueur: bool,
    etat_combat: EtatCombat,
}

impl Combat {
    pub fn new(recompense: u32) -> Self {
        Self {
            recompense,
            est_tour_joueur: true,
            etat_combat: EtatCombat::EnCours,
        }
    }

    pub fn recompense(&self) -> u32 {
        self.recompense
    }
    pub fn est_tour_joueur(&self) -> bool {
        self.est_tour_joueur
    }
    pub fn etat_combat(&self) -> EtatCombat {
        self.etat_combat.clone()
    }

    pub fn definir_etat(&mut self, nouvel_etat: EtatCombat) {
        self.etat_combat = nouvel_etat;
    }
    pub fn passer_tour_suivant(&mut self) {
        self.est_tour_joueur = !self.est_tour_joueur;
    }
}
