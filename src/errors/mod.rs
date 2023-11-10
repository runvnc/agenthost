// Define a common SimpleRejection struct that can be reused across modules.
#[derive(Debug)]
pub struct SimpleRejection(pub String);

impl warp::reject::Reject for SimpleRejection {}
