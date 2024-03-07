// This implementation is very simple and can't handle nested objects/maps/vectors
// I'll try to create a better serde-json in separate project and later migrate this to new serde
pub mod json {
    use std::collections::BTreeMap;

    use string_builder::Builder;

    const ARRAY_START: char = '[';
    const ARRAY_END: char = ']';
    const OBJECT_START: char = '{';
    const OBJECT_END: char = '}';
    const KEY_VALUE_SPLITTER: char = ':';
    const QUOTE: char = '"';
    const DELIMITER: char = ',';

    pub trait Serializable {
        fn serialize(&self) -> JsonValue;
    }

    #[derive(Debug)]
    pub enum JsonValue {
        JsonObject(Json),
        JsonString(String),
        JsonArray(Vec<JsonValue>),
    }

    impl PartialEq for JsonValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (JsonValue::JsonObject(o1), JsonValue::JsonObject(o2)) => o1 == o2,
                (JsonValue::JsonString(str1), JsonValue::JsonString(str2)) => str1 == str2,
                (JsonValue::JsonArray(vec1), JsonValue::JsonArray(vec2)) => vec1 == vec2,
                _ => false,
            }
        }
    }

    impl JsonValue {
        pub fn to_string(self) -> String {
            match self {
                JsonValue::JsonObject(obj) => obj.to_string(),
                JsonValue::JsonString(str) => format!("[{}]", str),
                JsonValue::JsonArray(vec) => Json::from_vec(vec).to_string(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Json {
        map: BTreeMap<String, JsonValue>,
        vec: Vec<JsonValue>,
        pub is_array: bool,
    }

    pub fn serialize_str(value: &str) -> JsonValue {
        JsonValue::JsonString(value.to_string())
    }

    pub fn serialize_string(value: String) -> JsonValue {
        return JsonValue::JsonString(value);
    }

    pub fn serialize_vec<T: Serializable>(value: &Vec<T>) -> JsonValue {
        let json_vec = value.into_iter().map(|v| v.serialize()).collect();
        JsonValue::JsonArray(json_vec)
    }

    impl Json {

        pub fn from_map(map: BTreeMap<String, JsonValue>) -> Json {
            Json {
                map,
                vec: Vec::with_capacity(0),
                is_array: false,
            }
        }

        pub fn from_vec(vec: Vec<JsonValue>) -> Json {
            Json {
                map: BTreeMap::new(),
                vec,
                is_array: true,
            }
        }

        pub fn default() -> Json {
            Json::from_map(BTreeMap::new())
        }

        pub fn nth(&self, index: usize) -> Option<&JsonValue> {
            if self.vec.get(index).is_some() {
                Some(self.vec.get(index).unwrap())
            } else {
                None
            }
        }

        pub fn push(&mut self, value: JsonValue) {
            self.vec.push(value)
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
                iter += 1;
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

        pub fn put_str(mut self, key: &str, value: String) -> Self {
            self.put(key.to_string(), serialize_string(value));
            self
        }

        pub fn put_vec<T: Serializable>(mut self, key: &str, value: &Vec<T>) -> Self {
            self.put(key.to_string(), serialize_vec(value));
            self
        }

        pub fn put_obj(mut self, key: &str, value: Json) -> Self {
            self.put(key.to_string(), JsonValue::JsonObject(value));
            self
        }
    }

    impl PartialEq for Json {
        fn eq(&self, other: &Self) -> bool {
            if self.is_array {
                return self.is_array == other.is_array && self.vec == self.vec;
            }

            self.map.keys().len() == other.map.keys().len() && self.map == other.map
        }
    }

}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    use json::*;

    struct SimpleObj {
        pub int_val: i32,
        pub str_val: String,
    }

    impl Serializable for SimpleObj {
        fn serialize(&self) -> JsonValue {
            let json_obj = Json::default()
                .put_str("int_val", self.int_val.to_string())
                .put_str("str_val", self.str_val.clone());

            JsonValue::JsonObject(json_obj)
        }
    }

    #[test]
    fn test_value_serialization() {
        let obj_to_test = SimpleObj {
            int_val: 10,
            str_val: "Hello World".to_string(),
        };

        let real = obj_to_test.serialize();

        let mut json_map: BTreeMap<String, JsonValue> = BTreeMap::new();
        json_map.insert("int_val".to_string(), JsonValue::JsonString("10".to_string()));
        json_map.insert("str_val".to_string(), JsonValue::JsonString("Hello World".to_string()));
        let expected_json = Json::from_map(json_map);

        assert_eq!(real, JsonValue::JsonObject(expected_json));
    }

    #[test]
    fn test_json_to_string() {
        let mut json_map: BTreeMap<String, JsonValue> = BTreeMap::new();
        json_map.insert("key1".to_string(), JsonValue::JsonString("value1".to_string()));
        json_map.insert("key2".to_string(), JsonValue::JsonString("value2".to_string()));
        let expected = "{\"key1\":\"value1\",\"key2\":\"value2\"}";

        let json = Json::from_map(json_map);
        let real = json.to_string();

        assert_eq!(real, expected);
        assert_ne!(json.to_string(), "{}");
    }
}
