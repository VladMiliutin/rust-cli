// This implementation is very simple and can't handle nested objects/maps/vectors
// I'll try to create a better serde-json in separate project and later migrate this to new serde
pub mod JSON {

    use std::{collections::{HashMap}};
    use string_builder::Builder;

    const OBJECT_START: char = '{';
    const OBJECT_END: char = '}';
    const KEY_VALUE_SPLITTER: char = ':';
    const QUOTE: char = '"';
    const DELIMITER: char = ',';
    /**
     * This first iteration of implementation, maybe I'll use some MappingConfig with class
     * description and it's filed later. Who knows.
     */
    pub struct FieldDescriptor<'a> {
        pub field_name: String,
        pub callback: Box<dyn FnOnce() -> String + 'a>,
    }

    pub trait Serializable {
        fn get_serializable_fields(&self) -> Vec<FieldDescriptor>;
    }

    #[derive(Debug)]
    pub struct Json {
        map: HashMap<String, String>,
    }

    impl Json {

        pub fn from_map(map: HashMap<String, String>) -> Json {
            Json {
                map: map,
            }
        }

        pub fn default() -> Json {
            Json::from_map(HashMap::new())
        }

        pub fn get(&self, key: &str) -> Option<&String> {
           self.map.get(key)
        }

        pub fn put(&mut self, key: String, value: String) {
           self.map.insert(key, value);
        }

        pub fn to_string(&self) -> String {
            let mut sb = Builder::default();
            sb.append(OBJECT_START);
            let map_iter = self.map.iter();
            let map_size = self.map.len();
            let mut iter = 0;
            for (key, value) in map_iter {
                iter += 1;
                sb.append(QUOTE);
                sb.append(key.to_string());
                sb.append(QUOTE);
                sb.append(KEY_VALUE_SPLITTER);
                sb.append(QUOTE);
                sb.append(value.to_string());
                sb.append(QUOTE);

                if iter < map_size {
                    sb.append(DELIMITER);
                }
            }

            sb.append(OBJECT_END);
            sb.string().unwrap()
        }
    }

    impl PartialEq for Json {
        fn eq(&self, other: &Self) -> bool {
            self.map == other.map
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
        fn get_serializable_fields(&self) -> Vec<FieldDescriptor> {
            let mut fields: Vec<FieldDescriptor>  = Vec::new();
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

        let mut json_map: HashMap<String, String> = HashMap::new();
        json_map.insert("int_val".to_string(), "10".to_string());
        json_map.insert("str_val".to_string(), "Hello World".to_string());
        let expected_json = Json::from_map(json_map);

        assert_eq!(result, Some(expected_json));
    }

    #[test]
    fn test_simepl_json_to_string() {
        let mut json_map: HashMap<String, String> = HashMap::new();
        json_map.insert("key1".to_string(), "value1".to_string());
        json_map.insert("key2".to_string(), "value2".to_string());
        let json = Json::from_map(json_map);

        let expected = "{\"key1\":\"value1\",\"key2\":\"value2\"}";

        assert_eq!(json.to_string(), expected);
        assert_ne!(json.to_string(), "{}");
    }
}
