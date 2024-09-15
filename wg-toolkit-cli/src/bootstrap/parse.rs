//! This module loads the full model from the game's resources.

use wgtk::pxml::{Element, Value};

use crate::bootstrap::model::TyDef;

use super::model::{
    Interface, Entity,
    Property, PropertyFlags, Method, Arg,
    TyDictProp, Ty, TyDict, TySeq, TySeqKind, TySystem
};


pub fn parse_aliases(elt: &Element, tys: &mut TySystem) {

    for (name, val) in elt.iter_children_all() {
        let ty = parse_ty(val, &mut *tys);
        tys.register(Some(name.clone()), TyDef::Alias(ty));
    }

}

pub fn parse_interface(elt: &Element, tys: &mut TySystem, name: String) -> Interface {

    let mut interface = Interface {
        name,
        base_methods: Vec::new(),
        client_methods: Vec::new(),
        cell_methods: Vec::new(),
    };

    // TODO: TempProperties (teambase_vehicle.def)

    if let Some(Value::Element(elt)) = elt.get_child("ClientMethods") {
        parse_methods(&elt, &mut *tys, &mut interface.client_methods);
    }

    if let Some(Value::Element(elt)) = elt.get_child("BaseMethods") {
        parse_methods(&elt, &mut *tys, &mut interface.base_methods);
    }

    if let Some(Value::Element(elt)) = elt.get_child("CellMethods") {
        parse_methods(&elt, &mut *tys, &mut interface.cell_methods);
    }

    interface

}

pub fn parse_entity(elt: &Element, tys: &mut TySystem, name: String) -> Entity {

    let mut entity = Entity {
        name,
        interfaces: Vec::new(),
        properties: Vec::new(),
        client_methods: Vec::new(),
        base_methods: Vec::new(),
        cell_methods: Vec::new(),
    };

    if let Some(Value::Element(elt)) = elt.get_child("Implements") {
        for interface_elt in elt.iter_children("Interface") {
            if let Some(interface_name) = interface_elt.as_string() {
                entity.interfaces.push(interface_name.to_string());
            }
        }
    }

    if let Some(Value::Element(elt)) = elt.get_child("Properties") {
        parse_properties(&elt, &mut *tys, &mut entity.properties);
    }

    if let Some(Value::Element(elt)) = elt.get_child("ClientMethods") {
        parse_methods(&elt, &mut *tys, &mut entity.client_methods);
    }

    if let Some(Value::Element(elt)) = elt.get_child("BaseMethods") {
        parse_methods(&elt, &mut *tys, &mut entity.base_methods);
    }

    if let Some(Value::Element(elt)) = elt.get_child("CellMethods") {
        parse_methods(&elt, &mut *tys, &mut entity.cell_methods);
    }

    entity

}

pub fn parse_properties(elt: &Element, tys: &mut TySystem, properties: &mut Vec<Property>) {
    for (name, val) in elt.iter_children_all() {
        if let Value::Element(property_elt) = val {
            properties.push(parse_property(&property_elt, &mut *tys, name.clone()));
        }
    }
}

pub fn parse_property(elt: &Element, tys: &mut TySystem, name: String) -> Property {

    let ty_val = elt.get_child("Type").expect("property should contain a type");
    let ty = parse_ty(ty_val, &mut *tys);

    let flags = elt.get_child("Flags")
        .and_then(Value::as_string)
        .unwrap_or_default();

    // TODO: AllowUnsafeData
    // TODO: Backupable
    // TODO: ExposedForReplay

    Property {
        name,
        ty,
        persistent: elt.get_child("Persistent")
            .and_then(Value::as_boolean)
            .unwrap_or_default(),
        identifier: elt.get_child("Identifier")
            .and_then(Value::as_boolean)
            .unwrap_or_default(),
        indexed: elt.get_child("Indexed")
            .and_then(Value::as_boolean)
            .unwrap_or_default(),
        database_len: elt.get_child("DatabaseLength")
            .and_then(Value::as_integer)
            .and_then(|v| u32::try_from(v).ok()),
        default: None, // TODO:
        flags: match flags {
            "" => PropertyFlags::None,
            "BASE" => PropertyFlags::Base,
            "BASE_AND_CLIENT" => PropertyFlags::BaseAndClient,
            "OWN_CLIENT" => PropertyFlags::OwnClient,
            "CELL_PRIVATE" => PropertyFlags::CellPrivate,
            "CELL_PUBLIC" => PropertyFlags::CellPublic,
            "ALL_CLIENTS" => PropertyFlags::AllClients,
            raw => PropertyFlags::Unknown(raw.to_string()),
        },
    }

}

pub fn parse_methods(elt: &Element, tys: &mut TySystem, methods: &mut Vec<Method>) {
    for (name, val) in elt.iter_children_all() {
        if let Value::Element(method_elt) = val {
            methods.push(parse_method(&method_elt, &mut *tys, name.clone()));
        }
    }
}

pub fn parse_method(elt: &Element, tys: &mut TySystem, name: String) -> Method {
    
    let mut method = Method {
        name,
        exposed: elt.get_child("Exposed")
            .and_then(Value::as_boolean)
            .unwrap_or_default(),
        args: Vec::new(),
    };

    // TODO: VariableLengthHeaderSize
    // TODO: AllowUnsafeData
    // TODO: IgnoreIfNoClient

    for arg_val in elt.iter_children("Arg") {
        let ty = parse_ty(arg_val, &mut *tys);
        method.args.push(Arg {
            ty,
        });
    }

    method
    
}

pub fn parse_ty(val: &Value, tys: &mut TySystem) -> Ty {
    match val {
        Value::String(name) => {
            parse_ty_name(&name, tys)
        }
        Value::Element(elt)
        if elt.value.as_string() == Some("FIXED_DICT") => {
            
            let properties_elt = elt.get_child("Properties")
                .and_then(Value::as_element)
                .expect("fixed dict should have properties");

            let mut dict = TyDict::new();
            for (field_name, field_val) in properties_elt.iter_children_all() {

                let field_elt = field_val.as_element().expect("field should be an element");
                let type_val = field_elt.get_child("Type").expect("field should contain a type");
                let ty = parse_ty(type_val, tys);

                dict.properties.insert(field_name.clone(), TyDictProp {
                    ty,
                    default: None,
                });

            }
            
            tys.register(None, TyDef::Dict(dict))

        }
        Value::Element(elt) => {

            let kind = match elt.value.as_string() {
                None => panic!("missing type element value: {val:?}"),
                Some("ARRAY") => TySeqKind::Array,
                Some("TUPLE") => TySeqKind::Tuple,
                Some(name) => {
                    // TODO: Support for default value.
                    return parse_ty_name(&name, tys);
                }
            };

            let ty = elt.get_child("of")
                .map(|val| parse_ty(val, &mut *tys))
                .expect("missing array type: {val:?}");

            let size = elt.get_child("size")
                .map(|val| val.as_integer().unwrap());

            tys.register(None, TyDef::Seq(TySeq {
                ty,
                kind,
                size: size.and_then(|v| u32::try_from(v).ok()),
            }))

        }
        _ => panic!("unsupported type: {val:?}")
    }
}

pub fn parse_ty_name(name: &str, tys: &mut TySystem) -> Ty {
    match tys.find(name) {
        Some(ty) => ty,
        None => panic!("unknown type: {name}"),
    }
}
