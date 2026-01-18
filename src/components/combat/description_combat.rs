use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;

#[component]
pub fn DescriptionCombat() -> Element {
    // récupérer perso
    let signal_gestion_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_gestion_jeu.read();

    let equipe_active = if jeu.est_tour_joueur() {
        "Player's team"
    } else {
        "Enemy team"
    };

    rsx! {
        div { class: "equipe",
            h1 { "Current team : {equipe_active}" }
        }
    }
}
