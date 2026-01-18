use crate::{
    depots::depot_personnages::DepotPersonnages,
    metier::{combat::Combat, equipe::Equipe, etat_combat::EtatCombat},
    services::service_equipe::ServiceEquipe,
};

pub struct ServiceCombat;

impl ServiceCombat {
    pub fn creer_prochain_combat(niveau_actuel: u32) -> Combat {
        Combat::new(niveau_actuel.pow(2))
    }

    pub fn passer_tour_suivant(combat: &mut Combat) {
        combat.passer_tour_suivant();
    }

    /// change l'état du combat et retourne vrai si le combat est fini
    pub fn verifier_et_mettre_a_jour_etat(
        combat: &mut Combat,
        equipe_joueur: &Equipe,
        equipe_ennemie: &Equipe,
        depot: &DepotPersonnages,
    ) -> bool {
        if ServiceEquipe::est_vaincue(equipe_joueur, depot) {
            combat.definir_etat(EtatCombat::Defaite);
            return true;
        }
        if ServiceEquipe::est_vaincue(equipe_ennemie, depot) {
            combat.definir_etat(EtatCombat::Victoire);
            return true;
        }

        false
    }
}
