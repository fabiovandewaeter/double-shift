use dioxus::prelude::*;
use game_core::{coordination::gestion_jeu::GestionJeu, metier::equipe::Equipe};

#[component]
pub fn DescriptionEquipe(est_equipe_joueur: bool) -> Element {
    let signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();

    let equipe = jeu.equipe(est_equipe_joueur);
    let nom_equipe = equipe.nom();
    let depot = jeu.depot_personnages();

    rsx! {
        div { class: "equipe",
            h2 { "{nom_equipe}" }
            div { class: "text-lg",
                ul {
                    li { "Coins: {equipe.argent()}" }
                }
            }
            for i in 0..Equipe::NOMBRE_MAX_MEMBRES {

                match equipe.recuperer_id_membre(i).and_then(|id| depot.recuperer_personnage(id)) {
                    Some(perso) => rsx! {
                        div { class: "text-lg",
                            ul {
                                li { "Nom: {perso.nom}" }
                                li { "HP: {perso.pv}" }
                                li { "Max HP: {perso.pv_max}" }
                            }
                            span { "---" }
                        }
                    },
                    None => rsx! {},
                }
            }
        }
    }
}
