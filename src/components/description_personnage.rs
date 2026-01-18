use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;

use crate::Route;

#[component]
pub fn DescriptionPersonnage(id: u32) -> Element {
    // récupérer perso
    let signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();
    let depot_personnages = jeu.depot_personnages();
    let option_perso = depot_personnages.recuperer_personnage(id);

    match option_perso {
        Some(perso) => rsx! {
            div { class: "flex flex-col gap-4 p-4 border rounded shadow-lg",

                h1 { class: "text-2xl font-bold", "Fiche : {perso.nom}" }

                div { class: "text-lg",
                    ul {
                        li { "HP: {perso.pv}" }
                        li { "Max HP: {perso.pv_max}" }
                    }
                }

                div {
                    if id > 0 {
                        Link {
                            to: Route::PagePersonnage {
                                id: id - 1,
                            },
                            "Previous"
                        }
                        span { " <---> " }
                    }
                    Link {
                        to: Route::PagePersonnage {
                            id: id + 1,
                        },
                        "Next"
                    }
                }
            }
        },
        None => rsx! { "Can't find character" },
    }
}
