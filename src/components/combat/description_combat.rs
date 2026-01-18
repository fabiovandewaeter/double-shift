use dioxus::prelude::*;
use game_core::{coordination::gestion_jeu::GestionJeu, metier::etat_partie::EtatPartie};

#[component]
pub fn DescriptionCombat() -> Element {
    let signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();

    let EtatPartie::Combat(combat) = jeu.etat_partie() else {
        return rsx! {
            div {
                h1 { "No battle started" }
            }
        };
    };

    let equipe = jeu.equipe(combat.est_tour_joueur());

    rsx! {
        div { class: "equipe",
            h1 { "Current team : {equipe.nom()}" }
        }
    }
}
