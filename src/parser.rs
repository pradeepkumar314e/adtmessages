

use std::collections::HashMap;
use serde::{Deserialize, Serialize};





#[derive(Debug, PartialEq, Default)]
pub struct Msg {
    pub segments:  Vec<Segment>,
    pub separator: char,
}


#[derive(Debug, PartialEq, Default)]
pub struct Segment {
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Field {
    pub components: Vec<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    pub version:String,
    pub messages:HashMap<String, SchemaMessage>,
    pub segments:HashMap<String, SchemaSegment>
} 

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaMessage{
    pub name:String,
    pub desc:String,
    pub segments:Vec<SchemaMsgSegment>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaMsgSegment{
    pub name:String,
    pub usage:bool,
    pub min:u64,
    pub max:u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaSegment {
    pub name:String,
    pub long_name:String,
    pub fields:Vec<SchemaSegField>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaSegField{
    pub name:String,
    pub data_type:Option<String>,
    pub usage:bool,
    pub min:u64,
    pub max:u64,
    pub length:Option<u64>,
    pub components:Vec<SchemaSegFieldComponent>,
    
}

#[derive(Debug,Clone , Deserialize, Serialize)]
pub struct SchemaSegFieldComponent{
    pub name:String,
    pub data_type:Option<String>,
    pub usage:bool,
    pub length:Option<u64>,
    
}

#[derive(Debug,Clone , Deserialize, Serialize, Default)]
pub struct ParsedMsg {
    pub msg_type:String,
    pub version:String,
    pub segments:Vec<Option<ParsedSegment>>
}

#[derive(Debug,Clone , Deserialize, Serialize, Default)]
pub struct ParsedSegment{
    pub name:String,
    pub fields:Vec<Option<ParsedField>>
}

#[derive(Debug,Clone , Deserialize, Serialize, Default)]
pub struct ParsedField{
    pub name:String,
    pub value:Option<String>,
    pub components:Vec<Option<(String, String)>>
}

#[derive(Debug,Clone , Deserialize, Serialize, Default)]
pub struct ParsedComponent{
    pub name:String,
    pub value:String
}