use juniper::EmptySubscription;

pub mod root;
use root::{
    Context,
    Mutation,
    Query,
    Schema,
};
pub mod user;

pub fn create_schema() -> Schema {
    // Schemaオブジェクトを新規に作成する関数.
    Schema::new(
        Query {},
        Mutation {},
        EmptySubscription::<Context>::new()
    )
}
