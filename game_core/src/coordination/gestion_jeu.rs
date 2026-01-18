use crate::{
    coordination::gestion_tour::GestionTour,
    depots::depot_personnages::DepotPersonnages,
    metier::{
        equipe::Equipe,
        etat_combat::EtatCombat,
        personnage::{Personnage, StatsPersonnage},
    },
    services::service_equipe::ServiceEquipe,
};

pub struct GestionJeu {
    depot_personnages: DepotPersonnages,
    equipe_joueur: Equipe,
    equipe_ennemie: Equipe,
    gestion_tour: GestionTour,
    est_tour_joueur: bool,
    etat_combat: EtatCombat,
}

impl GestionJeu {
    pub fn new() -> Self {
        Self {
            depot_personnages: DepotPersonnages::new(),
            equipe_joueur: Equipe::new("Player's team".to_owned()),
            equipe_ennemie: Equipe::new("Enemy team".to_owned()),
            gestion_tour: GestionTour::new(),
            est_tour_joueur: true,
            etat_combat: EtatCombat::EnCours,
        }
    }

    pub fn equipe(&self, est_equipe_joueur: bool) -> &Equipe {
        if est_equipe_joueur {
            &self.equipe_joueur
        } else {
            &self.equipe_ennemie
        }
    }

    pub fn depot_personnages(&self) -> &DepotPersonnages {
        &self.depot_personnages
    }

    pub fn creer_personnage(&mut self, nom: String, stats: StatsPersonnage) -> u32 {
        self.depot_personnages.creer_personnage(nom, stats)
    }

    // pub fn recuperer_personnage(&self, id: u32) -> Option<&Personnage> {
    //     self.depot_personnages.recuperer_personnage(id)
    // }

    pub fn recuperer_personnage_mut(&mut self, id: u32) -> Option<&mut Personnage> {
        self.depot_personnages.recuperer_personnage_mut(id)
    }

    // pub fn recuperer_id_membre_equipe(&self, position: i8, est_equipe_joueur: bool) -> Option<u32> {
    //     if est_equipe_joueur {
    //         self.equipe_joueur.recuperer_id_membre(position)
    //     } else {
    //         self.equipe_ennemie.recuperer_id_membre(position)
    //     }
    // }

    pub fn ajouter_membre_equipe(
        &mut self,
        id: u32,
        position: i8,
        est_equipe_joueur: bool,
    ) -> Result<Option<u32>, String> {
        if est_equipe_joueur {
            self.equipe_joueur.ajouter_membre_equipe(id, position)
        } else {
            self.equipe_ennemie.ajouter_membre_equipe(id, position)
        }
    }

    // --- gestion tour ---
    pub fn est_tour_joueur(&self) -> bool {
        self.est_tour_joueur
    }

    pub fn etat_combat(&self) -> &EtatCombat {
        &self.etat_combat
    }

    pub fn demarrer_tour(&mut self) {
        let (equipe_active, equipe_passive) = if self.est_tour_joueur {
            (&self.equipe_joueur, &self.equipe_ennemie)
        } else {
            (&self.equipe_ennemie, &self.equipe_joueur)
        };

        let nouveaux_events =
            ServiceEquipe::jouer_tour(equipe_active, equipe_passive, &self.depot_personnages);

        self.gestion_tour.ajouter_events(nouveaux_events);
    }

    /// retourne vrai si le tour et fini
    pub fn executer_un_pas_tour(&mut self) -> Result<bool, String> {
        self.gestion_tour.resourdre_un_event(
            &mut self.depot_personnages,
            &mut self.equipe_joueur,
            &mut self.equipe_ennemie,
        )?;

        if ServiceEquipe::est_vaincue(&self.equipe_joueur, &self.depot_personnages) {
            self.etat_combat = EtatCombat::Defaite;
            return Ok(false);
        }
        if ServiceEquipe::est_vaincue(&self.equipe_ennemie, &self.depot_personnages) {
            self.etat_combat = EtatCombat::Victoire;
            return Ok(false);
        }

        if self.gestion_tour.est_termine() {
            self.est_tour_joueur = !self.est_tour_joueur;
            return Ok(false);
        }

        Ok(true)
    }
}
