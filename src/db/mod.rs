//use super::ply;
use std::marker::PhantomData;
use crate::db::Data::Single;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {

    ///
    #[error("empty error")]
    EmptyError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    OtherError(#[from] std::io::Error),
}




// DB connection that maintains any shared state needed by all remote
// handles (See Handle struct). Example would ge auth tokens generated
// for accessing eg Google Datastore API.
pub struct Db {
    //handle: Handle,
}
pub fn new() -> Db {
    Db {
        //handle:Handle{},
    }
}

pub struct DbEntity(pub String);

impl Db {

    // Get a data element type specific handle to the DB object that can be send across threads.
    pub fn handle<T:Default + Element + Clone + From<DbEntity>>(&self) -> Handle<T> {
        Handle::new()
    }

}

#[derive(Debug, Clone)]
pub struct Handle<T> {
    // Needed because Handle is only generic over T but does not contain a T
    _marker: PhantomData<T>,
}

impl<T:Default + Element + Clone + From<DbEntity>> Handle<T> {
    pub fn new() -> Handle<T> {
        Handle {
            _marker: PhantomData,
        }
    }

    // Dummy implementation of a DB retrieval
    pub fn get(&self, id: &str) -> impl std::future::Future<Output=std::result::Result<Data<T>, DbError>> {
        //let assembly = ply::System{name:String::from(id)};
        //std::future::ready(std::result::Result::Ok(assembly))
        let db_entity = DbEntity(String::from(id));
        let s:T = db_entity.into();
        let d = Single(s);
        std::future::ready(std::result::Result::Ok(d))

    }
}


// Wrapper type that is the LWW Element Set like CRDT. There is a contract imposed
// on elements of the LWW set so that merges can be done or conflicts can be made
// explicit.
pub enum Data<T:Element> {
    // This is the case when only one state of the entity exists.
    Single(T),
    // This is the case when loading from storage resulted in versions of the entity
    // that could not be merged automatically.
    Many{head: T,tail: std::vec::Vec<T>}
}

impl<T:Element> Data<T> {
    // Helper function so we can work even if the actual workings of the data replication is not yet done
    pub fn extract_one(self) -> T {
        match self {
            Self::Single(d) => d,
            Self::Many{head,..} => head
        }
    }

    // Helper function so we can work even if the actual workings of the data replication is not yet done
    pub fn force_resolve(self) -> T {
        match self {
            Self::Single(s) => s,
            Self::Many{head,tail} => tail.iter().fold(head,T::fmerge),
        }
    }

}

// The contract that all CRDT type elements need to implement
// so merges can happen or conflicts can be made explicit.
pub trait Element {
    fn merge(&mut self,rhs:&Self);
    fn fmerge(self,rhs:&Self) -> Self;
}

