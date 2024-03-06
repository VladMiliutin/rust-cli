// This implementation is very simple and can't handle nested objects/maps/vectors
// I'll try to create a better serde-json in separate project and later migrate this to new serde
pub mod json {

    use std::{collections::{HashMap}};
    use string_builder::Builder;

    const ARRAY_START: char = '[';
    const ARRAY_END: char = ']';
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
    pub enum JsonValue {
        JsonObject(Json),
        JsonString(String),
        JsonArray(Vec<JsonValue>),
    }

    impl PartialEq for JsonValue {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }

    #[derive(Debug)]
    pub struct Json {
        map: HashMap<String, JsonValue>,
        vec: Vec<JsonValue>,
        pub is_array: bool,
    }

    impl Json {

        pub fn from_map(map: HashMap<String, JsonValue>) -> Json {
            Json {
                map,
                vec: Vec::with_capacity(0),
                is_array: false,
            }
        }

        pub fn from_vec(vec: Vec<JsonValue>) -> Json {
            Json {
                map: HashMap::with_capacity(0),
                vec,
                is_array: true,
            }
        }

        pub fn default() -> Json {
            Json::from_map(HashMap::new())
        }

        pub fn get(&self, key: &str) -> Option<&JsonValue> {
           self.map.get(key)
        }

        pub fn put(&mut self, key: String, value: JsonValue) {
           self.map.insert(key, value);
        }

        pub fn to_string(&self) -> String {
            if self.is_array {
                return self.write_array(&self.vec);
            }

            self.write_self()
        }

        fn write_self(&self) -> String {
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
                self.write_to_sb(&mut sb, value);

                if iter < map_size {
                    sb.append(DELIMITER);
                }
            }

            sb.append(OBJECT_END);
            sb.string().unwrap()
        }

        fn write_object(&self, value: &Json) -> String {
            value.write_self()
        }

        fn write_array(&self, vec: &Vec<JsonValue>) -> String {
            let mut sb = Builder::default();
            sb.append(ARRAY_START);
            let vec_size = vec.len();
            let mut iter = 0;
            for value in vec {
                self.write_to_sb(&mut sb, &value);
                if iter < vec_size {
                    sb.append(DELIMITER);
                }
            }
            sb.append(ARRAY_END);
            sb.string().unwrap()
        }

        fn write_to_sb(&self, sb: &mut Builder, value: &JsonValue) {
            match value {
                JsonValue::JsonString(str) => {
                    sb.append(QUOTE);
                    sb.append(str.to_string());
                    sb.append(QUOTE);
                },
                JsonValue::JsonObject(obj) => sb.append(self.write_object(obj)),
                JsonValue::JsonArray(arr) => sb.append(self.write_array(arr)),
            }
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
                json.put(field_name, JsonValue::JsonString(value));
            }

            return Some(json);
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;
    use string_builder::ToBytes;

    use super::{*, json::*};

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

        let mut json_map: HashMap<String, JsonValue> = HashMap::new();
        json_map.insert("int_val".to_string(), JsonValue::JsonString("10".to_string()));
        json_map.insert("str_val".to_string(), JsonValue::JsonString("Hello World".to_string()));
        let expected_json = Json::from_map(json_map);

        assert_eq!(result, Some(expected_json));
    }

    #[test]
    fn test_simple_json_to_string() {
        let mut json_map: HashMap<String, JsonValue> = HashMap::new();
        json_map.insert("key1".to_string(), JsonValue::JsonString("value1".to_string()));
        json_map.insert("key2".to_string(), JsonValue::JsonString("value2".to_string()));
        let expected = "{\"key1\":\"value1\",\"key2\":\"value2\"}";

        let json = Json::from_map(json_map);
        let real = json.to_string();

        assert_eq!(real, expected);
        assert_ne!(json.to_string(), "{}");
    }
}
