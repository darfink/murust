use context::{DataContext, DataContextInner};
use diesel::prelude::*;
use error::Result;
use models::Inventory;
use schema::inventory::dsl;

/// A repository for inventories.
#[derive(Clone)]
pub struct InventoryRepository {
  context: DataContextInner,
}

impl InventoryRepository {
  /// Creates a new inventory repository instance.
  pub fn new(context: &DataContext) -> Self {
    InventoryRepository {
      context: context.inner(),
    }
  }

  /// Returns an inventory by its ID.
  pub fn find_by_id(&self, id: i32) -> Result<Option<Inventory>> {
    dsl::inventory
      .find(id)
      .first::<Inventory>(&*self.context.access())
      .optional()
      .map_err(Into::into)
  }
}
