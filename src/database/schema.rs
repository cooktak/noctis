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
        parent_id -> Nullable<Integer>,
    }
}

table! {
    post (id) {
        id -> Integer,
        create_time -> Datetime,
        title -> Varchar,
        user_id -> Integer,
    }
}

table! {
    post_content (id) {
        id -> Integer,
        content -> Varchar,
        photo_link -> Varchar,
        post_id -> Integer,
    }
}

table! {
    post_cookery (post_id, cookery_id) {
        post_id -> Integer,
        cookery_id -> Integer,
    }
}

table! {
    post_food_type (post_id, food_type_id) {
        post_id -> Integer,
        food_type_id -> Integer,
    }
}

table! {
    post_ingredient (post_id, ingredient_id) {
        post_id -> Integer,
        ingredient_id -> Integer,
        amount -> Integer,
        unit -> Varchar,
    }
}

table! {
    post_product (post_id, product_id) {
        post_id -> Integer,
        product_id -> Varchar,
    }
}

table! {
    product (id) {
        id -> Varchar,
        info -> Varchar,
        kan_product_category_code -> Varchar,
        name -> Varchar,
        photo_link -> Nullable<Varchar>,
        unit -> Varchar,
        wight -> Varchar,
        seller_id -> Integer,
        vendor_id -> Integer,
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
        access_token -> Varchar,
        refresh_token -> Varchar,
        user_id -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        birthday -> Datetime,
        create_time -> Datetime,
        gender -> Varchar,
        nickname -> Varchar,
        password -> Varchar,
        username -> Varchar,
        user_tag -> Integer,
        photo_link -> Nullable<Varchar>,
    }
}

table! {
    vendor (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(post -> user (user_id));
joinable!(post_content -> post (post_id));
joinable!(post_cookery -> cookery (cookery_id));
joinable!(post_cookery -> post (post_id));
joinable!(post_food_type -> food_type (food_type_id));
joinable!(post_food_type -> post (post_id));
joinable!(post_ingredient -> ingredient (ingredient_id));
joinable!(post_ingredient -> post (post_id));
joinable!(post_product -> post (post_id));
joinable!(post_product -> product (product_id));
joinable!(product -> seller (seller_id));
joinable!(product -> vendor (vendor_id));
joinable!(token -> user (user_id));

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
