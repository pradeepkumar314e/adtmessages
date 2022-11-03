mod parser;

use crate::parser::{Msg, Field, Segment, Schema, SchemaSegment, ParsedMsg, ParsedSegment, ParsedField};
use actix_web::{HttpServer, App, web, Responder};


//use core::num::dec2flt::parse;
use std::{fs, path::PathBuf, io};

//use actix_web::Responder;
use nom::{
    
    branch::alt,
    bytes::complete::{tag, take_while},
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    IResult,
};

use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;



const HL7V2_VERSIONS: &[&str] = &[
    "2.1", "2.2", "2.3", "2.3.1", "2.4", "2.5", "2.6.1", "2.7", "2.7.1", "2.8",
];



impl Msg {





    pub fn slen(&self) -> usize { self.segments.len() }





    pub fn msg_type(&self) -> Option<String> {
    
        if let Some(segment) = self.segments.first() {



            if let Some(field) = segment.fields.get(8) {
                // If 3 components, take last (msg structure)
                // else take first and second component
                if field.components.len() == 3 && !field.components.last().unwrap().is_empty()
                {
                    return Some(field.components.last().unwrap().to_string());


                } else {




                    let mut msg_type = "".to_owned();



                    if let Some(component) = field.components.first() {
                        // TODO: Check if it is valid type
                        msg_type.push_str(&component);
                        msg_type.push('_');
                    
                    
                    
                        if let Some(component) = field.components.get(1) {
                    
                    
                            msg_type.push_str(&component);
                            return Some(msg_type);
                    
                    
                        }
                    
                    
                    }
                }
            }
        }
       None

    }

    pub fn version(&self) -> Option<String> {


        if let Some(segment) = self.segments.first() {
            
            
            if let Some(field) = segment.fields.get(11) {
            
            
                if let Some(component) = field.components.first() {
            
            
            
                    if HL7V2_VERSIONS.contains(&component.as_str()) {
            
            
            
            
                        let mut version = "V".to_owned();
                        version.push_str(component.as_str().replace(".", "_").as_str());
                        return Some(version);
            
            
                    }
                }
            }
        }
        None
    }

}




fn is_not_cs_fs_or_line_end(i: char) -> bool { i != '^' && i != '|' && i != '\n' && i != '\r' }






fn parse_component(i: Span) -> IResult<Span, Span> { 
    
    
    take_while(is_not_cs_fs_or_line_end)(i) 



}








fn parse_field(i: Span) -> IResult<Span, Field> {



    separated_list0(    tag("^")   ,   parse_component)(i).map(|(i, components)| {
        (i, Field {

            components: components
                .iter()
                .map(|&s| {
                    s.fragment().to_string()
                    
                })
                .collect(),
        })
    })
}
















fn parse_segment(i: Span) -> IResult<Span, Segment> {
    separated_list0(tag("|"), parse_field)(i).map(|(i, fields)| {
        (i, Segment {
            fields: fields
                .into_iter()
                .map(|field| {
                       field
                    
                })
                .collect(),
        })
    })
}












pub fn parse_msg(i: Span) -> IResult<Span, Msg> {
    

    let separator = match i.chars().nth(3) {
        Some(c) => {
            if c != '|' {
                return Err(nom::Err::Error(ParseError::from_error_kind(
                    i,
                    ErrorKind::Char,
                )));
            }
            c
        },





        None => {


            return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                
                ErrorKind::Eof,
            )));




        },
    };



    separated_list0(alt((tag("\n"), tag("\r"))), parse_segment)(i).map(|(i, segments)| {
        (i, Msg {
            segments: segments
                .into_iter()
                .map(|segment| {
                    
                        segment
                    
                })
                .collect(),
            separator,
        })
    })
}




async fn status() -> impl Responder{
     //let input = "MSH|^~\\&|AccMgr|1|||20050110045504||ADT^A08|599102|P|2.2|||\nEVN|A01|20050110045502|||||\nPID|1||10006579^^^1^MRN^1||DUCK^DONALD^D||19241010|M||1|111 DUCK ST^^FOWL^CA^999990000^^M|1|8885551212|8885551212|1|2||40007716^^^AccMgr^VN^1|123121234|||||||||||NO\nNK1|1|DUCK^HUEY|SO|3583 DUCK RD^^FOWL^CA^999990000|8885552222||Y||||||||||||||\nPV1|1|I|PREOP^101^1^1^^^S|3|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|||01||||1|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|2|40007716^^^AccMgr^VN|4|||||||||||||||||||1||G|||20050110045253||||||\nGT1|1|8291|DUCK^DONALD^D||111^DUCK ST^^FOWL^CA^999990000|8885551212||19241010|M||1|123121234||||#Cartoon Ducks Inc|111^DUCK ST^^FOWL^CA^999990000|8885551212||PT|\nDG1|1|I9|71596^OSTEOARTHROS NOS-L/LEG ^I9|OSTEOARTHROS NOS-L/LEG ||A|\nINSURANCE|1^MEDICARE^3^MEDICARE^^^^^^^Cartoon Ducks Inc^19891001^^^4^DUCK*DONALD*D^1^19241010^111*DUCK ST**FOWL*CA*999990000^^^^^^^^^^^^^^^^^123121234A^^^^^^PT^M^111 DUCK ST**FOWL*CA*999990000^^^^^8291|1^^123121234^Cartoon Ducks Inc^^^123121234A^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^8885551212\nINSURANCE|2^NON-PRIMARY^9^MEDICAL MUTUAL CALIF.^PO BOX 94776**HOLLYWOOD*CA*441414776^^8003621279^PUBSUMB^^^Cartoon Ducks Inc^^^^7^DUCK*DONALD*D^1^19241010^111 DUCK ST**FOWL*CA*999990000^^^^^^^^^^^^^^^^^056269770^^^^^^PT^M^111*DUCK ST**FOWL*CA*999990000^^^^^8291|2^^123121234^Cartoon Ducks Inc^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^8885551212\nINSURANCE|3^SELF PAY^1^SELF PAY^^^^^^^^^^^5^^1";
     let inputt = "MSH|^~\\&|AccMgr|1|||20050110045504||ADT^A08|599102|P|2.2|||\nEVN|A01|20050110045502|||||\nPID|1||10006579^^^1^MRN^1||DUCK^DONALD^D||19241010|M||1|111 DUCK ST^^FOWL^CA^999990000^^M|1|8885551212|8885551212|1|2||40007716^^^AccMgr^VN^1|123121234|||||||||||NO\nNK1|1|DUCK^HUEY|SO|3583 DUCK RD^^FOWL^CA^999990000|8885552222||Y||||||||||||||\nPV1|1|I|PREOP^101^1^1^^^S|3|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|||01||||1|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|2|40007716^^^AccMgr^VN|4|||||||||||||||||||1||G|||20050110045253||||||\nGT1|1|8291|DUCK^DONALD^D||111^DUCK ST^^FOWL^CA^999990000|8885551212||19241010|M||1|123121234||||#Cartoon Ducks Inc|111^DUCK ST^^FOWL^CA^999990000|8885551212||PT|\nDG1|1|I9|71596^OSTEOARTHROS NOS-L/LEG ^I9|OSTEOARTHROS NOS-L/LEG ||A|\nIN1|1|MEDICARE|3|MEDICARE|||||||Cartoon Ducks Inc|19891001|||4|DUCK^DONALD^D|1|19241010|111^DUCK ST^^FOWL^CA^999990000|||||||||||||||||123121234A||||||PT|M|111 DUCK ST^^FOWL^CA^999990000|||||8291\nIN2|1||123121234|Cartoon Ducks Inc|||123121234A|||||||||||||||||||||||||||||||||||||||||||||||||||||||||8885551212\nIN1|2|NON-PRIMARY|9|MEDICAL MUTUAL CALIF.|PO BOX 94776^^HOLLYWOOD^CA^441414776||8003621279|PUBSUMB|||Cartoon Ducks Inc||||7|DUCK^DONALD^D|1|19241010|111 DUCK ST^^FOWL^CA^999990000|||||||||||||||||056269770||||||PT|M|111^DUCK ST^^FOWL^CA^999990000|||||8291\nIN2|2||123121234|Cartoon Ducks Inc||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||8885551212\nIN1|3|SELF PAY|1|SELF PAY|||||||||||5||1";
     let (_, msg) = parse_msg(Span::new(inputt)).unwrap();
     let version = msg.version().unwrap();
    
     let filename = format!("v{}.json",version.chars().skip(1).collect::<String>());
     let filepath = PathBuf::from(format!("schema/{}",filename));
     let data = fs::read_to_string(filepath).unwrap();
     //println!("{}",filename); 
    
     let mut schema: Schema = serde_json::from_str(data.as_str()).unwrap();
    // //println!("deserialized = {:?}", schema);
     // let sch = serde_json::to_string(&schema).unwrap();
    // println!("{}",sch); 
     let msgtype = msg.msg_type().unwrap();   
     let mut schema_msg = schema.messages.get(&msgtype).unwrap();
     let mut schema_segment_names = schema_msg.segments.iter().map(|x| x.name.clone() ).collect::<Vec<String>>();
     //let mut schema_segments  = schema_segment_names.iter().map(|x| schema.segments.get(x).unwrap().clone()).collect::<Vec<SchemaSegment>>();
     //println!("{}",msgtype);
     
    let mut parsed_msg = ParsedMsg::default();
    parsed_msg.msg_type = msgtype;
    parsed_msg.version = version;
    
    let mut parsed_insurance_segments = Vec::<ParsedSegment>::new();

    for (seg_i, seg) in msg.segments.iter().enumerate() {
    
        //     // println!("{}, {}, {}", seg_i, field_i, component);
        let segment_key = &seg.fields[0].components[0];
        //if seg_i >= schema_segments.len(){break;}
        
        let mut schema_segment = schema.segments.get(segment_key);
        if schema_segment.is_none() {
            println!("{}", segment_key);  
            if segment_key.as_str().starts_with("IN"){

                let insurance_seg = schema.segments.get("INSURANCE".to_string().as_str()).unwrap();
                
                if segment_key.as_str().starts_with("IN1"){
                    let _seg = ParsedSegment::default();
                    parsed_insurance_segments.push(_seg);
                    
                }

                let insurance_parsed_segment = parsed_insurance_segments.last_mut().unwrap();

                let in_number:usize = segment_key.as_str().chars().nth(2).unwrap().to_string().parse().unwrap();
                

                insurance_parsed_segment.name = insurance_seg.long_name.clone();
                
                
                let mut seg_iter = seg.fields.iter().skip(0);
                if seg_i != 0 {
                   seg_iter = seg.fields.iter().skip(1);

                }
                let mut parsed_field = ParsedField::default();
                parsed_field.name=segment_key.clone();
                parsed_field.value = None;
                parsed_field.components = vec![];
                
               for (field_i, field) in seg_iter.enumerate() {
                   
                   let insurance_seg_field = insurance_seg.fields[in_number-1].clone();
                   if field_i >= insurance_seg_field.components.len(){break;}
            
            
                    if insurance_seg_field.components.len() == 0 && field.components.len() > 0 {
                       //this should not happen,  ignoring this  not panicking
                

                     }else {
                
                          //println!("\t{}", schema_seg_field.name);
                          let field_value = field.components.iter().map(|x| x.clone()).collect::<Vec<String>>();
                          let field_value = field_value.join("^");
                          let schema_component_name = insurance_seg_field.components[field_i].name.clone();
                          if !field_value.is_empty() {
                             
                              parsed_field.components.push(Some((schema_component_name,field_value)));
                          }
                       }
               
                }
                insurance_parsed_segment.fields.push(Some(parsed_field));

            
            }
            continue;
         }

        println!("{}", segment_key);  
        let mut schema_segment = schema_segment.unwrap();
        let mut parsed_segment = ParsedSegment::default();

        
        //println!("{}", schema_segment.long_name);
        parsed_segment.name = schema_segment.long_name.clone();
        
        let mut seg_iter = seg.fields.iter().skip(0);
        if seg_i != 0 {
            seg_iter = seg.fields.iter().skip(1);

        }
        
        for (field_i, field) in seg_iter.enumerate() {
            if field_i >= schema_segment.fields.len(){break;}
            let mut schema_seg_field = schema_segment.fields[field_i].clone();
            
            
            if schema_seg_field.components.len() == 0 && field.components.len() > 0 {
                let field_value = field.components.iter().map(|x| x.clone()).collect::<Vec<String>>();
                let field_value = field_value.join("^");
                
                if !field_value.is_empty(){
                    //println!("\t{} ===> {}", schema_seg_field.name, field_value);
                    
                    //schema_seg_field.value = Some(field_value);
                    parsed_segment.fields.push(Some(ParsedField {
                        name:schema_seg_field.name,
                        value:Some(field_value),
                        components:vec![]
                    }));
                }
                

            }else {
                
                //println!("\t{}", schema_seg_field.name);
                let mut parsed_field = ParsedField::default();
                parsed_field.name=schema_seg_field.name;
                parsed_field.value = None;
                parsed_field.components = vec![];
                for (component_i, component) in field.components.iter().enumerate() {
                    if component_i >= schema_seg_field.components.len(){break;}
                    let mut field_component = schema_seg_field.components[component_i].clone();
                    if !component.is_empty() {
                        
                        //println!("\t\t{} ===> {}", field_component.name, component);
                        parsed_field.components.push(Some((field_component.name,component.to_string())));
                        //field_component.value = Some(component.clone());
                    }
                   //  println!("\t{}, {}, {}",segment.long_name, seg_field.name, field_component.name);
               }
               parsed_segment.fields.push(Some(parsed_field));
            }
            
        }
        parsed_msg.segments.push(Some(parsed_segment));
    }
    
    for insurance_seg in parsed_insurance_segments{
        parsed_msg.segments.push(Some(insurance_seg));
    }

    let jsoon = serde_json::to_string(&parsed_msg).unwrap();
    jsoon

}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("http://127.0.0.1:8080");
    HttpServer::new(|| {
       App::new()
          .route("/", web::get().to(status))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await

}

// fn main(){
//     status();
// }