use dioxus::prelude::*;
use game_core::metier::etat_combat::EtatCombat;

#[component]
pub fn EcranFinCombat(etat: EtatCombat) -> Element {
    let message = match etat {
        EtatCombat::Victoire => "VICTORY !",
        EtatCombat::Defaite => "Defeat...",
        _ => return rsx! {},
    };

    rsx! {
        div {
            h1 { "{message}" }
        }
    }
}
