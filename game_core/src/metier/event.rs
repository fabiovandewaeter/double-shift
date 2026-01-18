#[derive(Clone, Debug)]
pub enum Event {
    FaireDegats {
        cible_id: u32,
        quantite: i32,
        source_id: Option<u32>,
    },
    Soigner {
        cible_id: u32,
        quantite: i32,
        source_id: Option<u32>,
    },
    Mort {
        id: u32,
    },
}
