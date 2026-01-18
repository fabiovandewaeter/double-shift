use dioxus::prelude::*;
use game_core::{coordination::gestion_jeu::GestionJeu, metier::etat_partie::EtatPartie};

#[component]
pub fn ListeAchats() -> Element {
    let mut signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();

    let EtatPartie::Magasin(magasin) = jeu.etat_partie() else {
        return rsx! {
            div {
                h1 { "Shop closed for now" }
            }
        };
    };

    let equipe_joueur = jeu.equipe(true);
    let depot_items = jeu.depot_items();

    rsx! {
        div { class: "magasin",
            h2 { "Shop" }
            div { class: "text-lg",
                ul {
                    li { "Coins: {equipe_joueur.argent()}" }
                }
            }
            for id_item in magasin.id_items_a_vendre() {
                match depot_items.recuperer_item(id_item) {
                    Some(item) => rsx! {
                        div { class: "text-lg",
                            ul {
                                li { "Nom: {item.nom}" }
                                li { "HP bonus: {item.stats.pv}" }
                                li { "Max HP bonus: {item.stats.pv_max}" }
                                li { "Attack bonus: {item.stats.attaque}" }
                            }
                            span { "---" }
                        }
                    },
                    None => rsx! {},
                }
            }

            div { class: "mt-8 border-t pt-4",
                button {
                    class: "bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded w-full",
                    onclick: move |_| {
                        signal_jeu.write().lancer_prochain_combat();
                    },
                    "Start Next Battle ->"
                }
            }
        }
    }
}
