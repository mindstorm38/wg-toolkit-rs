use std::fs::File;

use clap::ArgMatches;
use wgtk::pxml::{self, Element, Value};


pub fn cmd_pxml_show(matches: &ArgMatches) {

    let element_path = matches.get_one::<String>("path");

    let file_path = matches.get_one::<String>("file").unwrap();
    let file = File::open(file_path).unwrap();
    let mut root_elt = pxml::from_reader(file).unwrap();

    if let Some(path) = element_path {
        if !path.is_empty() {
            match resolve_element_path(&mut root_elt, &path, 0) {
                Ok(elt) => {
                    print!("{path}:");
                    print_value(elt, &mut "  ".to_string());
                }
                Err(PathResolveError::ChildNotFound { child, parent }) => {
                    eprintln!("error: can't find '{child}' in '{parent}'");
                }
                Err(PathResolveError::TerminalValue { child, parent }) => {
                    eprintln!("error: can't find '{child}' in '{parent}' because the later a terminal value");
                }
            }
            return
        }
    }

    // Print the whole root element.
    print_element(&root_elt, &mut String::new());

}


pub fn cmd_pxml_edit(matches: &ArgMatches) {

    

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
