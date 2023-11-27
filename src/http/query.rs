use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct Query<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> Query<'buf> {
    pub fn get(&self, key: &'buf str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

// Exp: a=1&b=2&b=3&c=4
impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(value: &'buf str) -> Self {
        // Create a hash map to collect key and value
        let mut data: HashMap<&'buf str, Value<'buf>> = HashMap::new();
        
        if !value.is_empty() {
            // Split out with '&'
            for split_value in value.split('&') {
                let mut k = "";
                let mut v = "";

                // To find '=' for separate key and value
                if let Some(i) = split_value.find('=') {
                    k = &split_value[..i]; // Key
                    v = &split_value[i + 1..]; // Value (Plus one for prevent '=')
                }

                // HashMap 'entry' is finding the key for manipulation
                data.entry(k)
                    .and_modify(|exist: &mut Value| match exist {
                        Value::Single(prev_val) => {
                            // Dereference 'exist' to get value from 'Value::Single'
                            // And change to 'Value::Multiple' from 'Value::Single' for support multiple of values
                            *exist = Value::Multiple(vec![prev_val, v]);
                        } // If key is duplicated, then change to 'Value::Multiple' for save multipe of values by key
                        Value::Multiple(prev_vec) => prev_vec.push(v), // Push to vector for new value
                    }) // To find the key and modify previous of value
                    .or_insert(Value::Single(v)); // By default to insert value if mismatch the key
            }
        }

        Query { data }
    }
}
