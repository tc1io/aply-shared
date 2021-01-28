// use std::convert::Infallible;
// use warp::{Filter, Rejection};

#[derive(Clone)]
pub struct AuthProvider {}
#[derive(Clone)]
pub struct ResourceAtts {}
#[derive(Clone)]
pub struct Principal {}

#[derive(Debug)]
pub struct Error {}
impl warp::reject::Reject for Error {}

impl Error {
    pub fn into_rejection(self) -> warp::Rejection {
        warp::reject::custom(self)
    }
}

// impl std::error::Error for Error {
//     fn description(&self) -> &str {
//         &self.message
//     }
// }


// pub fn auth(
//     provider: AuthProvider,
//     _attributes: ResourceAtts,
// ) -> impl Filter<Extract = (Principal,), Error = Rejection> + Clone {
//
//     warp::header("authorization").and_then(|value: String| async move {
//         // if !value.as_bytes().starts_with(b"Bearer ") {
//         //     return warp::reject::not_found();
//         // }
//         if value.len() < 8 {
//             return futures::future::ready(Err(Error{}.into_rejection())).await;
//         }
//         let (a, x) = value.split_at("Bearer ".len());
//         println!("{}", x);
//         let principal: Principal = Principal {};
//         futures::future::ready(Ok(principal)).await
//     })
// }
