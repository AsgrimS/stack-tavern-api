pub mod user;

/// This trait is used to implement the CRUD operations for the models.
/// it contains the name of the table in the database.
pub trait TableModel {
    ///  The name of the table in the database.
    const TABLE_NAME: &'static str;
}
