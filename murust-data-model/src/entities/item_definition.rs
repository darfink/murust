use types::{Class, ItemCode, ItemSlot};

#[derive(Debug)]
// TODO: Should Item definition arc itself?
pub struct ItemDefinition {
  pub code: ItemCode,
  pub name: String,
  pub equippable_slot: Option<ItemSlot>,
  pub max_durability: u8,
  pub width: u8,
  pub height: u8,
  pub drop_from_monster: bool,
  pub drop_level: u16,
  pub eligible_classes: Vec<Class>,
}

impl ItemDefinition {
  pub fn new<C: Into<ItemCode>, S: Into<String>>(code: C, name: S) -> Self {
    ItemDefinition {
      code: code.into(),
      name: name.into(),
      equippable_slot: None,
      max_durability: 0,
      width: 1,
      height: 1,
      drop_from_monster: false,
      drop_level: 0,
      eligible_classes: Vec::new(),
    }
  }
}
