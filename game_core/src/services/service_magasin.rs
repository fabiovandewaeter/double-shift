use crate::metier::magasin::Magasin;

pub struct ServiceMagasin;

impl ServiceMagasin {
    pub fn creer_prochain_magasin(niveau_actuel: u32) -> Magasin {
        Magasin::new(vec![1, 2, 3])
    }
}
