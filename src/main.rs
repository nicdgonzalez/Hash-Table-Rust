
#[derive(Clone, Debug, Default)]
struct Bucket<Value> {
    key: String,
    value: Value,
    next: Option<*mut Box<Bucket<Value>>>
}

#[derive(Debug)]
struct HashTable<Value> {
    table: Vec<Bucket<Value>>,
    count: usize,
}

impl<Value: Clone + Default> HashTable<Value> {
    /// Constructs an empty hash table.
    pub fn new() -> Self {
        return Self {
            table: vec![Bucket::<Value>::default(); 16],
            count: 0,
        }
    }

    /// Enters a key-value pair into the hash table.
    /// If the key already exists, the value is overwritten.
    pub fn insert(&mut self, key: &str, value: Value) -> () {
        return;
    }

    /// Returns the value associated with the key,
    /// or ``None`` if the key is not present.
    pub fn get(&mut self, key: &str) -> Option<&Value> {
        return None;
    }

    /// Deletes a key from the table,
    /// returning ``true`` if the key was present.
    pub fn remove(&mut self, key: &str) -> bool {
        return false;
    }
}  // end impl HashTable

fn main() -> () {
    let mut table = HashTable::<&str>::new();
    table.insert("foo", "bar");
    println!("{:#?}", table);
}
