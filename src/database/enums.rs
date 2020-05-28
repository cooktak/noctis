#[derive(Debug, diesel_derive_enum::DbEnum, juniper::GraphQLEnum)]
pub enum Gender {
    Private,
    Male,
    Female,
    Etc,
}
