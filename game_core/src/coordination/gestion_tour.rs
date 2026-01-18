use crate::{
    depots::depot_personnages::DepotPersonnages,
    metier::{equipe::Equipe, event::Event},
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

    pub fn ajouter_events(&mut self, events: Vec<Event>) {
        for e in events {
            self.events.push(e);
        }
    }

    pub fn vider_events(&mut self) {
        self.events.clear();
    }

    pub fn est_termine(&self) -> bool {
        self.events.is_empty()
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
                id_cible,
                quantite,
                id_source,
            } => {
                ServicePersonnage::appliquer_degats(depot, id_cible, quantite)?;

                if ServicePersonnage::est_mort(depot, id_cible)? {
                    self.events.push(Event::Mort { id: id_cible });
                }
            }
            Event::Soigner {
                id_cible,
                quantite,
                id_source,
            } => {
                ServicePersonnage::appliquer_soins(depot, id_cible, quantite)?;
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

    /// retourne vrai si un event a été traité, faux sinon
    pub fn resourdre_un_event(
        &mut self,
        depot: &mut DepotPersonnages,
        equipe_joueur: &mut Equipe,
        equipe_ennemie: &mut Equipe,
    ) -> Result<bool, String> {
        if !self.events.is_empty() {
            let event = self.events.remove(0);

            self.appliquer_event(event, depot, equipe_joueur, equipe_ennemie)?;

            return Ok(true);
        }

        Ok(false)
    }
}
