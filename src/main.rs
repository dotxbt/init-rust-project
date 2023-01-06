pub mod util;
pub mod model;
pub mod repository;

use repository::userrepo;
fn main() {
    userrepo::try_user_repo();
}
