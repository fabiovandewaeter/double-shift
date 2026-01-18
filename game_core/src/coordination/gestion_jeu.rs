use crate::{
    coordination::gestion_tour::GestionTour,
    depots::depot_personnages::DepotPersonnages,
    metier::{equipe::Equipe, etat_combat::EtatCombat, personnage::Personnage},
};

pub struct GestionJeu {
    depot_personnages: DepotPersonnages,
    equipe_joueur: Equipe,
    equipe_ennemie: Equipe,
    gestion_tour: GestionTour,
}

impl GestionJeu {
    pub fn new() -> Self {
        Self {
            depot_personnages: DepotPersonnages::new(),
            equipe_joueur: Equipe::new(),
            equipe_ennemie: Equipe::new(),
            gestion_tour: GestionTour::new(),
        }
    }

    pub fn creer_personnage(&mut self, nom: String, pv_max: i32) -> u32 {
        self.depot_personnages.creer_personnage(nom, pv_max)
    }

    pub fn recuperer_personnage(&self, id: u32) -> Option<&Personnage> {
        self.depot_personnages.recuperer_personnage(id)
    }

    pub fn recuperer_personnage_mut(&mut self, id: u32) -> Option<&mut Personnage> {
        self.depot_personnages.recuperer_personnage_mut(id)
    }

    pub fn recuperer_id_membre_equipe(&self, position: i8, est_equipe_joueur: bool) -> Option<u32> {
        if est_equipe_joueur {
            self.equipe_joueur.recuperer_id_membre(position)
        } else {
            self.equipe_ennemie.recuperer_id_membre(position)
        }
    }

    pub fn ajouter_membre_equipe_joueur(
        &mut self,
        id: u32,
        position: i8,
    ) -> Result<Option<u32>, String> {
        self.equipe_joueur.ajouter_membre_equipe(id, position)
    }

    pub fn ajouter_membre_equipe_ennemie(
        &mut self,
        id: u32,
        position: i8,
    ) -> Result<Option<u32>, String> {
        self.equipe_ennemie.ajouter_membre_equipe(id, position)
    }

    pub fn prochain_tour(&mut self) -> Result<EtatCombat, String> {
        self.gestion_tour.resoudre_tour(
            &mut self.depot_personnages,
            &mut self.equipe_joueur,
            &mut self.equipe_ennemie,
        )
    }
}
