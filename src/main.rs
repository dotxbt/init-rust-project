use repository::user_repository::{DataRepositoryInterface, User, UserRepository};

pub mod repository;
pub mod util;

fn main() {
    let mut repo = UserRepository { data: vec![] };
    for n in 0..=10 {
        repo.add_item(User {
            id: n,
            full_name: String::from("Sabituddin"),
            email: String::from("sabit@gmail.com"),
            password: String::from("H3ll0W0rlD"),
        });
    }
    println!("DATA : {:?}", repo.data);
    println!("\nDATA Idx 2 : {:?}", repo.get_item(2));
}
