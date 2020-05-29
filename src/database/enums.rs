#[derive(Debug, juniper::GraphQLEnum)]
pub enum Gender {
    Private,
    Male,
    Female,
    Etc,
}
