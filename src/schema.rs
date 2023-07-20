// @generated automatically by Diesel CLI.

diesel::table! {
    comment_likes (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 36]
        comment_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    comments (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 36]
        post_id -> Varchar,
        text -> Text,
        score -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    friendships (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_requesting -> Varchar,
        #[max_length = 36]
        user_responding -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    post_likes (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 36]
        post_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 255]
        watch_id -> Varchar,
        review -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        nick_name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        avatar -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    watch_images (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        watch_id -> Varchar,
        #[max_length = 36]
        title -> Varchar,
        data -> Nullable<Text>,
    }
}

diesel::table! {
    watches (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        brand -> Varchar,
        #[max_length = 255]
        model -> Varchar,
        #[max_length = 255]
        diameter -> Varchar,
        #[max_length = 255]
        lug_width -> Varchar,
        #[max_length = 255]
        case_material -> Varchar,
        #[max_length = 255]
        mechanism_model -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(comment_likes -> comments (comment_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(post_likes -> posts (post_id));
diesel::joinable!(post_likes -> users (user_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(posts -> watches (watch_id));
diesel::joinable!(watch_images -> watches (watch_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment_likes,
    comments,
    friendships,
    post_likes,
    posts,
    users,
    watch_images,
    watches,
);
