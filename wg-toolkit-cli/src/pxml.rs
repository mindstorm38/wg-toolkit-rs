use std::collections::{hash_map, HashMap};
use std::io::{self, Cursor, Read, Write};
use std::fs::File;

use wgtk::pxml::{self, Element, Value};

use super::{CliResult, PackedXmlArgs};


pub fn cmd_pxml(args: PackedXmlArgs) -> CliResult<()> {

    let mut root_xml_tag = "root".to_string();
    let mut root_elt = match args.file {
        Some(path) => {

            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    root_xml_tag = file_name.to_string();
                }
            }

            let file = File::open(&path)
                .map_err(|e| format!("Failed to open file at {path:?}: {e}"))?;

            pxml::from_reader(file)
                .map_err(|e| format!("Failed to read Packed XML file at {path:?}: {e}"))?

        }
        None => {

            let mut content = Vec::new();
            std::io::stdin().read_to_end(&mut content)
                .map_err(|e| format!("Failed to read content from stdin: {e}"))?;
            
            pxml::from_reader(Cursor::new(content))
                .map_err(|e| format!("Failed to read Packed XML from stdin: {e}"))?

        }
    };

    if let Some(filter) = args.filter {
        apply_filter(&mut *root_elt, &filter)?;
    }

    if args.raw {
        
        let mut buf = Vec::new();
        pxml::to_writer(Cursor::new(&mut buf), &root_elt)
            .map_err(|e| format!("Failed to write Packed XML to buffer: {e}"))?;

        io::stdout().write_all(&buf)
            .map_err(|e| format!("Failed to write Packed XML buffer to stdout: {e}"))?;

        return Ok(());

    }
    
    let mut indent = String::new();

    if args.xml {
        println!("<{root_xml_tag}>");
        indent.push_str("  ");
    }

    // Print the whole root element.
    print_element(&root_elt, &mut indent, false, args.xml);

    if args.xml {
        println!("</{root_xml_tag}>");
    } else {
        println!(); // Because 'print_element' don't print a line feed.
    }

    Ok(())

}

fn apply_filter(element: &mut Element, filter: &str) -> CliResult<()> {

    let mut context = FilterContext::new(element);

    for assign in filter.split(";") {

        let Some((dst, src)) = assign.split_once('=') else {
            return Err(format!("Invalid assignment: {assign}"));
        };

        if dst.is_empty() || src.is_empty() {
            return Err(format!("Invalid assignment: {assign}"));
        }

        let val;

        // If using a method to construct builtin values.
        if let Some((method_name, after)) = src.split_once('(') {
            if let Some((method_arg, after)) = after.split_once(')') {
                
                if !after.is_empty() {
                    return Err(format!("Invalid method call: {src} (following closing paren)"));
                }

                val = match method_name {
                    "false" => Value::Boolean(false),
                    "true" => Value::Boolean(true),
                    "int" => {

                        let i = method_arg.parse()
                            .map_err(|e| format!("Invalid integer: {e}"))?;

                        Value::Integer(i)

                    }
                    "str" => {
                        Value::String(method_arg.to_string())
                    }
                    _ => return Err(format!("Invalid method name: {method_name}")),
                }

            } else {
                return Err(format!("Invalid method call: {src} (no closing paren)"))
            }
        } else if let Some(src_val) = context.find(src, false) {
            val = src_val.clone();
        } else {
            return Err(format!("Failed to find source: {src}"));
        }

        let Some(dst) = context.find(dst, true) else {
            return Err(format!("Failed to create destination: {dst}"));
        };

        dst.clone_from(&val);
        
    }

    Ok(())

}

/// Print an element and its children, children are printed
/// prefixed with the given indent. No terminal line feed.
fn print_element(element: &Element, indent: &mut String, new_line: bool, xml: bool) {

    match &element.value {
        // If the value is an empty string, just do not print the value
        Value::String(s) if s.is_empty() => {}
        val => {
            // Incrementing indent is not really needed because the proper value 
            // should not be another element, but it can theoretically happen.
            indent.push_str("  ");
            print_value(val, indent, xml);
            indent.truncate(indent.len() - 2);
        }
    }
    
    if new_line {
        println!();
    }

    let rollback_len = indent.len();
    for (i, (child_key, child_value)) in element.iter_children_all().enumerate() {
        
        if xml {
            print!("{indent}<{child_key}>")
        } else {
            if i > 0 {
                println!();
            }
            print!("{indent}{child_key}: ");
        }

        indent.push_str("  ");
        print_value(child_value, &mut *indent, xml);
        indent.truncate(rollback_len);

        if xml {
            println!("</{child_key}>");
        }

    }

}

/// Print a Packed XML value inline -no terminal line feed-.
fn print_value(value: &Value, indent: &mut String, xml: bool) {

    let element = matches!(value, Value::Element(_));

    if xml && !element {
        print!("\t");
    }

    match value {
        Value::Element(element) => {
            print_element(&element, indent, true, xml);
            if xml {
                print!("{}", &indent[..indent.len() - 2]);
            }
        }
        Value::String(s) => {
            if xml {
                print!("{s}");
            } else {
                print!("{s:?}");
            }
        }
        &Value::Integer(n) => print!("{n}"),
        &Value::Boolean(b) => print!("{b}"),
        Value::Vector(v) => {
            if v.len() == 12 && !xml {
                // TODO: Support XML repr!
                println!();
                println!("{indent}| {:.02} | {:.02} | {:.02} | {:.02} |", v[0], v[3], v[6], v[9]);
                println!("{indent}| {:.02} | {:.02} | {:.02} | {:.02} |", v[1], v[4], v[7], v[10]);
                println!("{indent}| {:.02} | {:.02} | {:.02} | {:.02} |", v[2], v[5], v[8], v[11]);
            } else {
                for (i, &comp) in v.iter().enumerate() {
                    if i != 0 {
                        if xml {
                            print!(" ");
                        } else {
                            print!("/");
                        }
                    }
                    print!("{comp:.1}");
                }
            }
        }
    }

    if xml && !element {
        print!("\t");
    }

}

/// This function resolves a path and get a mutable reference to the value.
fn resolve_path<'xml>(elt: &'xml mut Element, path: &str, create: bool) -> Option<&'xml mut Value> {

    let (mut child_key, rest) = path.split_once('/').unwrap_or((path, ""));

    let mut index_specified = false;
    let mut index_create = false;
    let mut index = 0isize;
    
    // We also get the index of that value.
    if let Some((before, after)) = child_key.split_once('[') {
        child_key = before;
        if let Some((mut before, after)) = after.split_once(']') {
            
            // We don't want anything after the closing bracket.
            if !after.is_empty() {
                return None;
            }

            // If the index starts with a ^ then it means that we want to create a new
            // element in the element.
            if before.starts_with('^') {

                // We create mode is disabled...
                if !create {
                    return None;
                }

                before = &before[1..];
                index_create = true;

                // By default we push at the end, we don't use '-1' because that would
                // mean to insert just before the last child.
                index = elt.len() as isize;

            }

            // An empty index is equal to 0.
            if !before.is_empty() {
                index = before.parse().ok()?;
            }

            index_specified = true;

        } else {
            // No closing bracket.
            return None;
        }
    }

    let value;

    // Special key '^' used to target the value itself.
    if child_key == "^" {

        // We can't specify an index when targeting element's value, because there is one.
        if index_specified {
            return None;
        }

        value = Some(&mut elt.value);

    } else if index_create {

        let actual_index = if index >= 0 {
            index as usize
        } else {
            
            let offset = -index as usize;
            if offset > elt.len() {
                return None;
            }

            elt.len() - offset

        };

        value = Some(elt.insert_child(actual_index, child_key.to_string(), Value::default()))

    } else {

        let mut children = elt.iter_children_mut(child_key);
        if index >= 0 {
            value = children.nth(index as usize);
        } else {
            value = children.rev().nth(-index as usize - 1);
        }

    }

    let Some(value) = value else {
        return None;
    };

    if !rest.is_empty() {
        if let Value::Element(elt) = value {
            return resolve_path(&mut *elt, rest, create);
        } else {
            return None;
        }
    }

    Some(value)

}

#[derive(Debug)]
struct FilterContext<'xml> {
    /// The element to be filtered.
    element: &'xml mut Element,
    /// Temporary variables for filtering.
    variables: HashMap<String, Value>,
}

impl<'xml> FilterContext<'xml> {

    fn new(element: &'xml mut Element) -> Self {
        Self {
            element,
            variables: HashMap::new(),
        }
    }

    fn find(&mut self, mut path: &str, create: bool) -> Option<&mut Value> {

        let mut element = &mut *self.element;

        // Depending on this being temp variable or not.
        if path.starts_with('$') {

            path = &path[1..];

            let (var, rest) = path.split_once('/').unwrap_or((path, ""));
            let val = match self.variables.entry(var.to_string()) {
                hash_map::Entry::Occupied(o) => o.into_mut(),
                hash_map::Entry::Vacant(v) if create => {
                    v.insert(Value::default())
                }
                hash_map::Entry::Vacant(_) => return None,
            };

            if !rest.is_empty() {
                if let Value::Element(elt) = val {
                    element = &mut **elt;
                    path = rest;
                } else {
                    return None;  // Could not go into an non-element value
                }
            } else {
                // No further path, we return the value itself.
                return Some(val);
            }

        }

        resolve_path(element, path, create)

    }

}
