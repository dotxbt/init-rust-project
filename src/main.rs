pub mod util;
pub mod model;
pub mod repository;

use repository::user_repository;
fn main() {
    user_repository::try_user_repo();
}
