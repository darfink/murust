use schema::{item_definition, item_qualified_class};

#[derive(Identifiable, Queryable, AsChangeset)]
#[table_name = "item_definition"]
pub struct ItemDefinition {
  pub id: i32,
  pub name: String,
  pub group: i32,
  pub index: i32,
  pub modifier: i32,
  pub equippable_slot: Option<i32>,
  pub max_durability: i32,
  pub width: i32,
  pub height: i32,
  pub drop_from_monster: bool,
  pub drop_level: i32,
}

#[derive(Identifiable, Queryable)]
#[table_name = "item_qualified_class"]
#[primary_key(item_definition_id, class)]
pub struct ItemQualifiedClass {
  pub item_definition_id: i32,
  pub class: i32,
}