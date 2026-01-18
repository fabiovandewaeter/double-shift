use dioxus::prelude::*;
use game_core::{
    coordination::gestion_jeu::GestionJeu,
    metier::{etat_combat::EtatCombat, etat_partie::EtatPartie},
};

#[component]
pub fn EcranFinCombat(etat: EtatCombat) -> Element {
    let mut signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();

    let EtatPartie::Combat(combat) = jeu.etat_partie() else {
        return rsx! {
            div {
                h1 { "No battle started" }
            }
        };
    };

    let recompense = combat.recompense();

    match etat {
        EtatCombat::Victoire => rsx! {
            div { class: "overlay...",
                h1 { "VICTORY !" }
                p { "Rewards : {recompense} coins" }
            }

            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| {
                    let mut jeu_mut = signal_jeu.write();
                    jeu_mut.terminer_combat_et_recolter_et_aller_au_magasin();
                    // jeu_mut.lancer_prochain_combat();
                },
                "Next Battle ->"
            }
        },
        EtatCombat::Defaite => rsx! {
            div {
                h1 { "Game Over" }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| {
                        signal_jeu.write().reset_jeu();
                    },
                    "Restart Game"
                }
            }
        },
        _ => return rsx! {},
    }
}
