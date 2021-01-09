use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum JesterError<'a> {
    PermissionError(PermissionType<'a>),
    MissingError(&'a str),
    UnsuccessfulError(&'a str)
}

impl fmt::Display for JesterError <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JesterError::PermissionError(perm) => 
                write!(f, "{}", perm),
            JesterError::MissingError(missing) => write!(f, "Please provide a {}!", missing),
            JesterError::UnsuccessfulError(cmd) => 
                write!(f, "{} unsuccessful. The user must be in the guild and the bot must be above the user's role!", cmd)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum PermissionType<'b> {
    SelfPerm(&'b str),
    Mention(&'b str, &'b str)
}

impl fmt::Display for PermissionType <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PermissionType::SelfPerm(perm) => writeln!(f, "You can't execute this command because you're not a {} on this server!", perm),
            PermissionType::Mention(cmd, perm) => write!(f, "I can't {} an {}! Please demote the user and try again", cmd, perm)
        }
    }
}
