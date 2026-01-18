pub struct Magasin {
    id_items_a_vendre: Vec<u32>,
}

impl Magasin {
    pub fn new(id_items_a_vendre: Vec<u32>) -> Self {
        Self { id_items_a_vendre }
    }

    pub fn id_items_a_vendre(&self) -> Vec<u32> {
        self.id_items_a_vendre.clone()
    }
}
