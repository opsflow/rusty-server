use std::collections::HashMap;

/// Declare a struct, QueryString, that will hold our string in a hashmap.
// Specify a lifetime for our QueryString <'buff>
#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// Implement the From trait for QueryString, to convert our buffer into the hashmap.
impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(s: &'buff str) -> Self {
        let mut data = HashMap::new();

        // Create and return our query string. (e.g. field1=value1&field2=value2&field3=value3...)
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
