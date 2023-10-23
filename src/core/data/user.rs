use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    id: String,
    username: String,
    avatar: String,
    discriminator: String,
    global_name: String,
    public_flags: Option<i32>,
    flags: Option<i32>,
    banner: Option<String>,
    accent_color: Option<i32>,
    avatar_decoration_data: Option<String>,
    banner_color: Option<String>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    premium_type: i32,
}

 impl User {
    fn new(
        id: String,
        username: String,
        avatar: String,
        discriminator: String,
        public_flags: Option<i32>,
        flags: Option<i32>,
        banner: Option<String>,
        accent_color: Option<i32>,
        global_name: String,
        avatar_decoration_data: Option<String>,
        banner_color: Option<String>,
        mfa_enabled: Option<bool>,
        locale: Option<String>,
        premium_type: i32,
    ) -> User {
        User {
            id,
            username,
            avatar,
            discriminator,
            public_flags,
            flags,
            banner,
            accent_color,
            global_name,
            avatar_decoration_data,
            banner_color,
            mfa_enabled,
            locale,
            premium_type,
        }
    }
}