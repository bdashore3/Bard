use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BardError<'a> {
    Permission(PermissionType<'a>),
    Missing(&'a str),
    Unsuccessful(&'a str),
}

impl fmt::Display for BardError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BardError::Permission(perm) =>
                write!(f, "{}", perm),
            BardError::Missing(missing) => write!(f, "Please provide a {}!", missing),
            BardError::Unsuccessful(cmd) =>
                write!(f, "{} unsuccessful. The user must be in the guild and the bot must be above the user's role!", cmd)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum PermissionType<'b> {
    UserPerm(&'b str),
    Mention(&'b str, &'b str),
    External(&'b str),
}

impl fmt::Display for PermissionType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PermissionType::UserPerm(perm) => writeln!(
                f,
                "You can't execute this command because you do not have the `{}` permission!",
                perm
            ),
            PermissionType::Mention(cmd, perm) => write!(
                f,
                "I can't {} an {}! Please demote the user and try again",
                cmd, perm
            ),
            PermissionType::External(perm) => write!(
                f,
                "The bot can't execute this because it doesn't have the `{}` permission! Please grant it on the bot's role.",
                perm
            )
        }
    }
}
