use juniper::RootNode;

pub mod context;
pub mod enums;
pub mod input;
pub mod mutation;
pub mod object;
pub mod query;

pub type Schema = RootNode<'static, query::QueryRoot, mutation::MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(query::QueryRoot {}, mutation::MutationRoot {})
}
