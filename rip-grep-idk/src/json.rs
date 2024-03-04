pub mod JSON {

    use std::{collections::{HashMap}};

    /**
     * This first iteration of implementation, maybe I'll use some MappingConfig with class
     * description and it's filed later. Who knows.
     */
    struct FieldDescriptor<T> {
        field_name: String,
        callback: Box<dyn FnOnce() -> T>,
        return_type: T,
    }

    impl<T> FieldDescriptor<T>
    {
        pub fn get_value(self) -> T {
            let f = self.callback;
            f()
        }
    }

    pub trait Serializable {
        fn get_serializable_fields(&self) -> Vec<FieldDescriptor<String>>;
    }

    // oh, I've no idea of what I'm doing :)
    struct Json {
        map: HashMap<String, String>,
    }

    impl Json {
       pub fn get(&self, key: &str) -> Option<&String> {
           self.map.get(key)
       }

       pub fn put(&mut self, key: String, value: String) {
           self.map.insert(key, value);
       }

       pub fn default() -> Json {
           Json { map: HashMap::new() }
       }
    }

    struct JsonMapper {
    }

    impl JsonMapper {

        fn default() -> JsonMapper {
            JsonMapper {
            }
        }

        pub fn to_json(&self, value: &dyn Serializable) -> Option<Json> {
            let fields = value.get_serializable_fields();

            let mut json = Json::default();
            for field in fields {
                let field_name = field.field_name.clone();
                let value = field.get_value();
                json.put(field_name, value);
            }

            return Some(json);
        }
    }
}
