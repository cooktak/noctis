use juniper::GraphQLEnum;

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLEnum)]
pub enum Gender {
    Private,
    Male,
    Female,
    Etc,
}
