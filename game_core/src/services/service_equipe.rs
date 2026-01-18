use crate::{
    depots::depot_personnages::DepotPersonnages,
    metier::{equipe::Equipe, event::Event},
    services::service_personnage::ServicePersonnage,
};

#[derive(Clone, Debug)]
pub struct ServiceEquipe;

impl ServiceEquipe {
    pub fn est_vaincue(equipe: &Equipe, depot: &DepotPersonnages) -> bool {
        let membres_ids = equipe.recuperer_liste_id_membres();

        if membres_ids.is_empty() {
            return true;
        }

        membres_ids
            .iter()
            .all(|id| match depot.recuperer_personnage(*id) {
                Some(p) => p.stats.pv <= 0,
                None => true,
            })
    }

    pub fn contient_membre(equipe: &Equipe, id: u32) -> bool {
        equipe.recuperer_liste_id_membres().contains(&id)
    }

    /// cible l'adversaire le plus à gauche par défaut
    pub fn trouver_cible(defenseurs: &Equipe) -> Option<u32> {
        for pos in 0..Equipe::NOMBRE_MAX_MEMBRES {
            if let Some(id) = defenseurs.recuperer_id_membre(pos) {
                return Some(id);
            }
        }
        None
    }

    pub fn jouer_tour(
        attaquants: &Equipe,
        defenseurs: &Equipe,
        depot: &DepotPersonnages,
    ) -> Vec<Event> {
        let mut events = Vec::new();

        for pos in 0..Equipe::NOMBRE_MAX_MEMBRES {
            if let Some(id_attaquant) = attaquants.recuperer_id_membre(pos) {
                if let Some(id_cible) = Self::trouver_cible(defenseurs) {
                    if let Ok(event) = ServicePersonnage::jouer_tour(id_attaquant, id_cible, depot)
                    {
                        events.push(event);
                    }
                }
            }
        }

        events
    }
}
