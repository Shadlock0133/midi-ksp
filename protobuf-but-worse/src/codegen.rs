use std::collections::HashMap;

use heck::{CamelCase, SnakeCase};
use quote::quote;

use protobuf_parser::{
    Enumeration, FieldType, FileDescriptor, Message, Rule, Syntax,
};
use syn::{Expr, Ident, LitInt, Path, Type};

#[derive(Debug)]
enum TypeInfo {
    Message,
    Enum,
}

// Scoped map of type information
// Codegen needs to know in certain place to know if a type is Message or Enum,
// and in other place if type is in local or outer scope
#[derive(Default, Debug)]
struct TypeInfoMap<'a> {
    prev: Option<&'a TypeInfoMap<'a>>,
    current: HashMap<String, TypeInfo>,
}

impl<'a> TypeInfoMap<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn wrap(prev: &'a TypeInfoMap) -> Self {
        Self {
            prev: Some(prev),
            ..Self::default()
        }
    }

    fn insert(&mut self, key: &str, info: TypeInfo) {
        self.current.insert(key.to_owned(), info);
    }

    fn get_current(&self, key: &str) -> Option<&TypeInfo> {
        self.current.get(key)
    }

    fn get_prev(&self, key: &str) -> Option<&TypeInfo> {
        self.prev.map(|prev| prev.get(key)).flatten()
    }

    fn get(&self, key: &str) -> Option<&TypeInfo> {
        self.get_current(key).or_else(|| self.get_prev(key))
    }

    fn populate(&mut self, messages: &[Message], enums: &[Enumeration]) {
        for message in messages {
            self.insert(&message.name, TypeInfo::Message);
        }
        for e in enums {
            self.insert(&e.name, TypeInfo::Enum);
        }
    }
}

pub fn gen_proto(proto: &FileDescriptor) -> syn::Result<String> {
    assert!(matches!(proto.syntax, Syntax::Proto3));

    let mut type_info = TypeInfoMap::new();
    type_info.populate(&proto.messages, &proto.enums);

    let mut file = quote! {};

    let import = quote! { use protobuf_but_worse::encoding::*; };
    file = quote! { #file #import };

    for message in &proto.messages {
        let code = gen_message(&type_info, message)?;
        file = quote! { #file #code };
    }

    for e in &proto.enums {
        let code = gen_enum(e)?;
        file = quote! { #file #code };
    }
    Ok(file.to_string())
}

fn gen_message(
    type_info: &TypeInfoMap,
    message: &Message,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut type_info = TypeInfoMap::wrap(type_info);
    type_info.populate(&message.messages, &message.enums);

    let struct_name: Ident = syn::parse_str(&message.name.to_camel_case())?;
    let module_name_str = message.name.to_snake_case();
    let module_name_str = escape_rust_keyword(&module_name_str);
    let module_name: Ident = syn::parse_str(&module_name_str)?;

    // TODO: deduplicate all fields loops
    let mut fields = quote! {};
    for field in &message.fields {
        let field_name: Ident =
            syn::parse_str(escape_rust_keyword(&field.name))?;
        let field_type = match field.rule {
            Rule::Required => {
                to_rust_type(&field.typ, &module_name_str, &type_info)
            }
            Rule::Repeated => {
                format!(
                    "Vec<{}>",
                    to_rust_type(&field.typ, &module_name_str, &type_info)
                )
            }
            Rule::Optional => {
                format!(
                    "Option<{}>",
                    to_rust_type(&field.typ, &module_name_str, &type_info)
                )
            }
        };
        let field_type: Type = syn::parse_str(&field_type)?;
        fields = quote! { #fields pub #field_name: #field_type, };
    }
    let main_struct = quote! {
        #[derive(Clone, PartialEq, Debug)]
        pub struct #struct_name { #fields }
    };

    let mut sizes = quote! {};
    for field in &message.fields {
        let field_name: Ident =
            syn::parse_str(escape_rust_keyword(&field.name))?;
        match field.rule {
            Rule::Optional => {
                let encoded_field: Expr =
                    syn::parse_str(&encoding_field(&field.typ, "*x"))?;
                sizes = quote! {
                    #sizes
                    size += self.#field_name
                        .as_ref()
                        .map(|x| 1 + (#encoded_field).size())
                        .unwrap_or(0);
                };
            }
            Rule::Repeated => {
                sizes = quote! {
                    #sizes
                    size += 1 + self.#field_name.size();
                }
            }
            Rule::Required => todo!(),
        }
    }
    let mut encoding_code = quote! {};
    for field in &message.fields {
        let number = field.number as u8;
        let wire_type = to_wire_type(&field.typ, &type_info);
        let field_name_str = escape_rust_keyword(&field.name);
        let field_name: Ident = syn::parse_str(&field_name_str)?;
        match field.rule {
            Rule::Optional => {
                let field_name: Ident =
                    syn::parse_str(escape_rust_keyword(&field.name))?;
                let encoded_field: Expr =
                    syn::parse_str(&encoding_field(&field.typ, "*x"))?;
                encoding_code = quote! {
                    #encoding_code
                    self.#field_name
                        .as_ref()
                        .map(|x| encode_field(&mut w, #number, #wire_type, &#encoded_field))
                        .transpose()?;
                };
            }
            Rule::Repeated => {
                // let encoded_field: Expr = syn::parse_str(&encoding_field(
                //     &field.typ,
                //     &("self.".to_string() + field_name_str),
                // ))?;
                encoding_code = quote! {
                    #encoding_code
                    for i in &self.#field_name {
                        let buf = i.encode_to_vec()?;
                        encode_field(&mut w, #number, #wire_type, &buf)?;
                    }
                };
            }
            _ => {
                let encoded_field: Expr = syn::parse_str(&encoding_field(
                    &field.typ,
                    &("self.".to_string() + field_name_str),
                ))?;
                encoding_code = quote! {
                    #encoding_code
                    encode_field(&mut w, #number, #wire_type, &#encoded_field)?;
                };
            }
        }
    }
    let encode_impl = quote! {
        impl Encode for #struct_name {
            fn size(&self) -> u32 {
                let mut size = 0;
                #sizes
                size
            }

            fn encode<W: std::io::Write>(&self, mut w: W) -> Result<(), EncodingError> {
                #encoding_code
                Ok(())
            }
        }
    };

    let mut decode_init_fields = quote! {};
    let mut decode_fields = quote! {};
    let mut decode_match = quote! {};
    for field in &message.fields {
        let field_name: Ident =
            syn::parse_str(escape_rust_keyword(&field.name))?;
        let number = field.number as u8;
        let wire_type = to_wire_type(&field.typ, &type_info);
        let encoding_type: Path = syn::parse_str(&encoding_type(
            &field.typ,
            module_name_str,
            &type_info,
        ))?;
        match field.rule {
            Rule::Repeated => {
                decode_init_fields = quote! {
                    #decode_init_fields
                    let mut #field_name = vec![];
                }
            }
            _ => {
                decode_init_fields = quote! {
                    #decode_init_fields
                    let mut #field_name = None;
                }
            }
        }
        match field.rule {
            Rule::Required => {
                decode_fields = quote! {
                    #decode_fields
                    #field_name: #field_name.ok_or(EncodingError::MissingField(number))?,
                }
            }
            _ => {
                decode_fields = quote! {
                    #decode_fields
                    #field_name: #field_name,
                }
            }
        }
        match (field.rule, uses_wrapper(&field.typ)) {
            (Rule::Repeated, false) => {
                decode_match = quote! {
                    #decode_match
                    #number => if __wire_type == #wire_type {
                        #field_name.push(<#encoding_type>::decode_as_field(&mut r)?);
                    } else if __wire_type == 2 {
                        #field_name.append(
                            &mut decode_packed::<_, #encoding_type>(&mut r)?);
                    } else {
                        return Err(EncodingError::WrongWireType(
                            stringify!(#struct_name.#field_name), __wire_type))
                    }
                }
            }
            (Rule::Repeated, true) => {
                decode_match = quote! {
                    #decode_match
                    #number => if __wire_type == #wire_type {
                        #field_name.push(<#encoding_type>::decode_as_field(&mut r)?.0);
                    } else if __wire_type == 2 {
                        #field_name.extend(
                            decode_packed::<_, #encoding_type>(&mut r)?
                                .into_iter()
                                .map(|x| x.0)
                        );
                    } else {
                        return Err(EncodingError::WrongWireType(
                            stringify!(#struct_name.#field_name), __wire_type))
                    }
                }
            }
            (_, false) => {
                decode_match = quote! {
                    #decode_match
                    #number => if __wire_type == #wire_type {
                        #field_name = Some(<#encoding_type>::decode_as_field(&mut r)?);
                    } else {
                        return Err(EncodingError::WrongWireType(
                            stringify!(#struct_name.#field_name), __wire_type))
                    }
                }
            }
            (_, true) => {
                decode_match = quote! {
                    #decode_match
                    #number => if __wire_type == #wire_type {
                        #field_name = Some(<#encoding_type>::decode_as_field(&mut r)?.0);
                    } else {
                        return Err(EncodingError::WrongWireType(
                            stringify!(#struct_name.#field_name), __wire_type))
                    }
                }
            }
        }
    }
    let decode_impl = quote! {
        impl Decode for #struct_name {
            fn decode<R: std::io::Read>(mut r: R) -> Result<Self, EncodingError> {
                #decode_init_fields
                loop {
                    let mut __key = 0u8;
                    if r.read(std::slice::from_mut(&mut __key))? == 0 {
                        return Ok(Self {
                            #decode_fields
                        });
                    }
                    let (__number, __wire_type) = (__key >> 3, __key & 0x7);
                    match __number {
                        #decode_match
                        _ => (),
                    }
                }
            }
            fn decode_as_field(r: &mut dyn std::io::Read) -> Result<Self, EncodingError> {
                Self::decode_with_len(r)
            }
        }
    };

    let sub_messages: proc_macro2::TokenStream = message
        .messages
        .iter()
        .map(|m| gen_message(&type_info, m))
        .collect::<Result<_, _>>()?;
    let sub_enums: proc_macro2::TokenStream = message
        .enums
        .iter()
        .map(|e| gen_enum(e))
        .collect::<Result<_, _>>()?;

    // subtypes
    let emit_mod = !message.messages.is_empty() || !message.enums.is_empty();
    let sub_mod = if emit_mod {
        quote! {
            pub mod #module_name {
                #[allow(unused_imports)]
                use super::*;
                #sub_messages
                #sub_enums
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #main_struct
        #encode_impl
        #decode_impl
        #sub_mod
    })
}

fn gen_enum(e: &Enumeration) -> syn::Result<proc_macro2::TokenStream> {
    let name: Ident = syn::parse_str(&e.name.to_camel_case())?;
    let mut variants = quote! {};
    for f in e.values.iter() {
        let name: Ident = syn::parse_str(&f.name.to_camel_case())?;
        let number: LitInt = syn::parse_str(&f.number.to_string())?;
        variants = quote! { #variants #name = #number, };
    }
    let mut match_variants = quote! {};
    for f in e.values.iter() {
        let name: Ident = syn::parse_str(&f.name.to_camel_case())?;
        let number: LitInt = syn::parse_str(&f.number.to_string())?;
        match_variants = quote! { #match_variants #number => Ok(Self::#name), };
    }
    Ok(quote! {
        #[repr(u32)]
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum #name {
            #variants
        }
        impl Encode for #name {
            fn size(&self) -> u32 {
                Varint(*self as u32).size()
            }

            fn encode<W: std::io::Write>(&self, w: W) -> Result<(), EncodingError> {
                Varint(*self as u32).encode(w)
            }
        }
        impl Decode for #name {
            fn decode<R: std::io::Read>(r: R) -> Result<Self, EncodingError> {
                match <Varint<u32>>::decode(r)?.0 {
                    #match_variants
                    e => Err(EncodingError::InvalidEnumValue(stringify!(#name), e)),
                }
            }
        }
    })
}

fn encoding_field(typ: &FieldType, field_name: &str) -> String {
    match typ {
        FieldType::Int32 => format!("Varint({})", field_name),
        FieldType::Int64 => format!("Varint({})", field_name),
        FieldType::Uint32 => format!("Varint({})", field_name),
        FieldType::Uint64 => format!("Varint({})", field_name),
        FieldType::Sint32 => format!("SVarint({})", field_name),
        FieldType::Sint64 => format!("SVarint({})", field_name),
        FieldType::Fixed32 => format!("Fixed({})", field_name),
        FieldType::Fixed64 => format!("Fixed({})", field_name),
        FieldType::Sfixed32 => format!("Fixed({})", field_name),
        FieldType::Sfixed64 => format!("Fixed({})", field_name),
        _ => field_name.to_string(),
    }
}

fn encoding_type(
    typ: &FieldType,
    parent: &str,
    type_info: &TypeInfoMap,
) -> String {
    match typ {
        FieldType::Int32 => "Varint<i32>".to_string(),
        FieldType::Int64 => "Varint<i64>".to_string(),
        FieldType::Uint32 => "Varint<u32>".to_string(),
        FieldType::Uint64 => "Varint<u64>".to_string(),
        FieldType::Sint32 => "SVarint<i32>".to_string(),
        FieldType::Sint64 => "SVarint<i64>".to_string(),
        FieldType::Fixed32 => "Fixed<u32>".to_string(),
        FieldType::Fixed64 => "Fixed<u64>".to_string(),
        FieldType::Sfixed32 => "Fixed<i32>".to_string(),
        FieldType::Sfixed64 => "Fixed<i64>".to_string(),
        FieldType::Double => "f64".to_string(),
        FieldType::Float => "f32".to_string(),
        FieldType::Bool => "bool".to_string(),
        FieldType::String => "String".to_string(),
        FieldType::Bytes => "Vec<u8>".to_string(),
        FieldType::MessageOrEnum(s) => {
            if type_info.get_current(s).is_some() {
                format!("{}::{}", parent, s)
            } else {
                s.to_string()
            }
        }
        FieldType::Group(_) => todo!("group"),
        FieldType::Map(_) => todo!("map"),
    }
}

fn uses_wrapper(typ: &FieldType) -> bool {
    match typ {
        FieldType::Int32
        | FieldType::Int64
        | FieldType::Uint32
        | FieldType::Uint64
        | FieldType::Sint32
        | FieldType::Sint64
        | FieldType::Fixed32
        | FieldType::Fixed64
        | FieldType::Sfixed32
        | FieldType::Sfixed64 => true,
        _ => false,
    }
}

fn to_rust_type(
    typ: &FieldType,
    parent: &str,
    type_info: &TypeInfoMap,
) -> String {
    match typ {
        FieldType::Double => "f64".to_string(),
        FieldType::Float => "f32".to_string(),
        FieldType::Int32 => "i32".to_string(),
        FieldType::Int64 => "i64".to_string(),
        FieldType::Uint32 => "u32".to_string(),
        FieldType::Uint64 => "u64".to_string(),
        FieldType::Sint32 => "i32".to_string(),
        FieldType::Sint64 => "i64".to_string(),
        FieldType::Fixed32 => "u32".to_string(),
        FieldType::Fixed64 => "u64".to_string(),
        FieldType::Sfixed32 => "i32".to_string(),
        FieldType::Sfixed64 => "i64".to_string(),
        FieldType::Bool => "bool".to_string(),
        FieldType::String => "String".to_string(),
        FieldType::Bytes => "Vec<u8>".to_string(),
        FieldType::MessageOrEnum(s) => {
            if type_info.get_current(s).is_some() {
                format!("{}::{}", parent, s)
            } else {
                s.to_string()
            }
        }
        FieldType::Group(_) => todo!("group"),
        FieldType::Map(_) => todo!("map"),
    }
}

fn to_wire_type(typ: &FieldType, type_info: &TypeInfoMap) -> u8 {
    match typ {
        FieldType::Int32
        | FieldType::Int64
        | FieldType::Uint32
        | FieldType::Uint64
        | FieldType::Sint32
        | FieldType::Sint64
        | FieldType::Bool => 0,
        FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => 1,
        FieldType::String | FieldType::Bytes => 2,
        FieldType::MessageOrEnum(s) => {
            match type_info
                .get(s)
                .expect(&format!("Missing type info on {}", s))
            {
                TypeInfo::Enum => 0,
                TypeInfo::Message => 2,
            }
        }
        FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => 5,
        FieldType::Group(_) => todo!("group wire type"),
        FieldType::Map(_) => todo!("map wire type"),
    }
}

const RUST_KEYWORDS: &[(&str, &str)] = &[
    ("as", "r#as"),
    ("break", "r#break"),
    ("const", "r#const"),
    ("continue", "r#continue"),
    ("crate", "r#crate"),
    ("else", "r#else"),
    ("enum", "r#enum"),
    ("extern", "r#extern"),
    ("false", "r#false"),
    ("fn", "r#fn"),
    ("for", "r#for"),
    ("if", "r#if"),
    ("impl", "r#impl"),
    ("in", "r#in"),
    ("let", "r#let"),
    ("loop", "r#loop"),
    ("match", "r#match"),
    ("mod", "r#mod"),
    ("move", "r#move"),
    ("mut", "r#mut"),
    ("pub", "r#pub"),
    ("ref", "r#ref"),
    ("return", "r#return"),
    ("self", "r#self"),
    ("Self", "r#Self"),
    ("static", "r#static"),
    ("struct", "r#struct"),
    ("super", "r#super"),
    ("trait", "r#trait"),
    ("true", "r#true"),
    ("type", "r#type"),
    ("unsafe", "r#unsafe"),
    ("use", "r#use"),
    ("where", "r#where"),
    ("while", "r#while"),
    ("async", "r#async"),
    ("await", "r#await"),
    ("dyn", "r#dyn"),
    ("abstract", "r#abstract"),
    ("become", "r#become"),
    ("box", "r#box"),
    ("do", "r#do"),
    ("final", "r#final"),
    ("macro", "r#macro"),
    ("override", "r#override"),
    ("priv", "r#priv"),
    ("typeof", "r#typeof"),
    ("unsized", "r#unsized"),
    ("virtual", "r#virtual"),
    ("yield", "r#yield"),
    ("try", "r#try"),
];

fn escape_rust_keyword(s: &str) -> &str {
    RUST_KEYWORDS
        .iter()
        .find(|x| x.0 == s)
        .map(|x| x.1)
        .unwrap_or(s)
}
