use crate::{depots::depot_personnages::DepotPersonnages, metier::equipe::Equipe};

#[derive(Clone, Debug)]
pub struct ServiceEquipe;

impl ServiceEquipe {
    pub fn est_vaincue(equipe: &Equipe, depot: &DepotPersonnages) -> bool {
        let membres_ids = equipe.recuperer_liste_membres();

        if membres_ids.is_empty() {
            return true;
        }

        membres_ids
            .iter()
            .all(|id| match depot.recuperer_personnage(*id) {
                Some(p) => p.pv <= 0,
                None => true,
            })
    }

    pub fn contient_membre(equipe: &Equipe, id: u32) -> bool {
        equipe.recuperer_liste_membres().contains(&id)
    }
}
