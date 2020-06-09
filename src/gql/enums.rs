use juniper::GraphQLEnum;

#[derive(GraphQLEnum)]
pub enum Gender {
    Private,
    Male,
    Female,
    Etc,
}
