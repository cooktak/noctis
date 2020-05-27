use juniper::RootNode;

pub mod enums;

pub mod object;

pub mod input;

pub mod query;

pub mod mutation;

pub type Schema = RootNode<'static, query::QueryRoot, mutation::MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(query::QueryRoot {}, mutation::MutationRoot {})
}