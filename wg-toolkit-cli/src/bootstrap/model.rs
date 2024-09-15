use std::collections::HashMap;
use std::sync::OnceLock;
use std::fmt::Write;


/// Type system.
#[derive(Debug, Default)]
pub struct TySystem {
    types: Vec<TyInternalDef>,
    names_map: HashMap<String, isize>,
}

/// Internal 
#[derive(Debug)]
pub struct TyInternalDef {
    /// The name of the type, if none this type is anonymous.
    name: Option<String>,
    /// The type definition.
    def: TyDef,
    /// Full representation of that type.
    representation: OnceLock<String>,
}

#[derive(Debug)]
pub enum TyDef {
    Alias(Ty),
    Seq(TySeq),
    Dict(TyDict),
}

/// A reference to a type, that can be used to find the type definition back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ty(isize);

impl TySystem {

    /// Return the total number of defined types, not counting the builtins.
    pub fn count(&self) -> usize {
        self.types.len()
    }

    /// Return the total number of defined types that have a name, not counting 
    /// the builtins.
    pub fn named_count(&self) -> usize {
        self.names_map.len()
    }

    /// Iterate type definitions.
    pub fn iter(&self) -> impl Iterator<Item = (Ty, Option<&'_ str>, &'_ TyDef)> + '_ {
        self.types.iter()
            .enumerate()
            .map(|(i, def)| (Ty(i as isize), def.name.as_deref(), &def.def))
    }

    /// Find a type reference given its name.
    pub fn find(&self, name: impl AsRef<str>) -> Option<Ty> {
        let name = name.as_ref();
        Ty::from_builtin_name(name)
            .or_else(|| self.names_map.get(name).copied().map(Ty))
    }

    /// Get the name from a type, the type should be a builtin, a previously 
    /// registered type or a type found. If not this function will panic, this
    /// function returns None if the type is unnamed.
    pub fn name(&self, ty: Ty) -> Option<&str> {
        ty.builtin_name()
            .or_else(|| usize::try_from(ty.0).ok()
                .and_then(|i| self.types.get(i))
                .expect("unknown type").name.as_deref())
    }

    /// Get the type definition of a given type, this panics if the type is 
    /// unknown or a builtin.
    pub fn get(&self, ty: Ty) -> &TyDef {
        if let Some(def) = usize::try_from(ty.0).ok().and_then(|i| self.types.get(i)) {
            &def.def
        } else {
            panic!("unknown type")
        }
    }

    /// Register a new type definition into the system, if the name is not specified 
    /// then it's considered anonymous, but it can be used to define aliases.
    pub fn register(&mut self, name: Option<String>, def: TyDef) -> Ty {

        // If we are defining a named alias to an anonymous type, we'll just rename the
        // aliased type to the new name without creating any alias def, this is just an
        // internal optimization.
        if name.is_some() {
            if let TyDef::Alias(alias_ty) = def {
                if alias_ty.0 >= 0 {
                    let alias_def = &mut self.types[alias_ty.0 as usize];
                    if alias_def.name.is_none() {
                        let name = name.unwrap();
                        self.names_map.insert(name.clone(), alias_ty.0);
                        alias_def.name = Some(name);
                        return alias_ty;
                    }
                }
            }
        }

        let index = self.types.len() as isize;
        if let Some(name) = name.as_ref() {
            self.names_map.insert(name.clone(), index);
        }

        self.types.push(TyInternalDef {
            name,
            def,
            representation: OnceLock::new(),
        });

        Ty(index)

    }

    pub fn representation(&self, ty: Ty) -> &str {

        if ty.0 >= 0 {

            let def = self.types.get(ty.0 as usize).expect("unknown type");
            
            def.representation.get_or_init(|| {

                if let Some(name) = &def.name {
                    return name.clone();
                }

                let mut buf = String::new();

                match &def.def {
                    TyDef::Alias(_) => {
                        unimplemented!()
                    }
                    TyDef::Seq(seq) => {
                        let ty_representation = self.representation(seq.ty);
                        let (open, close) = match seq.kind {
                            TySeqKind::Array => ('[', ']'),
                            TySeqKind::Tuple => ('(', ')'),
                        };
                        if let Some(size) = seq.size {
                            write!(buf, "{open}{ty_representation}; {size}{close}").unwrap();
                        } else {
                            write!(buf, "{open}{ty_representation}{close}").unwrap();
                        }
                    }
                    TyDef::Dict(dict) => {
                        buf.push('{');
                        for (name, prop) in &dict.properties {
                            buf.push_str(&name);
                            buf.push_str(": ");
                            buf.push_str(self.representation(prop.ty));
                            if let Some(def) = &prop.default {
                                buf.push_str(" = ");
                                buf.push_str(&def);
                            }
                            buf.push_str(", ");
                        }
                        buf.push('}');
                    }
                }

                buf

            }).as_str()

        } else {
            // This is a builtin so it will panic if the builtin is invalid.
            ty.builtin_name().expect("unknown type")
        }

    }

}

/// Defining building types.
impl Ty {

    pub const INT8: Self    = Ty(-1);
    pub const INT16: Self   = Ty(-2);
    pub const INT32: Self   = Ty(-3);
    pub const INT64: Self   = Ty(-4);
    pub const UINT8: Self   = Ty(-5);
    pub const UINT16: Self  = Ty(-6);
    pub const UINT32: Self  = Ty(-7);
    pub const UINT64: Self  = Ty(-8);

    pub const FLOAT: Self   = Ty(-9);
    pub const FLOAT32: Self = Ty(-10);
    pub const FLOAT64: Self = Ty(-11);
    pub const VECTOR2: Self = Ty(-12);
    pub const VECTOR3: Self = Ty(-13);
    pub const VECTOR4: Self = Ty(-14);

    pub const STRING: Self  = Ty(-20);
    pub const PYTHON: Self  = Ty(-21);
    pub const MAILBOX: Self = Ty(-22);

    pub fn is_builtin(self) -> bool {
        self.0 < 0
    }

    fn builtin_name(self) -> Option<&'static str> {
        Some(match self {
            Ty::INT8 =>     "INT8",
            Ty::INT16 =>    "INT16",
            Ty::INT32 =>    "INT32",
            Ty::INT64 =>    "INT64",
            Ty::UINT8 =>    "UINT8",
            Ty::UINT16 =>   "UINT16",
            Ty::UINT32 =>   "UINT32",
            Ty::UINT64 =>   "UINT64",
            Ty::FLOAT =>    "FLOAT",
            Ty::FLOAT32 =>  "FLOAT32",
            Ty::FLOAT64 =>  "FLOAT64",
            Ty::VECTOR2 =>  "VECTOR2",
            Ty::VECTOR3 =>  "VECTOR3",
            Ty::VECTOR4 =>  "VECTOR4",
            Ty::STRING =>   "STRING",
            Ty::PYTHON =>   "PYTHON",
            Ty::MAILBOX =>  "MAILBOX",
            _ => return None
        })
    }

    fn from_builtin_name(name: &str) -> Option<Self> {
        Some(match name {
            "INT8" =>       Ty::INT8,
            "INT16" =>      Ty::INT16,
            "INT32" =>      Ty::INT32,
            "INT64" =>      Ty::INT64,
            "UINT8" =>      Ty::UINT8,
            "UINT16" =>     Ty::UINT16,
            "UINT32" =>     Ty::UINT32,
            "UINT64" =>     Ty::UINT64,
            "FLOAT" =>      Ty::FLOAT,
            "FLOAT32" =>    Ty::FLOAT32,
            "FLOAT64" =>    Ty::FLOAT64,
            "VECTOR2" =>    Ty::VECTOR2,
            "VECTOR3" =>    Ty::VECTOR3,
            "VECTOR4" =>    Ty::VECTOR4,
            "STRING" =>     Ty::STRING,
            "PYTHON" =>     Ty::PYTHON,
            "MAILBOX" =>    Ty::MAILBOX,
            _ => return None
        })
    }

}

/// A type of dictionary with properties.
#[derive(Debug)]
pub struct TyDict {
    pub properties: HashMap<String, TyDictProp>,
}

#[derive(Debug)]
pub struct TyDictProp {
    pub ty: Ty,
    pub default: Option<String>,
}

impl TyDict {

    pub fn new() -> Self {
        Self { properties: HashMap::new(), }
    }

}

/// A type that is a sequence of values.
#[derive(Debug)]
pub struct TySeq {
    pub ty: Ty,
    pub kind: TySeqKind,
    pub size: Option<u32>,
}

#[derive(Debug)]
pub enum TySeqKind {
    Array,
    Tuple,
}


/// Represent the a full model of resources.
#[derive(Debug, Default)]
pub struct Model {
    /// The type system in this model where all types are defined.
    pub tys: TySystem,
    /// The list of all interfaces available.
    pub interfaces: Vec<Interface>,
    /// The list of all entities available.
    pub entities: Vec<Entity>,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub client_methods: Vec<Method>,
    pub base_methods: Vec<Method>,
    pub cell_methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub interfaces: Vec<String>,
    pub properties: Vec<Property>,
    pub client_methods: Vec<Method>,
    pub base_methods: Vec<Method>,
    pub cell_methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub exposed: bool,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg {
    pub ty: Ty,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub ty: Ty,
    pub persistent: bool,
    pub identifier: bool,
    pub indexed: bool,
    pub database_len: Option<u32>,
    pub default: Option<String>,
    pub flags: PropertyFlags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyFlags {
    None,
    Base,
    BaseAndClient,
    OwnClient,
    CellPrivate,
    CellPublic,
    AllClients,
    Unknown(String),
}
