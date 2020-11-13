use super::ply;

// DB connection that maintains any shared state needed by all remote
// handles (See Handle struct). Example would ge auth tokens generated
// for accessing eg Google Datastore API.
pub struct Db {
    handle: Handle,
}
pub fn new() -> Db {
    Db {
        handle:Handle{},
    }
}

impl Db {

    // Get a handle to the DB object that can be send across threads.
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

}

#[derive(Debug, Clone)]
pub struct Handle {
}

impl Handle {

    pub fn get_system(&self,id: &str) ->  impl std::future::Future<Output=std::result::Result<ply::System,i32>> {
        let system = ply::System{name:String::from(id)};
        std::future::ready(std::result::Result::Ok(system))
    }

}


// trait LES[T] {
//
// }