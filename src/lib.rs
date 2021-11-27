use proc_macro::TokenStream;

mod datatypes;
use datatypes::DataType;
#[proc_macro]
pub fn sonic_serde_macros(_item: TokenStream) -> TokenStream {
    let mut code = String::new();
    code.push_str(r#"use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::SystemTime;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub enum SonicSerdeObjectError {
    NotA(String),
}
"#);
    // Set up all the object types and their proprties 
    let obj_types: Vec<datatypes::DataType> = [
        DataType::new("String", "String"),
        DataType::new("Vec", "Vec<SonicSerdeObject>").exclude_from_froms(true),
        DataType::new("Map", "BTreeMap<SonicSerdeObject, SonicSerdeObject>").exclude_from_froms(true),
        DataType::new("Bool", "bool"),
        DataType::new("SystemTime", "SystemTime").can_ref(false),
        DataType::new("U8", "u8"),
        DataType::new("U16", "u16"),
        DataType::new("U32", "u32"),
        DataType::new("U64", "u64"),
        DataType::new("U128", "u128"),
        DataType::new("I8", "i8"),
        DataType::new("I16", "i16"),
        DataType::new("I32", "i32"),
        DataType::new("I64", "i64"),
        DataType::new("I128", "i128"),
        DataType::new("VecU8", "Vec<u8>").exclude_from_froms(true),
        DataType::new("USize", "usize").can_ref(false),
        DataType::new("Option", "Box<Option<SonicSerdeObject>>").output_as("Option<SonicSerdeObject>").how_to_output("*(x.clone())").how_to_convert("Box::new(newval)"),
        DataType::new("F32", "OrderedFloat<f32>").output_as("f32").how_to_output("x.clone().into_inner()").how_to_convert("OrderedFloat(newval)"),
        DataType::new("F64", "OrderedFloat<f64>").output_as("f64").how_to_output("x.clone().into_inner()").how_to_convert("OrderedFloat(newval)")
    ].to_vec();
    
    code.push_str("#[derive(Debug, Hash, PartialOrd, Ord, Serialize, Eq, PartialEq, Deserialize, Clone)]\npub enum SonicSerdeObject {\n");
    
    for obj_type in obj_types.clone() {
        code.push_str(format!("    {}({}),\n", obj_type.name, obj_type.stored_as).as_str());
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
    // Go through each object type and make a function for checking whether or not the object is of that type
    for obj_type_outer in obj_types.clone() {
        code.push_str(format!("    pub fn is_{}(&self) -> bool", obj_type_outer.name.to_lowercase()).as_str());
        code.push_str(" {\n        match self {\n");
        for obj_type_inner in obj_types.clone() {
            let bool_string: String;
            if obj_type_inner.name == obj_type_outer.name {
                bool_string = "true".to_string();
            } else {
                bool_string = "false".to_string();
            }
            code.push_str(format!("            Self::{}(_x) => {},\n", obj_type_inner.name, bool_string.clone()).as_str());
        }
        code.push_str("        }\n    }\n");
    }
    // Special case for outputting as &str
    code.push_str("    pub fn as_str(&self) -> Result<&str, SonicSerdeObjectError> {\n        match self {\n");
    for obj_type in obj_types.clone() {
        //let output: String;
        if obj_type.name.as_str() == "String" {
            code.push_str("            Self::String(x) => Ok(x.as_str()),\n")
        } else {
            code.push_str(format!("            Self::{}(_x) => Err(SonicSerdeObjectError::NotA(\"str\".to_string())),\n", obj_type.name).as_str());
        }
    }
    code.push_str("        }\n    }\n");
    // Make functions that take data out of the SonicSerdeObject wrappers
    for obj_type_outer in obj_types.clone() {
        code.push_str(format!("    pub fn as_{}(&self) -> Result<{}, ", obj_type_outer.name.to_lowercase(), obj_type_outer.output_as).as_str());
        code.push_str("SonicSerdeObjectError> {\n        match self {\n");
        for obj_type_inner in obj_types.clone() {
            let bool_string: String;
            if obj_type_inner.name == obj_type_outer.name {
                bool_string = format!("Ok({})", obj_type_inner.how_to_output);
                code.push_str(format!("            Self::{}(x) => {},\n", obj_type_inner.name, bool_string.clone()).as_str());
            } else {
                bool_string = format!("Err(SonicSerdeObjectError::NotA(\"{}\".to_string()))", obj_type_outer.name);
                code.push_str(format!("            Self::{}(_x) => {},\n", obj_type_inner.name, bool_string.clone()).as_str());
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

impl From<Vec<u8>> for SonicSerdeObject {
    fn from(vec_val: Vec<u8>) -> SonicSerdeObject {
        SonicSerdeObject::VecU8(vec_val)
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
    // Make From implemenations
    let mut included_object_types = obj_types.clone();
    included_object_types.retain(|x| !x.exclude_from_froms);
    for obj_type in included_object_types.clone() {
        if obj_type.name == "U8" {
            continue;
        }
        code.push_str(format!("impl From<{}> for SonicSerdeObject ", obj_type.output_as).as_str());
        code.push_str("{\n    fn from(val: ");
        code.push_str(&obj_type.output_as);
        code.push_str(") -> SonicSerdeObject {\n        let newval = val;\n        SonicSerdeObject::");
        code.push_str(format!("{}(", &obj_type.name).as_str());
        code.push_str(&obj_type.how_to_convert);
        //code.push_str(&obj_type.name);
        code.push_str(")\n    }\n}\n");
    }
    
    for obj_type in included_object_types.clone() {
        if obj_type.name == "U8" {
            continue;
        }
        code.push_str(format!("impl From<Vec<{}>> for SonicSerdeObject ", obj_type.output_as).as_str());
        code.push_str("{\n    fn from(val: ");
        code.push_str(format!("Vec<{}>) -> SonicSerdeObject ", &obj_type.output_as).as_str());
        code.push_str(r#"{
        let mut out = SonicSerdeObject::new_vec();
        for item in val {
            let x: SonicSerdeObject = item.into();
            out.push(x);
        }
        out
    }
}
"#);
    }
        /*
        code.push_str(") -> SonicSerdeObject {\n        let newval = val;\n        SonicSerdeObject::");
        code.push_str(format!("{}(", &obj_type.name).as_str());
        code.push_str(&obj_type.how_to_convert);
        //code.push_str(&obj_type.name);
        code.push_str(")\n    }\n}\n");
        */

    included_object_types.retain(|x| x.can_ref);
    for obj_type in included_object_types.clone() {
        code.push_str(format!("impl From<&{}> for SonicSerdeObject ", obj_type.output_as).as_str());
        code.push_str("{\n    fn from(val: &");
        code.push_str(&obj_type.output_as);
        code.push_str(") -> SonicSerdeObject {\n        let newval = val.to_owned();\n        SonicSerdeObject::");
        code.push_str(format!("{}(", &obj_type.name).as_str());
        code.push_str(&obj_type.how_to_convert);
        code.push_str(")\n    }\n}\n");
    }
    
    for obj_type in included_object_types.clone() {
        code.push_str(format!("impl From<Vec<&{}>> for SonicSerdeObject ", obj_type.output_as).as_str());
        code.push_str("{\n    fn from(val: ");
        code.push_str(format!("Vec<&{}>) -> SonicSerdeObject ", &obj_type.output_as).as_str());
        code.push_str(r#"{
        let mut out = SonicSerdeObject::new_vec();
        for item in val {
            let x: SonicSerdeObject = item.into();
            out.push(x);
        }
        out
    }
}
"#);
    }
    //println!("{}", code);
    //"".parse().unwrap()
    code.as_str().parse().unwrap()
}
