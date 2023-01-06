pub trait DataRepositoryInterface<T> {
    fn add_item(&mut self, item: T);
    fn get_item(&mut self, index: usize) -> &T;
    fn update_item(&mut self, index: usize, item: T);
    fn delete_item(&mut self, index: usize);
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password: String,
}

pub struct UserRepository {
    pub(crate) data: Vec<User>,
}

impl DataRepositoryInterface<User> for UserRepository {
    fn add_item(&mut self, item: User) {
        self.data.push(item);
    }

    fn get_item(&mut self, index: usize) -> &User {
        return self.data.get(index).unwrap();
    }

    fn update_item(&mut self, index: usize, item: User) {
        self.data[index] = item;
    }

    fn delete_item(&mut self, index: usize) {
        self.data.remove(index);
    }
}

