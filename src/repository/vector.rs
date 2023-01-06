
#[derive(Debug)]
struct ModelData {
    _name: String,
    _email: String,
}

struct DataRepo {
    _data: Vec<ModelData>,
}

impl DataRepo {
    fn add_data(&mut self, item: ModelData) {
        self._data.push(item);
    }

    fn get_data(&mut self, index: usize) -> &ModelData {
        return self._data.get(index).unwrap();
    }

    fn update_data(&mut self, index:usize, new_item: ModelData) {
        self._data[index] = new_item;
    }

    fn delete_data(&mut self, index: usize) {
        self._data.remove(index);
    }
}

pub fn test_data_repo() {
    let mut data_repo = DataRepo { _data: vec![] };
    for _ in 0..=5 {
        data_repo.add_data(ModelData {
            _name: String::from("Sabituddin Bigbang"),
            _email: String::from("sabit@gmail.com"),
        });
    }
    println!("DATA  ke - 1: {:?}", data_repo._data);
    println!("DATA  ke - 1: {:?}", data_repo.get_data(1));
    data_repo.update_data(1, ModelData {
        _name: String::from("Oke Oce"),
        _email: String::from("sabit12@gmail.com"),
    });
    println!("Update DATA  ke - 1: {:?}", data_repo.get_data(1));
    data_repo.delete_data(1);
    
}
