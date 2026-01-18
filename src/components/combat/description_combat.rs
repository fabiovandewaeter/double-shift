use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;

#[component]
pub fn DescriptionCombat() -> Element {
    let signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();
    let combat = jeu.combat_actuel();

    let equipe = jeu.equipe(combat.est_tour_joueur());

    rsx! {
        div { class: "equipe",
            h1 { "Current team : {equipe.nom()}" }
        }
    }
}
