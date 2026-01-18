use crate::metier::{combat::Combat, magasin::Magasin};

pub enum EtatPartie {
    Combat(Combat),
    Magasin(Magasin),
}
