use dioxus::prelude::*;
use game_core::{coordination::gestion_jeu::GestionJeu, metier::equipe::Equipe};

#[component]
pub fn DescriptionEquipe(est_equipe_joueur: bool) -> Element {
    // récupérer perso
    let signal_gestion_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_gestion_jeu.read();

    let nom_equipe = if est_equipe_joueur {
        "Player's team"
    } else {
        "Enemy team"
    };

    rsx! {
        div { class: "equipe",
            h2 { "{nom_equipe}" }
            for i in 0..Equipe::NOMBRE_MAX_MEMBRES {

                match jeu
                    .recuperer_id_membre_equipe(i, est_equipe_joueur)
                    .and_then(|id| jeu.recuperer_personnage(id))
                {
                    Some(perso) => rsx! {
                        div { class: "text-lg",
                            ul {
                                li { "Nom: {perso.nom}" }
                                li { "HP: {perso.pv}" }
                                li { "Max HP: {perso.pv_max}" }
                            }
                        }
                    },
                    None => rsx! {},
                }
            }
        }
    }
}
