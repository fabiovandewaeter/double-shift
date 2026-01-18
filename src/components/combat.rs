use dioxus::prelude::*;
use game_core::coordination::gestion_jeu::GestionJeu;

#[component]
pub fn BoutonsCombat() -> Element {
    // récupérer perso
    let mut signal_gestion_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_gestion_jeu.read();

    rsx! {}
    // rsx! {
    //     div {
    //         div { class: "flex gap-2",
    //             button {
    //                 class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
    //                 onclick: move |_| {
    //                     signal_gestion_jeu.write().ajouter_bonus_argent(id);
    //                 },
    //                 "Ajouter bonus argent une fois (+{perso.bonus.0})"
    //             }

    //             button {
    //                 class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
    //                 onclick: move |_| {
    //                     // let farm_auto_actif = perso.farm_auto;

    //                     let farm_auto_actif = match signal_gestion_jeu.read().recuperer_personnage(id) {
    //                         Some(perso) => perso.farm_auto,
    //                         None => false,
    //                     };

    //                     signal_gestion_jeu.write().switch_farm_auto(id);

    //                     if !farm_auto_actif {
    //                         spawn(async move {
    //                             loop {
    //                                 TimeoutFuture::new(1_000).await;
    //                                 let farm_auto_actif = {
    //                                     let gestion_jeu = signal_gestion_jeu.read();
    //                                     match gestion_jeu.recuperer_personnage(id) {
    //                                         Some(perso) => perso.farm_auto,
    //                                         None => false,
    //                                     }
    //                                 };
    //                                 if !farm_auto_actif {
    //                                     break;
    //                                 }
    //                                 if signal_gestion_jeu.write().ajouter_bonus_argent(id).is_err() {
    //                                     break;
    //                                 }
    //                             }
    //                         });
    //                     }
    //                 },
    //                 "Ajouter bonus argent toutes les secondes (+{perso.bonus.0}/s) (etat : {perso.farm_auto})"
    //             }
    //         }
    //     }
    // }
}
