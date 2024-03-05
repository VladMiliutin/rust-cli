pub mod JSON {

    use std::{collections::{HashMap}};

    /**
     * This first iteration of implementation, maybe I'll use some MappingConfig with class
     * description and it's filed later. Who knows.
     */
    pub struct FieldDescriptor<'a, T> {
        pub field_name: String,
        pub callback: Box<dyn FnOnce() -> T + 'a>,
    }

    impl<'a, T> FieldDescriptor<'a, T>
    {
    }

    pub trait Serializable {
        fn get_serializable_fields(&self) -> Vec<FieldDescriptor<String>>;
    }

    // oh, I've no idea of what I'm doing :)
    #[derive(Debug)]
    pub struct Json {
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

    pub struct JsonMapper {
    }

    impl JsonMapper {

        pub fn default() -> JsonMapper {
            JsonMapper {
            }
        }

        pub fn to_json(&self, value: Box<dyn Serializable>) -> Option<Json> {
            let fields = value.get_serializable_fields();

            let mut json = Json::default();
            for field in fields {
                let field_name = field.field_name;
                let f = field.callback;
                let value = f();
                json.put(field_name, value);
            }

            return Some(json);
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::{*, JSON::Serializable, JSON::FieldDescriptor, JSON::{JsonMapper, Json}};

    struct SimpleObj {
        pub int_val: i32,
        pub str_val: String,
    }

    impl SimpleObj {
        fn get_int_val(&self,) -> String {
            self.int_val.to_string()
        }

        fn get_str_val(&self,) -> String {
            self.str_val.to_string()
        }
    }

    impl Serializable for SimpleObj {
        fn get_serializable_fields(&self) -> Vec<FieldDescriptor<String>> {
            let mut fields: Vec<FieldDescriptor<String>>  = Vec::new();
            fields.push(
                FieldDescriptor {
                    field_name: "int_val".to_string(),
                    callback: Box::new(|| self.get_int_val()),
                }
            );
            fields.push(
                FieldDescriptor {
                    field_name: "str_val".to_string(),
                    callback: Box::new(|| self.get_str_val()),
                }
            );
            fields
        }
    }

    #[test]
    fn test_simple_json() {
        let mapper = JsonMapper::default();
        let obj_to_test = SimpleObj {
            int_val: 10,
            str_val: "Hello World".to_string(),
        };
        let result = mapper.to_json(Box::new(obj_to_test));

        let expected_json = Json {
            map: HashMap::new()
        };

        assert_eq!(result, Some(expected_json));
    }
}
