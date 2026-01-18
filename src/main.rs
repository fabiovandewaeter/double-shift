use dioxus::prelude::*;
use game_core::{
    coordination::gestion_jeu::GestionJeu,
    metier::{etat_combat::EtatCombat, etat_partie::EtatPartie},
};

use crate::components::{
    combat::{
        boutons_combat::BoutonsCombat, description_combat::DescriptionCombat,
        description_equipe::DescriptionEquipe, ecran_fin_combat::EcranFinCombat,
    },
    description_personnage::DescriptionPersonnage,
    magasin::liste_achats::ListeAchats,
};

mod components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    PageJeu {},
    #[route("/page_combat")]
    PageCombat{},
    #[route("/page_perso/:id")]
    PagePersonnage{id: u32},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut jeu = GestionJeu::new();
    use_context_provider(|| Signal::new(jeu));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::PageJeu {}, "Game" }
            Link { to: Route::PageCombat {}, "Combat" }
            Link { to: Route::PagePersonnage { id: 0 }, "Personnage" }
        }

        Outlet::<Route> {}
    }
}

/// Home page
#[component]
fn PageJeu() -> Element {
    let signal_jeu = use_context::<Signal<GestionJeu>>();
    let jeu = signal_jeu.read();

    match jeu.etat_partie() {
        EtatPartie::Combat(_) => rsx! {
            PageCombat {}
        },
        EtatPartie::Magasin(_) => rsx! {
            PageMagasin {}
        },
    }
}

#[component]
fn PagePersonnage(id: u32) -> Element {
    rsx! {
        DescriptionPersonnage { id }
    }
}

#[component]
fn PageCombat() -> Element {
    let jeu_signal = use_context::<Signal<GestionJeu>>();
    let jeu = jeu_signal.read();

    let EtatPartie::Combat(combat) = jeu.etat_partie() else {
        return rsx! {
            div {
                h1 { "No battle started" }
            }
        };
    };
    let etat_combat = combat.etat_combat();

    rsx! {
        div { class: "",
            DescriptionCombat {}

            if etat_combat != EtatCombat::EnCours {
                EcranFinCombat { etat: etat_combat.clone() }
            }

            DescriptionEquipe { est_equipe_joueur: false }
            DescriptionEquipe { est_equipe_joueur: true }

            if etat_combat == EtatCombat::EnCours {
                BoutonsCombat {}
            }
        }
    }
}

#[component]
fn PageMagasin() -> Element {
    rsx! {
        ListeAchats {}
    }
}
