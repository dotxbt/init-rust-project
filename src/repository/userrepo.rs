use crate::model::user;

pub struct UserRepoModel {
    data: Vec<user::User>,
}

trait UserRepository {
    fn add_user(&mut self, user: user::User);
    fn update_user(&mut self, index: usize, user: user::User);
    fn get_users(self) -> Vec<user::User>;
    fn delete_user(&mut self, index: usize);
}

impl UserRepository for UserRepoModel {
    fn add_user(&mut self, user: user::User) {
        self.data.push(user);
    }

    fn update_user(&mut self, index: usize, user: user::User) {
        self.data[index] = user;
    }
    fn get_users(self) -> Vec<user::User> {
        return self.data;
    }

    fn delete_user(&mut self, index: usize) {
        self.data.remove(index);
    }
}

pub fn try_user_repo() {
    let mut repo = UserRepoModel { data: vec![] };
    for i in 0..=5 {
        repo.add_user(user::User {
            id: i,
            full_name: "Sabituddin Bibgbang".to_string(),
            email: "sabituddin@gmail.com".to_string(),
            password: "Hell0C00kz".to_string(),
        });
    }
    println!("ADD USER : {:?}", repo.data);
    repo.delete_user(0);
    println!("\n\nDELETE USER : {:?}", repo.data);
}
