
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

impl<Value: Copy + Clone + Default> HashTable<Value> {
    /// Constructs an empty hash table.
    pub fn new() -> Self {
        return Self {
            table: vec![Bucket::<Value>::default(); 16],
            count: 0,
        }
    }

    fn _hash(&mut self, key: &str) -> usize {
        let mut hash: usize = 1;
        for c in key.chars() {
            hash *= 163;
            hash += c as usize;
        }
        hash %= self.table.len() as usize;
        return hash;
    }

    fn _is_prime(&mut self, value: usize) -> bool {
        if value == 0 || value == 1 {
            return false;
        }
        for i in 2..(value / 2) {
            if value % i == 0 {
                return false;
            }
        }
        return true;
    }

    fn _next_prime(&mut self, value: usize) -> usize {
        let mut result: usize = value;
        if result % 2 == 0 {
            result += 1;
        }
        while !self._is_prime(result) {
            result += 2;
        }
        return result;
    }

    /// Enters a key-value pair into the hash table.
    /// If the key already exists, the value is overwritten.
    pub fn insert(&mut self, key: &str, value: Value) -> () {
        let hash: usize = self._hash(key);
        let mut bucket: &mut Bucket<Value> = &mut self.table[hash];

        // Check if there is already a key in the bucket.
        if bucket.key != "" {
            // Check if the key already exists at the head of the linked list.
            if bucket.key == key {
                // The key already exists, so overwrite the value.
                bucket.value = value;
                return;
            }
            // We encountered a hash collision.
            // Check if the key is in any of the linked buckets.
            while bucket.next != None {
                // TODO: Is this safe? Consider pattern matching instead.
                bucket = unsafe { &mut *bucket.next.unwrap() };
                if bucket.key == key {
                    bucket.value = value;
                    return;
                }
            }
        }
        // Either this bucket was empty or we made it to the end of
        // the linked list. We can just add the key-value pair.
        bucket.key = key.to_string();
        bucket.value = value;
        bucket.next = None;
        self.count += 1;
        // Check if the table is at 75% capacity.
        if self.count < (self.table.len() * (3 / 4)) {
            return;
        }
        // TODO: Resize the table ((capacity * 2) + 1).
        return;
    }

    /// Returns the value associated with the key,
    /// or ``None`` if the key is not present.
    pub fn get(&mut self, key: &str) -> Option<&Value> {
        return None;
    }

    /// Deletes a key from the table,
    /// returning ``false`` if the key was not present.
    pub fn remove(&mut self, key: &str) -> bool {
        return false;
    }
}  // end impl HashTable

fn main() -> () {
    let mut table: HashTable<&str> = HashTable::<&str>::new();
    table.insert("foo", "bar");
    println!("{:#?}", table);
}
