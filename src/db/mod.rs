
pub struct Db {
    handle: Handle,
}
pub fn new() -> Db {
    Db {
        handle:Handle{},
    }
}

impl Db {

    pub fn handle(&self) -> &Handle {
        &self.handle
    }

}

#[derive(Debug, Clone)]
pub struct Handle {

}

impl Handle {

    pub fn handle(&self) -> &Handle {
        &self
    }


}


// trait LES[T] {
//
// }