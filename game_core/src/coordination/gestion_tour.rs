use crate::{
    depots::depot_personnages::DepotPersonnages,
    metier::{equipe::Equipe, etat_combat::EtatCombat, event::Event},
    services::{service_equipe::ServiceEquipe, service_personnage::ServicePersonnage},
};

pub struct GestionTour {
    events: Vec<Event>,
}

impl GestionTour {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn ajouter_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn appliquer_event(
        &mut self,
        event: Event,
        depot: &mut DepotPersonnages,
        equipe_joueur: &mut Equipe,
        equipe_ennemie: &mut Equipe,
    ) -> Result<(), String> {
        match event {
            Event::FaireDegats {
                cible_id,
                quantite,
                source_id,
            } => {
                ServicePersonnage::appliquer_degats(depot, cible_id, quantite)?;

                if ServicePersonnage::est_mort(depot, cible_id)? {
                    self.events.push(Event::Mort { id: cible_id });
                }
            }
            Event::Soigner {
                cible_id,
                quantite,
                source_id,
            } => {
                ServicePersonnage::appliquer_soins(depot, cible_id, quantite)?;
            }
            Event::Mort { id } => {
                if ServiceEquipe::contient_membre(equipe_joueur, id) {
                    equipe_joueur.retirer_membre(id);
                }
                if ServiceEquipe::contient_membre(equipe_ennemie, id) {
                    equipe_ennemie.retirer_membre(id);
                }
            }
        }

        Ok(())
    }

    pub fn resoudre_tour(
        &mut self,
        depot: &mut DepotPersonnages,
        equipe_joueur: &mut Equipe,
        equipe_ennemie: &mut Equipe,
    ) -> Result<EtatCombat, String> {
        while !self.events.is_empty() {
            let event = self.events.remove(0);

            self.appliquer_event(event, depot, equipe_joueur, equipe_ennemie)?;

            if ServiceEquipe::est_vaincue(equipe_joueur, depot) {
                return Ok(EtatCombat::Defaite);
            }
            if ServiceEquipe::est_vaincue(equipe_ennemie, depot) {
                return Ok(EtatCombat::Victoire);
            }
        }

        Ok(EtatCombat::EnCours)
    }
}
