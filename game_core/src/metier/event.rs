#[derive(Clone, Debug)]
pub enum Event {
    FaireDegats {
        id_cible: u32,
        quantite: i32,
        id_source: Option<u32>,
    },
    Soigner {
        id_cible: u32,
        quantite: i32,
        id_source: Option<u32>,
    },
    Mort {
        id: u32,
    },
}
