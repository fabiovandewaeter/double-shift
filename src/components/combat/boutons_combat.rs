use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;

#[component]
pub fn BoutonsCombat() -> Element {
    let mut signal_gestion_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_gestion_jeu.read();

    // rsx! {}
    rsx! {
        div {
            div { class: "flex gap-2",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| {
                        signal_gestion_jeu.write().prochain_tour();
                    },
                    "Next turn"
                }
            }
        }
    }
}
