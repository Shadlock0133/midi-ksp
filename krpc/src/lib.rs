use std::fmt::Write;

use krpc_proto::{Class, Enumeration, EnumerationValue, Procedure, Services};

mod class;
pub mod connection;
pub mod control;
pub mod vessel;

pub use connection::KrpcConnection;

fn clean_doc(doc: &Option<String>) -> String {
    let mut doc = doc.as_deref().unwrap_or_default().to_string();
    doc = doc.replace("<doc>", "").replace("</doc>", "");
    doc = doc.replace("<summary>", "").replace("</summary>", "");
    doc = doc
        .replace("<returns>", "# Returns\n\n")
        .replace("</returns>", "\n");
    doc = doc.replace("<see cref=\"M:", "").replace("\" />", "");
    let doc = doc.trim();

    let mut res = String::new();
    for doc_line in doc.lines() {
        writeln!(res, "/// {}", doc_line).unwrap();
    }
    res
}

pub fn print_class(class: &Class) -> String {
    let mut res = String::new();
    res += &clean_doc(&class.documentation);
    writeln!(res, "struct {};", class.name.as_ref().unwrap()).unwrap();
    res
}

pub fn print_enumeration_value(value: &EnumerationValue) -> String {
    let mut res = String::new();
    res += &clean_doc(&value.documentation);
    let name = value.name.as_deref().unwrap();
    let value = value.value.unwrap_or(0);
    writeln!(res, "{} = {},", name, value).unwrap();
    res
}

pub fn print_enumeration(enumeration: &Enumeration) -> String {
    let mut res = String::new();
    res += &clean_doc(&enumeration.documentation);
    writeln!(res, "enum {} {{", enumeration.name.as_ref().unwrap()).unwrap();
    for value in &enumeration.values {
        for line in print_enumeration_value(value).lines() {
            writeln!(res, "    {}", line).unwrap();
        }
    }
    writeln!(res, "}}").unwrap();
    res
}

pub fn print_procedure(procedure: &Procedure) -> String {
    let mut res = String::new();

    res += &clean_doc(&procedure.documentation);

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
        if matches!(procedure.return_is_nullable, Some(true)) {
            write!(res, " -> Option<{:?}>", ret).unwrap();
        } else {
            write!(res, " -> {:?}", ret).unwrap();
        }
    }
    if let Some(first) = procedure.game_scenes.first() {
        writeln!(res).unwrap();
        write!(res, "    where GameScene: ").unwrap();
        write!(res, "{:?}", first).unwrap();
        for scene in &procedure.game_scenes {
            write!(res, " + {:?}", scene).unwrap();
        }
    }
    writeln!(res, ";").unwrap();
    res
}

fn wrap_comments(text: String, line_width: usize) -> String {
    let mut res = String::new();
    for line in text.lines() {
        let is_comment = line.find("/// ");
        if let Some(pos) = is_comment {
            let (prefix, mut line) = line.split_at(pos + 4);
            // Correct line width for indent and slashes
            let line_width = line_width - prefix.len();
            while line.chars().count() > line_width {
                let last_space = line
                    .char_indices()
                    .take(line_width)
                    .fold(None, |acc, x| {
                        match x.1 {
                            ' ' => Some(x.0),
                            _ => acc,
                        }
                    });
                if let Some(i) = last_space {
                    let (comment, rest) = line.split_at(i);
                    // Skip space
                    line = &rest[1..];
                    writeln!(res, "{}{}", prefix, comment).unwrap();
                } else {
                    writeln!(res, "{}{}", prefix, line).unwrap();
                    line = "";
                }
            }
            writeln!(res, "{}{}", prefix, line).unwrap();
        } else {
            writeln!(res, "{}", line).unwrap();
        }
    }
    res
}

pub fn dump_services_info(services: &Services) -> String {
    let mut res = String::new();
    for service in &services.services {
        res += &clean_doc(&service.documentation);
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
    wrap_comments(res, 80)
}
