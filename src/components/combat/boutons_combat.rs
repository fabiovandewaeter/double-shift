use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn BoutonsCombat() -> Element {
    let mut signal_jeu = use_context::<Signal<GestionJeu>>();

    // désactive le bouton quand les events sont entrain d'être animés
    let mut est_en_animation = use_signal(|| false);
    let classe_bouton = if est_en_animation() {
        "bg-gray-400 cursor-not-allowed text-white font-bold py-2 px-4 rounded" // Gris, pas de curseur main
    } else {
        "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" // Bleu normal
    };

    rsx! {
        div {
            div { class: "flex gap-2",
                button {
                    class: classe_bouton,
                    disabled: est_en_animation(),

                    onclick: move |_| {
                        if est_en_animation() {
                            return;
                        }

                        spawn(async move {
                            est_en_animation.set(true);

                            {
                                signal_jeu.write().demarrer_tour();
                            }

                            loop {
                                TimeoutFuture::new(500).await;
                                let contiuer_tour = {
                                    let mut jeu = signal_jeu.write();
                                    match jeu.executer_un_pas_tour() {
                                        Ok(encore) => encore,
                                        Err(_) => false,
                                    }
                                };
                                if !contiuer_tour {
                                    break;
                                }
                            }

                            est_en_animation.set(false);
                        });
                    },
                    if est_en_animation() {
                        "Turn in process..."
                    } else {
                        "Next turn"
                    }
                }
            }
        }
    }
}
