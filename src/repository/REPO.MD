### SETPS :

- create struct
  ```rust
  struct RepoName {
      data: Vect<String>,
      selectedIdx: usize,
  }
  ```
- implement without struct

  ```rust
  impl RepoName {
      fn add_item(&mut self, item: String) {
          // implement add user
      }

      fn get_item(&mut self, index:usize) -> &String {
          // implement with return item
      }

      fn update_item(&mut self, item: String, index: usize) {
          // implement update item
          // change item using index position
      }

      fn delete_item(&mut self, index: usize) {
          // implement delete item
          // delete item with index position
      }
  }
  ```

### IMPLEMENT USING INTERFACE | GENERIC

- create trait

  ```rust
  pub trait RepoNameInterface<T> {
      fn add_item(&mut self, item: T);
      fn get_item(&mut self, index: usize) -> &T;
      fn update_item(&mut self, item: T, index: usize);
      fn delete_item(&mut self, index: usize);
  }

  impl RepoNameInterface<User> for RepoName1 {
      fn add_item(&mut self, item: User) {
          // implement add user
      }

      fn get_items(self) -> Vect<User> {
          // implement with return items
      }

      fn get_item(&mut self, index:usize) -> &User {
          // implement with return item
      }

      fn update_item(&mut self, item: User, index: usize) {
          // implement update item
          // change item using index position
      }

      fn delete_item(&mut self, index: usize) {
          // implement delete item
          // delete item with index position
      }
  }

  impl RepoNameInterface<String> for RepoName2 {
      fn add_item(&mut self, item: String) {
          // implement add user
      }

      fn get_items(self) -> Vect<String> {
          // implement with return items
      }

      fn get_item(&mut self, index:usize) -> String {
          // implement with return item
      }

      fn update_item(&mut self, item: String, index: usize) {
          // implement update item
          // change item using index position
      }

      fn delete_item(&mut self, index: usize) {
          // implement delete item
          // delete item with index position
      }
  }
  ```
