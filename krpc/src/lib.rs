use std::{collections::HashMap, fmt::Write};

use krpc_proto::{
    r#type::TypeCode, Class, Enumeration, EnumerationValue, Error, Procedure,
    Services, Type,
};

mod class;
pub mod connection;
pub mod control;
pub mod vessel;

pub use connection::KrpcConnection;
use protobuf_but_worse::encoding::EncodingResult;

type CallResult<T = ()> = EncodingResult<Result<T, Error>>;

fn clean_doc(doc: &Option<String>) -> String {
    let mut doc = doc.as_deref().unwrap_or_default().to_string();
    doc = doc.replace("<doc>", "").replace("</doc>", "");
    doc = doc.replace("<summary>", "").replace("</summary>", "");
    doc = doc
        .replace("<returns>", "# Returns\n\n")
        .replace("</returns>", "\n");
    doc = doc
        .replace("<remarks>", "# Remarks\n")
        .replace("</remarks>", "\n");
    doc = doc
        .replace("<item><description>", "- ")
        .replace("</description></item>", "\n");
    doc = doc
        .replace("<list type=\"bullet\">", "")
        .replace("</list>", "");
    doc = doc.replace("<see cref=\"", "").replace("\" />", "");
    doc = doc.replace("<c>", "`").replace("</c>", "`");
    let doc = doc.trim();

    let mut res = String::new();
    for doc_line in doc.lines() {
        writeln!(res, "/// {}", doc_line).unwrap();
    }
    res
}

fn print_class(class: &Class) -> String {
    let mut res = String::new();
    res += &clean_doc(&class.documentation);
    writeln!(res, "struct {};", class.name.as_ref().unwrap()).unwrap();
    res
}

fn print_enumeration_value(value: &EnumerationValue) -> String {
    let mut res = String::new();
    res += &clean_doc(&value.documentation);
    let name = value.name.as_deref().unwrap();
    let value = value.value.unwrap_or(0);
    writeln!(res, "{} = {},", name, value).unwrap();
    res
}

fn print_enumeration(enumeration: &Enumeration) -> String {
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

fn print_type(r#type: &Type) -> String {
    let mut res = String::new();
    let code = r#type.code.as_ref().unwrap();
    match code {
        krpc_proto::r#type::TypeCode::Tuple => {
            write!(res, "(").unwrap();
            for t in &r#type.types {
                write!(res, "{}, ", print_type(&t)).unwrap();
            }
            res = res.trim_end().to_string();
            write!(res, ")").unwrap();
        }
        krpc_proto::r#type::TypeCode::List => {
            assert_eq!(r#type.types.len(), 1);
            write!(res, "List<").unwrap();
            let t = r#type.types.first().unwrap();
            write!(res, "{}", print_type(&t)).unwrap();
            write!(res, ">").unwrap();
        }
        c if !r#type.types.is_empty() => write!(res, "{:?}<>", c).unwrap(),
        c => write!(res, "{:?}", c).unwrap(),
    }
    res
}

fn print_procedure(procedure: &Procedure) -> String {
    let mut res = String::new();

    res += &clean_doc(&procedure.documentation);

    let mut name = procedure.name.as_deref().unwrap();
    writeln!(res, "/// real name: {}", name).unwrap();

    // We have to do method name stripping here,
    // because parameters are immutable
    if is_nonstatic_method(procedure) {
        name = name.split_at(name.find('_').unwrap() + 1).1;
    } else if is_static_method(procedure) {
        name = name.split_at(name.find('_').unwrap() + 1).1;
        name = name.strip_prefix("static_").unwrap();
    }
    write!(res, "fn {}(", name).unwrap();
    if let Some(param) = &procedure.parameters.first() {
        if is_nonstatic_method(procedure) {
            write!(res, "&self").unwrap();
        } else {
            let name = match param.name.as_deref().unwrap().trim() {
                "type" => "r#type",
                n => n,
            };
            write!(
                res,
                "{}: {}",
                name,
                print_type(param.r#type.as_ref().unwrap())
            )
            .unwrap();
        }
        for param in &procedure.parameters[1..] {
            let name = match param.name.as_deref().unwrap().trim() {
                "type" => "r#type",
                n => n,
            };
            write!(
                res,
                ", {}: {}",
                name,
                print_type(param.r#type.as_ref().unwrap())
            )
            .unwrap();
        }
    }
    write!(res, ")").unwrap();

    if let Some(ret) = procedure.return_type.as_ref() {
        let ret = print_type(&ret);
        if matches!(procedure.return_is_nullable, Some(true)) {
            write!(res, " -> Option<{}>", ret).unwrap();
        } else {
            write!(res, " -> {}", ret).unwrap();
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

// Checks if first parameter is `this: Class`
fn is_nonstatic_method(p: &Procedure) -> bool {
    p.parameters
        .first()
        .filter(|f| {
            let name = f.name.as_deref();
            let code = f.r#type.as_ref().and_then(|t| t.code);
            name == Some("this") && code == Some(TypeCode::Class)
        })
        .is_some()
}

fn is_static_method(p: &Procedure) -> bool {
    p.name.as_deref().unwrap().contains("_static_")
}

// Turns list of procedures into list of class impl's + list of free procedures
fn declasser(
    procedures: &[Procedure],
) -> (HashMap<&str, Vec<&Procedure>>, Vec<&Procedure>) {
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    let mut free = vec![];

    for p in procedures {
        if is_nonstatic_method(p) || is_static_method(p) {
            let name = p.name.as_ref().unwrap();
            let (class_name, _) = name.split_at(name.find("_").unwrap());
            map.entry(class_name).or_default().push(p);
        } else {
            free.push(p);
        }
    }

    (map, free)
}

fn wrap_comments(text: String, line_width: usize) -> String {
    let mut res = String::new();
    for line in text.lines() {
        let is_comment = line.find("/// ");
        if let Some(pos) = is_comment {
            let (prefix, mut line) = line.split_at(pos + 4);
            // Correct line width for indent and slashes
            let line_width = line_width - prefix.len();
            let mut wrapped = false;
            while line.chars().count() > line_width {
                wrapped = true;
                let last_space = line.char_indices().take(line_width).fold(
                    None,
                    |acc, x| match x.1 {
                        ' ' => Some(x.0),
                        _ => acc,
                    },
                );
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
            if !line.is_empty() || !wrapped {
                writeln!(res, "{}{}", prefix, line).unwrap();
            }
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
        let (map, free) = declasser(&service.procedures);
        for proc in &free {
            let text = print_procedure(proc);
            for line in text.lines() {
                writeln!(res, "    {}", line).unwrap();
            }
            writeln!(res).unwrap();
        }
        let mut map: Vec<_> = map.into_iter().collect();
        map.sort_by_key(|&(name, _)| name);
        for (class, procs) in &map {
            writeln!(res, "    impl {} {{", class).unwrap();
            for proc in procs {
                let text = print_procedure(proc);
                for line in text.lines() {
                    writeln!(res, "        {}", line).unwrap();
                }
                writeln!(res).unwrap();
            }
            writeln!(res, "    }}").unwrap();
            writeln!(res).unwrap();
        }
        writeln!(res, "}}").unwrap();
        writeln!(res).unwrap();
    }
    wrap_comments(res, 80)
}
