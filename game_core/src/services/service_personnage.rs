use crate::depots::depot_personnages::DepotPersonnages;

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

        perso.pv -= quantite;

        if perso.pv < 0 {
            perso.pv = 0;
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

        perso.pv += quantite;

        if perso.pv > perso.pv_max {
            perso.pv = perso.pv_max;
        }

        Ok(())
    }

    pub fn est_mort(depot: &mut DepotPersonnages, id: u32) -> Result<bool, String> {
        let Some(perso) = depot.recuperer_personnage_mut(id) else {
            return Err(format!("Personnage inexistante : {}", id));
        };

        Ok(perso.pv > 0)
    }
}
