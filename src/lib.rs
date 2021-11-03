use proc_macro::TokenStream;
use std::collections::HashMap;
#[proc_macro]
pub fn sonic_serde_macros(_item: TokenStream) -> TokenStream {
    let mut code = String::new();
    code.push_str(r#"use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub enum SonicSerdeObjectError {
    NotA(String),
}
"#);
    let mut obj_types: HashMap<String, String> = HashMap::new();
    obj_types.insert("String".to_string(), "String".to_string());
    obj_types.insert("Vec".to_string(), "Vec<SonicSerdeObject>".to_string());
    obj_types.insert("Map".to_string(), "BTreeMap<SonicSerdeObject, SonicSerdeObject>".to_string());
    obj_types.insert("U8".to_string(), "u8".to_string());
    obj_types.insert("Bool".to_string(), "bool".to_string());
    obj_types.insert("SystemTime".to_string(), "SystemTime".to_string());
    obj_types.insert("U32".to_string(), "u32".to_string());
    obj_types.insert("U64".to_string(), "u64".to_string());
    obj_types.insert("I8".to_string(), "i8".to_string());
    obj_types.insert("I32".to_string(), "i32".to_string());
    obj_types.insert("I64".to_string(), "i64".to_string());
    
    code.push_str("#[derive(Debug, Hash, PartialOrd, Ord, Serialize, Eq, PartialEq, Deserialize, Clone)]\npub enum SonicSerdeObject {\n");
    let obj_types_vec: Vec<String> = obj_types.keys().into_iter().map(|x| x.to_string()).collect();
    for obj_type in obj_types_vec.clone() {
        code.push_str(format!("    {}({}),\n", obj_type.clone(), obj_types.get(&obj_type).unwrap()).as_str());
    }
    code.push_str("}\n");
    code.push_str(r#"impl SonicSerdeObject {
    pub fn new_vec() -> Self {
        Self::Vec(Vec::new())
    }
    pub fn new_map() -> Self {
        Self::Map(BTreeMap::new())
    }
    pub fn new_map_with(key: impl Into<SonicSerdeObject>, value: impl Into<SonicSerdeObject>) -> Self {
        let mut x = Self::Map(BTreeMap::new());
        x.insert(key.into(), value.into());
        x
    }
    pub fn from_str(val_str: impl Into<String>) -> Self {
        Self::String(val_str.into())
    }
"#);
    for obj_type_outer in obj_types_vec.clone() {
        code.push_str(format!("    pub fn is_{}(&self) -> bool", obj_type_outer.to_lowercase()).as_str());
        code.push_str(" {\n        match self {\n");
        for obj_type_inner in obj_types_vec.clone() {
            let bool_string: String;
            if obj_type_inner == obj_type_outer {
                bool_string = "true".to_string();
            } else {
                bool_string = "false".to_string();
            }
            code.push_str(format!("            Self::{}(_x) => {},\n", obj_type_inner.clone(), bool_string.clone()).as_str());
        }
        code.push_str("        }\n    }\n");
    }
    code.push_str("    pub fn as_str(&self) -> Result<&str, SonicSerdeObjectError> {\n        match self {\n");
    for obj_type in obj_types_vec.clone() {
        //let output: String;
        if obj_type.as_str() == "String" {
            code.push_str("            Self::String(x) => Ok(x.as_str()),\n")
        } else {
            code.push_str(format!("            Self::{}(_x) => Err(SonicSerdeObjectError::NotA(\"str\".to_string())),\n", obj_type.clone()).as_str());
        }
    }
    code.push_str("        }\n    }\n");
    for obj_type_outer in obj_types_vec.clone() {
        //let first: String;
        //if obj_type_outer.as_str() == "SystemTime" {
        //    first = "system_time".to_string();
        //} else {
        //    first = obj_type_outer.clone();
        //}
        code.push_str(format!("    pub fn as_{}(&self) -> Result<{}, ", obj_type_outer.to_lowercase(), obj_types.get(&obj_type_outer).unwrap()).as_str());
        code.push_str("SonicSerdeObjectError> {\n        match self {\n");
        for obj_type_inner in obj_types_vec.clone() {
            //let bool_string: String;
            if obj_type_inner == obj_type_outer {
                
                let bool_string = "Ok(x.clone())".to_string();
                code.push_str(format!("            Self::{}(x) => {},\n", obj_type_inner.clone(), bool_string.clone()).as_str());
            } else {
                let bool_string = format!("Err(SonicSerdeObjectError::NotA(\"{}\".to_string()))", obj_type_outer.to_lowercase());
                code.push_str(format!("            Self::{}(_x) => {},\n", obj_type_inner.clone(), bool_string.clone()).as_str());
            }
        }
        code.push_str("        }\n    }\n");
    }
    code.push_str(r#"
    pub fn push(&mut self, val: impl Into<SonicSerdeObject>) {
        if self.is_vec() {
            let mut y = self.as_vec().unwrap();
            y.push(val.into());
            *self = Self::Vec(y);
        }
    }
    pub fn insert(&mut self, key: impl Into<SonicSerdeObject>, val: impl Into<SonicSerdeObject>) {
        if self.is_map() {
            let mut x = self.as_map().unwrap();
            x.insert(key.into(), val.into());
            *self = Self::Map(x);
        }
    }
}

impl AsMut<SonicSerdeObject> for SonicSerdeObject {
    fn as_mut(&mut self) -> &mut SonicSerdeObject {
        self
    }
}

impl AsRef<SonicSerdeObject> for SonicSerdeObject {
    fn as_ref(&self) -> &SonicSerdeObject {
        self
    }
}
impl From<&str> for SonicSerdeObject {
    fn from(string_val: &str) -> SonicSerdeObject {
        SonicSerdeObject::String(string_val.to_string())
    }
}

impl<T> From<Vec<T>> for SonicSerdeObject where SonicSerdeObject: std::convert::From<T> {
    fn from(vec_val: Vec<T>) -> SonicSerdeObject {
        let mut out = SonicSerdeObject::new_vec();
        for item in vec_val {
            let x: SonicSerdeObject = item.into();
            out.push(x);
        }
        out
    }
}

impl<K, V> From<HashMap<K, V>> for SonicSerdeObject where SonicSerdeObject: std::convert::From<K>, SonicSerdeObject: std::convert::From<V> {
    fn from(hashmap_val: HashMap<K, V>) -> SonicSerdeObject {
        let mut out: SonicSerdeObject = SonicSerdeObject::new_map();
        for item in hashmap_val.into_iter() {
            let a: SonicSerdeObject = item.0.into();
            let b: SonicSerdeObject = item.1.into();
            out.insert(a, b);
        }
        out
    }
}
"#);
    let mut vecless_object_types_vec = obj_types_vec.clone();
    vecless_object_types_vec.retain(|x| !(x.to_lowercase().starts_with("vec") || x.as_str() == "SystemTime"));
    for obj_type1 in vecless_object_types_vec.clone() {
        let obj_type = obj_types.get(&obj_type1).unwrap();
        code.push_str(format!("impl From<{}> for SonicSerdeObject ", obj_type.clone()).as_str());
        code.push_str("{\n    fn from(val: ");
        code.push_str(&obj_type);
        code.push_str(") -> SonicSerdeObject {\n        SonicSerdeObject::");
        code.push_str(&obj_type1);
        code.push_str("(val)\n    }\n}\n");
    }
    for obj_type1 in vecless_object_types_vec.clone() {
        let obj_type = obj_types.get(&obj_type1).unwrap();
        code.push_str(format!("impl From<&{}> for SonicSerdeObject ", obj_type.clone()).as_str());
        code.push_str("{\n    fn from(val: &");
        code.push_str(&obj_type);
        code.push_str(") -> SonicSerdeObject {\n        SonicSerdeObject::");
        code.push_str(&obj_type1);
        code.push_str("(val.clone())\n    }\n}\n");
    }
    //println!("{}", code);
    //"".parse().unwrap()
    code.as_str().parse().unwrap()
}