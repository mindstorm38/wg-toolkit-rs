use std::path::Path;
use std::fs::File;
use std::time::SystemTime;

use clap::ArgMatches;

use wgtk::pxml::{self, Element, Value};

use super::CmdResult;


pub fn cmd_pxml_show(matches: &ArgMatches) -> CmdResult<()> {

    let file_path = matches.get_one::<String>("file").unwrap();
    let mut root_elt = cmd_read_pxml_file(file_path)?;

    if let Some(value_path) = matches.get_one::<String>("path") {
        if !value_path.is_empty() {
            let value = cmd_resolve_element_path(&mut root_elt, &value_path)?;
            print!("/{value_path}:");
            print_value(value, &mut "  ".to_string());
            return Ok(())
        }
    }

    // Print the whole root element.
    print_element(&root_elt, &mut String::new());
    Ok(())

}


pub fn cmd_pxml_edit(matches: &ArgMatches) -> CmdResult<()> {

    let file_path = matches.get_one::<String>("file").unwrap();
    let backup_file_path = format!("{file_path}.{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

    let value_path = matches.get_one::<String>("path").unwrap();
    let value_raw = matches.get_one::<String>("value").unwrap();

    let mut root_elt = cmd_read_pxml_file(file_path)?;
    let value = cmd_resolve_element_path(&mut root_elt, &value_path)?;

    print!("/{value_path} (current):");
    print_value(value, &mut "  ".to_string());

    match value {
        Value::String(s) => s.replace_range(.., &value_raw),
        Value::Integer(n) => {
            *n = value_raw.parse::<i64>()
                .map_err(|_| format!("Invalid integer."))?;
        }
        Value::Boolean(b) => {
            *b = match &value_raw[..] {
                "true" => true,
                "false" => false,
                _ => return Err(format!("Invalid boolean."))
            };
        }
        Value::Float(n) => {
            *n = value_raw.parse::<f32>()
                .map_err(|_| format!("Invalid float."))?;
        }
        _ => return Err(format!("It is not possible to edit such values."))
    }

    print!("/{value_path}     (new):");
    print_value(value, &mut "  ".to_string());

    // Make a backup file.
    std::fs::copy(file_path, backup_file_path)
        .map_err(|e| format!("Failed to backup Packed XML file, because of: {e}"))?;

    let file = File::create(file_path)
        .map_err(|e| format!("Failed to create file at {file_path:?}, because of: {e}"))?;

    pxml::to_writer(file, &root_elt)
        .map_err(|e| format!("Failed to write Packed XML file at {file_path:?}, because of: {e}"))?;

    Ok(())

}


fn cmd_read_pxml_file<P: AsRef<Path>>(path: P) -> CmdResult<Box<Element>> {

    let path = path.as_ref();

    let file = File::open(path)
        .map_err(|e| format!("Failed to open file at {path:?}, because of: {e}"))?;

    pxml::from_reader(file)
        .map_err(|e| format!("Failed to read Packed XML file at {path:?}, because of: {e}"))

}


fn cmd_resolve_element_path<'a, 'b>(
    element: &'a mut Element, 
    path: &'b str
) -> CmdResult<&'a mut Value> {
    resolve_element_path(element, path, 0)
        .map_err(|e| {
            match e {
                PathResolveError::ChildNotFound { child, parent } => {
                    format!("Can't find '{child}' in '/{parent}'")
                }
                PathResolveError::TerminalValue { child, parent } => {
                    format!("Can't find '{child}' in '/{parent}' because the later a terminal value")
                }
            }
        })
}


fn print_element(element: &Element, indent: &mut String) {
    let rollback_len = indent.len();
    for (child_key, child_value) in element.iter_children_all() {
        print!("{indent}{child_key}:");
        indent.push_str("  ");
        print_value(child_value, &mut *indent);
        indent.truncate(rollback_len);
    }
}


fn print_value(value: &Value, indent: &mut String) {
    match value {
        Value::Element(element) => {
            println!();
            print_element(&element, indent);
        }
        Value::String(s) => println!(" {s:?}"),
        &Value::Integer(n) => println!(" {n}"),
        &Value::Boolean(b) => println!(" {b}"),
        &Value::Float(n) => println!(" {n}f"),
        Value::Vec3(v) => println!(" {}/{}/{}", v.x, v.y, v.z),
        Value::Affine3(v) => {
            let mat = &v.matrix3;
            let vec = &v.translation;
            println!();
            println!("{indent}  {:.02} {:.02} {:.02} {:.02}", mat.x_axis.x, mat.y_axis.x, mat.z_axis.x, vec.x);
            println!("{indent}  {:.02} {:.02} {:.02} {:.02}", mat.x_axis.y, mat.y_axis.y, mat.z_axis.y, vec.y);
            println!("{indent}  {:.02} {:.02} {:.02} {:.02}", mat.x_axis.z, mat.y_axis.z, mat.z_axis.z, vec.z);
        }
    }

}


/// Possible errors while resolving path in an element.
enum PathResolveError<'a> {
    ChildNotFound {
        child: &'a str,
        parent: &'a str,
    },
    TerminalValue {
        child: &'a str,
        parent: &'a str,
    },
}


/// Internal recursive function to traval the given element and path
/// and return the pointed value if existing.
fn resolve_element_path<'a, 'b>(
    element: &'a mut Element, 
    path: &'b str,
    path_index: usize
) -> Result<&'a mut Value, PathResolveError<'b>> {

    let child_path = &path[path_index..];
    
    // foo/bar/baz => first = "foo", sec = "bar/baz"
    // foo/        => first = "foo", sec = ""
    // foo         => first = "foo", sec = ""
    let (first, sec) = child_path.split_once('/').unwrap_or((child_path, ""));
    
    let value = element.get_child_mut(first)
        .ok_or_else(|| PathResolveError::ChildNotFound {
            child: first,
            parent: &path[..path_index],
        })?;

    if sec.is_empty() {
        Ok(value)
    } else if let Value::Element(elt) = value {
        resolve_element_path(&mut **elt, path, path_index + first.len() + 1)
    } else {
        Err(PathResolveError::TerminalValue {
            child: sec,
            parent: &path[..(path_index + first.len())],
        })
    }

}
