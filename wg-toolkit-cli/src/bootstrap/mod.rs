use std::collections::HashMap;
use std::io::{self, Write, BufWriter};
use std::path::Path;
use std::fs::File;

use wgtk::res::ResFilesystem;
use wgtk::pxml;

use crate::{BootstrapArgs, CliResult};

mod parse;
mod model;

use model::{Method, Model, Ty, TyDef, PropertyFlags};


/// Entrypoint.
pub fn cmd_bootstrap(args: BootstrapArgs) -> CliResult<()> {

    let fs = ResFilesystem::new(args.dir)
        .map_err(|e| format!("Failed to open resource filesystem, reason: {e}"))?;
        
    let model = load_model(fs)
        .map_err(|e| format!("Failed to load model, reason: {e}"))?;
    
    generate_model(&args.dest, &model)
        .map_err(|e| format!("Failed to generate model, reason: {e}"))?;

    Ok(())

}

fn load_model(fs: ResFilesystem) -> io::Result<Model> {

    let mut model = Model::default();

    println!("== Reading aliases...");
    let alias_reader = fs.read("scripts/entity_defs/alias.xml")?;
    let alias_elt = pxml::from_reader(alias_reader).unwrap();
    parse::parse_aliases(&alias_elt, &mut model.tys);

    println!("== Reading interfaces...");
    for interface_file in fs.read_dir("scripts/entity_defs/interfaces")? {
        
        let interface_file = interface_file?;
        let Some((interface_name, "")) = interface_file.name().split_once(".def") else {
            continue;
        };

        println!(" = {interface_name}");

        let interface_reader = fs.read(interface_file.path())?;
        let interface_elt = pxml::from_reader(interface_reader).unwrap();
        let interface = parse::parse_interface(&interface_elt, &mut model.tys, interface_name.to_string());
        model.interfaces.push(interface);

    }

    println!("== Reading entities...");
    for entity_file in fs.read_dir("scripts/entity_defs")? {

        let entity_file = entity_file?;
        let Some((entity_name, "")) = entity_file.name().split_once(".def") else {
            continue;
        };

        println!(" = {entity_name}");

        let entity_reader = fs.read(entity_file.path())?;
        let entity_elt = pxml::from_reader(entity_reader).unwrap();
        let entity = parse::parse_entity(&entity_elt, &mut model.tys, entity_name.to_string());
        model.entities.push(entity);

    }

    println!("== Types...");
    println!(" = Count: {}", model.tys.count());
    println!(" = Named count: {}", model.tys.named_count());

    Ok(model)

}

fn generate_model(dest: &Path, model: &Model) -> io::Result<()> {

    let mut gen_model = GenModel {
        model,
        inline_types: HashMap::new(),
    };

    println!("== Writing aliases...");
    let aliases_file = dest.join("aliases.rs");
    let mut a_writer = BufWriter::new(File::create(&aliases_file)?);

    writeln!(a_writer, "#![allow(non_camel_case_types, non_snake_case)]")?;
    writeln!(a_writer)?;
    writeln!(a_writer, "pub type Vector2 = [f32; 2];")?;
    writeln!(a_writer, "pub type Vector3 = [f32; 3];")?;
    writeln!(a_writer, "pub type Vector4 = [f32; 4];")?;
    writeln!(a_writer, "pub type Python = String;")?;
    writeln!(a_writer, "pub type Mailbox = String;")?;
    writeln!(a_writer)?;

    let mut rust_names = HashMap::<Ty, String>::new();
    let mut rust_anon_count = 0usize;
    let mut rust_last_struct = false;

    for (ty, name, def) in model.tys.iter() {

        assert!(!ty.is_builtin());

        let rust_name = match rust_names.get(&ty) {
            Some(rust_name) => rust_name.as_str(),
            None => {
                let name = name.map(str::to_string).unwrap_or_else(|| {
                    // Type is unnamed so we need an unnamed index and count.
                    let s = format!("Anonymous{rust_anon_count}");
                    rust_anon_count += 1;
                    s
                });
                rust_names.insert(ty, name);
                rust_names.get(&ty).unwrap().as_str()
            }
        };

        match *def {
            TyDef::Alias(aliased_ty) => {

                let aliased_name = gen_model.type_name(aliased_ty).to_string();
                gen_model.inline_types.insert(ty, aliased_name);

                if name.is_none() {
                    continue;
                }

            }
            TyDef::Seq(ref seq) => {

                let component_name = gen_model.type_name(seq.ty);
                let inline_type = if let Some(size) = seq.size {
                    format!("Box<[{component_name}; {size}]>")
                } else {
                    format!("Vec<{component_name}>")
                };

                gen_model.inline_types.insert(ty, inline_type);

                if name.is_none() {
                    continue;
                }

            }
            TyDef::Dict(_) => {
                gen_model.inline_types.insert(ty, rust_name.to_string());
            }
        }

        match *def {
            TyDef::Alias(_) |
            TyDef::Seq(_) => {

                let inline_type = gen_model.type_name(ty);
                writeln!(a_writer, "pub type {rust_name} = {inline_type};")?;
                rust_last_struct = false;

            }
            TyDef::Dict(ref dict) => {

                if !rust_last_struct {
                    writeln!(a_writer)?;
                    rust_last_struct = true;
                }

                writeln!(a_writer, "#[derive(Debug)]")?;
                writeln!(a_writer, "pub struct {rust_name} {{")?;

                for (name, prop) in &dict.properties {

                    let name = match name.as_str() {
                        "type" => "r#type",
                        _ => name.as_str(),
                    };
                    
                    let prop_ty_name = gen_model.type_name(prop.ty);
                    writeln!(a_writer, "    pub {name}: {prop_ty_name},")?;

                }

                writeln!(a_writer, "}}")?;
                writeln!(a_writer)?;

            }
        }

    }

    println!("== Writing interfaces...");
    let interfaces_file = dest.join("interfaces.rs");
    let mut i_writer = BufWriter::new(File::create(&interfaces_file)?);

    writeln!(i_writer, "#![allow(non_camel_case_types, non_snake_case)]")?;
    writeln!(i_writer)?;
    writeln!(i_writer, "use crate::aliases::*;")?;
    writeln!(i_writer)?;

    for interface in &model.interfaces {

        // // TODO: Interface properties...
        // writeln!(i_writer, "pub struct {} {{", interface.name)?;
        // writeln!(i_writer)?;
        // writeln!(i_writer, "}}")?;
        // writeln!(i_writer)?;

        writeln!(i_writer, "// {}", interface.name)?;
        writeln!(i_writer)?;

        generate_methods(&mut i_writer, &gen_model, &interface.client_methods, &[], &interface.name, "Client")?;
        generate_methods(&mut i_writer, &gen_model, &interface.base_methods, &[], &interface.name, "Base")?;
        generate_methods(&mut i_writer, &gen_model, &interface.cell_methods, &[], &interface.name, "Cell")?;

    }

    println!("== Writing entities...");
    let entities_file = dest.join("entities.rs");
    let mut e_writer = BufWriter::new(File::create(&entities_file)?);

    writeln!(e_writer, "#![allow(non_camel_case_types, non_snake_case)]")?;
    writeln!(e_writer)?;
    writeln!(e_writer, "use crate::aliases::*;")?;
    writeln!(e_writer, "use crate::interfaces::*;")?;
    writeln!(e_writer)?;

    for entity in &model.entities {

        // // TODO: Interface properties...
        // writeln!(i_writer, "pub struct {} {{", interface.name)?;
        // writeln!(i_writer)?;
        // writeln!(i_writer, "}}")?;
        // writeln!(i_writer)?;

        writeln!(e_writer, "// {}", entity.name)?;
        writeln!(e_writer)?;

        writeln!(e_writer, "pub struct {} {{", entity.name)?;
        for property in &entity.properties {
            write!(e_writer, "    {}: {},", property.name, gen_model.type_name(property.ty))?;
            match property.flags {
                PropertyFlags::None => (),
                PropertyFlags::Base => write!(e_writer, " // base")?,
                PropertyFlags::BaseAndClient => write!(e_writer, " // base and client")?,
                PropertyFlags::OwnClient => write!(e_writer, " // own client")?,
                PropertyFlags::CellPrivate => write!(e_writer, " // cell private")?,
                PropertyFlags::CellPublic => write!(e_writer, " // cell public")?,
                PropertyFlags::AllClients => write!(e_writer, " // all clients")?,
                PropertyFlags::Unknown(ref raw) => write!(e_writer, " // unknown: {raw}")?,
            }
            writeln!(e_writer)?;
        }
        writeln!(e_writer, "}}")?;
        writeln!(e_writer)?;

        generate_methods(&mut e_writer, &gen_model, &entity.client_methods, &entity.interfaces, &entity.name, "Client")?;
        generate_methods(&mut e_writer, &gen_model, &entity.base_methods, &entity.interfaces, &entity.name, "Base")?;
        generate_methods(&mut e_writer, &gen_model, &entity.cell_methods, &entity.interfaces, &entity.name, "Cell")?;

    }

    Ok(())

}

fn generate_methods(mut writer: impl Write, gen_model: &GenModel, methods: &[Method], interfaces: &[String], object: &str, app: &str) -> io::Result<()> {

    write!(writer, "pub enum {object}_{app} {{ ")?;

    if !methods.is_empty() || !interfaces.is_empty() {
        writeln!(writer)?;
    }

    for interface_name in interfaces {
        writeln!(writer, "    {interface_name}({interface_name}_{app}),")?;
    }

    for method in methods {
        write!(writer, "    {}(", method.name)?;
        for arg in &method.args {
            let arg_ty_name = gen_model.type_name(arg.ty);
            write!(writer, "{arg_ty_name}, ")?;
        }
        write!(writer, "),")?;
        if method.exposed {
            write!(writer, " // exposed")?;
        }
        writeln!(writer)?;
    }

    writeln!(writer, "}}")?;
    writeln!(writer)?;

    Ok(())

}

/// Internal structure for tracking the generation of code.
#[derive(Debug)]
struct GenModel<'a> {
    /// The underlying model that we want to generate.
    model: &'a Model,
    /// For each non-builtin type, this represent a 
    inline_types: HashMap<Ty, String>,
}

impl GenModel<'_> {
    
    fn type_name(&self, ty: Ty) -> &str {
        if ty.is_builtin() {
            self.builtin_type_name(ty)
        } else {
            self.inline_types.get(&ty)  
                .unwrap_or_else(|| panic!("unknown type: {:?}", self.model.tys.name(ty)))
                .as_str()
        }
    }

    fn builtin_type_name(&self, ty: Ty) -> &'static str {
        match ty {
            Ty::INT8 =>     "i8",
            Ty::INT16 =>    "i16",
            Ty::INT32 =>    "i32",
            Ty::INT64 =>    "i64",
            Ty::UINT8 =>    "u8",
            Ty::UINT16 =>   "u16",
            Ty::UINT32 =>   "u32",
            Ty::UINT64 =>   "u64",
            Ty::FLOAT |
            Ty::FLOAT32 =>  "f32",
            Ty::FLOAT64 =>  "f64",
            Ty::VECTOR2 =>  "Vector2",
            Ty::VECTOR3 =>  "Vector3",
            Ty::VECTOR4 =>  "Vector4",
            Ty::STRING =>   "String",
            Ty::PYTHON =>   "Python",
            Ty::MAILBOX =>  "Mailbox",
            _ => panic!("unknown builtin: {}", self.model.tys.name(ty).unwrap()),
        }
    }

}