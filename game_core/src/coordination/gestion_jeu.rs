use crate::{
    coordination::gestion_tour::GestionTour,
    depots::depot_personnages::DepotPersonnages,
    metier::{
        combat::Combat,
        equipe::Equipe,
        etat_combat::EtatCombat,
        personnage::{Personnage, StatsPersonnage},
    },
    services::{service_combat::ServiceCombat, service_equipe::ServiceEquipe},
};

pub struct GestionJeu {
    depot_personnages: DepotPersonnages,
    equipe_joueur: Equipe,
    equipe_ennemie: Equipe,
    gestion_tour: GestionTour,
    combat_actuel: Combat,
    niveau_actuel: u32,
}

impl GestionJeu {
    pub fn new() -> Self {
        let niveau_actuel = 0;
        let combat_actuel = ServiceCombat::creer_prochain_combat(niveau_actuel);

        let mut jeu = Self {
            depot_personnages: DepotPersonnages::new(),
            equipe_joueur: Equipe::new("Player's team".to_owned()),
            equipe_ennemie: Equipe::new("Enemy team".to_owned()),
            gestion_tour: GestionTour::new(),
            combat_actuel,
            niveau_actuel,
        };

        jeu.lancer_prochain_combat();

        // TODO à supprimer
        let id_joueur = jeu.creer_personnage(
            "Fabebou".to_string(),
            StatsPersonnage {
                pv_max: 100,
                attaque: 10,
            },
        );
        let id_joueur2 = jeu.creer_personnage(
            "Abebou".to_string(),
            StatsPersonnage {
                pv_max: 100,
                attaque: 10,
            },
        );

        jeu.ajouter_membre_equipe(id_joueur, 0, true);
        jeu.ajouter_membre_equipe(id_joueur2, 1, true);

        jeu
    }

    pub fn reset_jeu(&mut self) {
        *self = Self::new();
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
    pub fn recuperer_personnage_mut(&mut self, id: u32) -> Option<&mut Personnage> {
        self.depot_personnages.recuperer_personnage_mut(id)
    }

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
    pub fn combat_actuel(&self) -> &Combat {
        &self.combat_actuel
    }

    pub fn demarrer_tour(&mut self) -> Result<(), String> {
        let (equipe_active, equipe_passive) = if self.combat_actuel.est_tour_joueur() {
            (&self.equipe_joueur, &self.equipe_ennemie)
        } else {
            (&self.equipe_ennemie, &self.equipe_joueur)
        };

        let nouveaux_events =
            ServiceEquipe::jouer_tour(equipe_active, equipe_passive, &self.depot_personnages);

        self.gestion_tour.ajouter_events(nouveaux_events);

        Ok(())
    }

    /// retourne vrai si le tour et fini
    pub fn executer_un_pas_tour(&mut self) -> Result<bool, String> {
        self.gestion_tour.resourdre_un_event(
            &mut self.depot_personnages,
            &mut self.equipe_joueur,
            &mut self.equipe_ennemie,
        )?;

        let combat_fini = ServiceCombat::verifier_et_mettre_a_jour_etat(
            &mut self.combat_actuel,
            &self.equipe_joueur,
            &self.equipe_ennemie,
            &self.depot_personnages,
        );

        if combat_fini {
            return Ok(false);
        }

        if self.gestion_tour.est_termine() {
            ServiceCombat::passer_tour_suivant(&mut self.combat_actuel);
            return Ok(false);
        }

        Ok(true)
    }

    // --- gestion combat ---
    pub fn lancer_prochain_combat(&mut self) {
        self.niveau_actuel += 1;

        let prochain_combat = ServiceCombat::creer_prochain_combat(self.niveau_actuel);
        self.combat_actuel = prochain_combat;

        self.equipe_ennemie = Equipe::new("Enemy team".to_string());
        let id_ennemi = self.depot_personnages.creer_personnage(
            format!("Goblin lvl {}", self.niveau_actuel),
            StatsPersonnage {
                pv_max: 10 * self.niveau_actuel as i32,
                attaque: 10 * self.niveau_actuel,
            },
        );
        self.equipe_ennemie
            .ajouter_membre_equipe(id_ennemi, 0)
            .unwrap();

        self.gestion_tour = GestionTour::new();
    }

    pub fn terminer_combat_et_recolter(&mut self) -> Result<(), String> {
        if self.combat_actuel.etat_combat() != EtatCombat::Victoire {
            return Err("Impossible de récolter : le combat n'est pas gagné".to_string());
        }

        self.equipe_joueur
            .ajouter_argent(self.combat_actuel.recompense());

        Ok(())
    }
}
