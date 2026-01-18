use dioxus::prelude::*;
use game_core::{
    coordination::gestion_jeu::GestionJeu,
    metier::{etat_combat::EtatCombat, personnage::StatsPersonnage},
};

use crate::components::{
    combat::{
        boutons_combat::BoutonsCombat, description_combat::DescriptionCombat,
        description_equipe::DescriptionEquipe, ecran_fin_combat::EcranFinCombat,
    },
    description_personnage::DescriptionPersonnage,
};

mod components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
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

    // let id_ennemi = jeu.creer_personnage(
    //     "DarkSasuke".to_string(),
    //     StatsPersonnage {
    //         pv_max: 200,
    //         attaque: 5,
    //     },
    // );
    // jeu.ajouter_membre_equipe(id_ennemi, 0, false);

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
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::PageCombat {}, "Combat" }
            Link { to: Route::PagePersonnage { id: 0 }, "Personnage" }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

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

    let etat_combat = jeu.combat_actuel().etat_combat();

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
