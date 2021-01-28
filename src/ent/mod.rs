/// A string that is a valid solution ID.
pub struct SolutionId(pub String);

/// Generic error for turning strings into more constrained string types.
#[derive(Clone, Debug)]
pub struct StringError { cause:String }

impl std::str::FromStr for SolutionId {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("parse Solution ID {}",s);
        // TODO This is just dummy code to illustrate the validity check
        if s.len() > 10 {
            return Err(StringError{cause:"String to long".into()})
        }
        Ok(SolutionId(s.into()))
    }
}

