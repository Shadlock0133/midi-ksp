use krpc_proto::{Procedure, Services};

mod class;
pub mod connection;
pub mod control;
pub mod vessel;

pub use connection::KrpcConnection;

pub fn print_procedure_signature(procedure: &Procedure) -> String {
    use std::fmt::Write;
    let mut res = String::new();

    let mut doc = procedure
        .documentation
        .as_deref()
        .unwrap_or_default()
        .to_string();
    doc = doc.replace("<doc>", "");
    doc = doc.replace("</doc>", "");
    doc = doc.replace("<summary>", "");
    doc = doc.replace("</summary>", "");
    doc = doc.replace("<returns>", "# Returns\n\n");
    doc = doc.replace("</returns>", "\n");
    let doc = doc.trim();
    for doc_line in doc.lines() {
        writeln!(res, "/// {}", doc_line).unwrap();
    }

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

pub fn dump_procs_sigs(services: &Services) -> String {
    use std::fmt::Write;
    let mut res = String::new();
    for service in &services.services {
        let service_name = service.name.as_deref().unwrap();
        writeln!(res, "mod {} {{", service_name).unwrap();
        for proc in &service.procedures {
            let text = print_procedure_signature(proc);
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
