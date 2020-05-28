use juniper::GraphQLInputObject;

use crate::gql::enums::Episode;

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}