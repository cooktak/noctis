table! {
    cookery (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    food_type (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    ingredient (id) {
        id -> Integer,
        name -> Varchar,
        parentId -> Nullable<Integer>,
    }
}

table! {
    post (id) {
        id -> Integer,
        createTime -> Datetime,
        title -> Varchar,
        userId -> Integer,
    }
}

table! {
    post_content (id) {
        id -> Integer,
        content -> Varchar,
        photoLink -> Varchar,
        postId -> Integer,
    }
}

table! {
    post_cookery (postId, cookeryId) {
        postId -> Integer,
        cookeryId -> Integer,
    }
}

table! {
    post_food_type (postId, foodTypeId) {
        postId -> Integer,
        foodTypeId -> Integer,
    }
}

table! {
    post_ingredient (postId, ingredientId) {
        postId -> Integer,
        ingredientId -> Integer,
        amount -> Integer,
        unit -> Varchar,
    }
}

table! {
    post_product (postId, productId) {
        postId -> Integer,
        productId -> Varchar,
    }
}

table! {
    product (id) {
        id -> Varchar,
        info -> Varchar,
        KANProductCategoryCode -> Varchar,
        name -> Varchar,
        photoLink -> Nullable<Varchar>,
        unit -> Varchar,
        wight -> Varchar,
        sellerId -> Integer,
        vendorId -> Integer,
    }
}

table! {
    seller (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    token (id) {
        id -> Integer,
        accessToken -> Varchar,
        refreshToken -> Varchar,
        userId -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        birthday -> Datetime,
        createTime -> Datetime,
        gender -> Enum,
        nickname -> Varchar,
        password -> Varchar,
        username -> Varchar,
        userTag -> Integer,
        photoLink -> Nullable<Varchar>,
    }
}

table! {
    vendor (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(post -> user (userId));
joinable!(post_content -> post (postId));
joinable!(post_cookery -> cookery (cookeryId));
joinable!(post_cookery -> post (postId));
joinable!(post_food_type -> food_type (foodTypeId));
joinable!(post_food_type -> post (postId));
joinable!(post_ingredient -> ingredient (ingredientId));
joinable!(post_ingredient -> post (postId));
joinable!(post_product -> post (postId));
joinable!(post_product -> product (productId));
joinable!(product -> seller (sellerId));
joinable!(product -> vendor (vendorId));
joinable!(token -> user (userId));

allow_tables_to_appear_in_same_query!(
    cookery,
    food_type,
    ingredient,
    post,
    post_content,
    post_cookery,
    post_food_type,
    post_ingredient,
    post_product,
    product,
    seller,
    token,
    user,
    vendor,
);
