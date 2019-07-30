use whoami::*;

/// User information.
pub struct User {
    env: DesktopEnv,
    host: String,
    hostname: String,
    os: String,
    platform: Platform,
    user: String,
    username: String,
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

impl User {
    pub(crate) fn new() -> User {
        User {
            env: env(),
            host: host(),
            hostname: hostname(),
            os: os(),
            platform: platform(),
            user: user(),
            username: username(),
        }
    }
}
