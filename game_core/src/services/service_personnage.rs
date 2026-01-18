use crate::{depots::depot_personnages::DepotPersonnages, metier::event::Event};

#[derive(Clone, Debug)]
pub struct ServicePersonnage;

impl ServicePersonnage {
    pub fn appliquer_degats(
        depot: &mut DepotPersonnages,
        cible_id: u32,
        quantite: i32,
    ) -> Result<(), String> {
        let Some(perso) = depot.recuperer_personnage_mut(cible_id) else {
            return Err(format!("Cible inexistante : {}", cible_id));
        };

        perso.stats.pv -= quantite;

        if perso.stats.pv < 0 {
            perso.stats.pv = 0;
        }

        Ok(())
    }

    pub fn appliquer_soins(
        depot: &mut DepotPersonnages,
        cible_id: u32,
        quantite: i32,
    ) -> Result<(), String> {
        let Some(perso) = depot.recuperer_personnage_mut(cible_id) else {
            return Err(format!("Cible inexistante : {}", cible_id));
        };

        perso.stats.pv += quantite;

        if perso.stats.pv > perso.stats.pv_max {
            perso.stats.pv = perso.stats.pv_max;
        }

        Ok(())
    }

    pub fn est_mort(depot: &mut DepotPersonnages, id: u32) -> Result<bool, String> {
        let Some(perso) = depot.recuperer_personnage_mut(id) else {
            return Err(format!("Personnage inexistante : {}", id));
        };

        Ok(perso.stats.pv <= 0)
    }

    pub fn jouer_tour(
        id_attaquant: u32,
        id_cible: u32,
        depot: &DepotPersonnages,
    ) -> Result<Event, String> {
        let Some(attaquant) = depot.recuperer_personnage(id_attaquant) else {
            return Err(format!("Personnage inexistante : {}", id_attaquant));
        };

        let degats = attaquant.stats.attaque;

        Ok(Event::FaireDegats {
            id_cible,
            quantite: degats as i32,
            id_source: Some(id_attaquant),
        })
    }
}
