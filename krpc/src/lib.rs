use std::fmt::Write;

use krpc_proto::{Class, Enumeration, Procedure, Services};

mod class;
pub mod connection;
pub mod control;
pub mod vessel;

pub use connection::KrpcConnection;

fn clean_doc(doc: &str) -> String {
    let mut doc = doc.to_string();
    doc = doc.replace("<doc>", "");
    doc = doc.replace("</doc>", "");
    doc = doc.replace("<summary>", "");
    doc = doc.replace("</summary>", "");
    doc = doc.replace("<returns>", "# Returns\n\n");
    doc = doc.replace("</returns>", "\n");
    let doc = doc.trim();

    let mut res = String::new();
    for doc_line in doc.lines() {
        writeln!(res, "/// {}", doc_line).unwrap();
    }
    res
}

pub fn print_class(class: &Class) -> String {
    let mut res = String::new();
    let doc = class.documentation.as_deref().unwrap_or_default();
    res += &clean_doc(doc);
    writeln!(res, "struct {};", class.name.as_ref().unwrap()).unwrap();
    res
}

pub fn print_enumeration(enumeration: &Enumeration) -> String {
    let mut res = String::new();
    let doc = enumeration.documentation.as_deref().unwrap_or_default();
    res += &clean_doc(doc);
    writeln!(res, "enum {} {{", enumeration.name.as_ref().unwrap()).unwrap();
    for value in &enumeration.values {
        let doc = value.documentation.as_deref().unwrap_or_default();
        for doc_line in clean_doc(doc).lines() {
            writeln!(res, "    {}", doc_line).unwrap();
        }
        let name = value.name.as_deref().unwrap();
        let value = value.value.unwrap_or(0);
        writeln!(res, "    {} = {},", name, value).unwrap();
    }
    writeln!(res, "}}").unwrap();
    res
}

pub fn print_procedure(procedure: &Procedure) -> String {
    let mut res = String::new();

    let doc = procedure.documentation.as_deref().unwrap_or_default();
    res += &clean_doc(doc);

    write!(res, "fn {}(", procedure.name.as_ref().unwrap()).unwrap();
    if let Some(param) = &procedure.parameters.first() {
        let name = match param.name.as_deref().unwrap().trim() {
            "type" => "r#type",
            n => n,
        };
        write!(
            res,
            "{}: {:?}",
            name,
            param.r#type.as_ref().unwrap().code.as_ref().unwrap()
        )
        .unwrap();
        for param in &procedure.parameters[1..] {
            let name = match param.name.as_deref().unwrap().trim() {
                "type" => "r#type",
                n => n,
            };
            write!(
                res,
                ", {}: {:?}",
                name,
                param.r#type.as_ref().unwrap().code.as_ref().unwrap()
            )
            .unwrap();
        }
    }
    write!(res, ")").unwrap();

    if let Some(ret) =
        procedure.return_type.as_ref().and_then(|x| x.code.as_ref())
    {
        write!(res, " -> {:?}", ret).unwrap();
    }
    writeln!(res, ";").unwrap();
    res
}

pub fn dump_services_info(services: &Services) -> String {
    let mut res = String::new();
    for service in &services.services {
        let doc = service.documentation.as_deref().unwrap_or_default();
        res += &clean_doc(doc);
        let service_name = service.name.as_deref().unwrap();
        writeln!(res, "mod {} {{", service_name).unwrap();
        for proc in &service.classes {
            let text = print_class(proc);
            for line in text.lines() {
                writeln!(res, "    {}", line).unwrap();
            }
            writeln!(res).unwrap();
        }
        for proc in &service.enumerations {
            let text = print_enumeration(proc);
            for line in text.lines() {
                writeln!(res, "    {}", line).unwrap();
            }
            writeln!(res).unwrap();
        }
        for proc in &service.procedures {
            let text = print_procedure(proc);
            for line in text.lines() {
                writeln!(res, "    {}", line).unwrap();
            }
            writeln!(res).unwrap();
        }
        writeln!(res, "}}").unwrap();
        writeln!(res).unwrap();
    }
    res
}
