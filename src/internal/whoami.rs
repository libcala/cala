// This is one of the global contexts of Cala.  This one is particularly fast because it
// is read only, and multiple threads can read the data at the same time.

/// User information.
pub struct User {
    /// User's desktop environment.
    pub env: &'static str,
    /// The user's host machine's name.
    pub host: &'static str,
    /// The user's host machine's hostname.
    pub hostname: &'static str,
    /// The user's host machine's operating system.
    pub os: &'static str,
    /// The user's host machine's platform.
    pub platform: &'static str,
    /// The user's full name.
    pub user: &'static str,
    /// The user's username.
    pub username: &'static str,
}

struct UserInfo {
    pub env: String,
    pub host: String,
    pub hostname: String,
    pub os: String,
    pub platform: String,
    pub user: String,
    pub username: String,
}

static mut USER_INFO: FakeUserInfo = FakeUserInfo([0; std::mem::size_of::<UserInfo>()]);

#[repr(align(8))]
struct FakeUserInfo([u8; std::mem::size_of::<UserInfo>()]);

// // // // // //

pub(crate) fn initialize_user_io() {
    use whoami::*;

    unsafe {
        let user_info = &mut USER_INFO as *mut _ as *mut UserInfo;

        *user_info = UserInfo {
            env: env().to_string(),
            host: host(),
            hostname: hostname(),
            os: os(),
            platform: platform().to_string(),
            user: whoami::user(),
            username: username(),
        };
    }
}

/// Get information about the current user.
pub fn user() -> User {
    let userinfo = unsafe { &USER_INFO as *const _ as *const UserInfo };

    User {
        /// User's desktop environment.
        env: unsafe { &(*userinfo).env },
        /// The user's host machine's name.
        host: unsafe { &(*userinfo).host },
        /// The user's host machine's hostname.
        hostname: unsafe { &(*userinfo).hostname },
        /// The user's host machine's operating system.
        os: unsafe { &(*userinfo).os },
        /// The user's host machine's platform.
        platform: unsafe { &(*userinfo).platform },
        /// The user's full name.
        user: unsafe { &(*userinfo).user },
        /// The user's username.
        username: unsafe { &(*userinfo).username },
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "User {{")?;
        writeln!(f, "    env: \"{}\"", self.env)?;
        writeln!(f, "    host: \"{}\"", self.host)?;
        writeln!(f, "    hostname: \"{}\"", self.hostname)?;
        writeln!(f, "    os: \"{}\"", self.os)?;
        writeln!(f, "    platform: \"{}\"", self.platform)?;
        writeln!(f, "    user: \"{}\"", self.user)?;
        writeln!(f, "    username: \"{}\"", self.username)?;
        write!(f, "}}")
    }
}
