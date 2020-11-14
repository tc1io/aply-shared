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

impl Db {

    // Get a handle to the DB object that can be send across threads.
    pub fn handle<T:Default + Element + Clone>(&self) -> Handle<T> {
        Handle::new()
    }

}

#[derive(Debug, Clone)]
pub struct Handle<T> {
    _marker: PhantomData<T>,
}

impl<T:Default + Element + Clone> Handle<T> {
    pub fn new() -> Handle<T> {
        Handle {
            _marker: PhantomData,
        }
    }

    pub fn get(&self, id: &str) -> impl std::future::Future<Output=std::result::Result<Data<T>, DbError>> {
        //let system = ply::System{name:String::from(id)};
        //std::future::ready(std::result::Result::Ok(system))
        let s = T::default();
        let d = Single(s);
        std::future::ready(std::result::Result::Ok(d))
        //std::future::ready(std::result::Result::Err(0))

    }
}


pub enum Data<T:Element> {
    Single(T),
    Many(std::vec::Vec<T>)
}

pub trait Element {
    fn merge(&mut self,rhs:&Self);
}

