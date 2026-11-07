// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors
//
// SPDX-License-Identifier: Apache-2.0

//! The generator of the types of IEC 60870-6-802 and -503 (task F2), from the data files of the standards.
//!
//! Rules (`PROVENANCE.md`, D-001 to D-006):
//! - a definition is a Rust type named by its data name, in CamelCase; inline component types are named after
//!   their parent and their field;
//! - a name defined in several clauses is scoped by clause: each definition goes in a module named after its clause;
//! - a reference to a name defined several times resolves to the definition of the same clause; otherwise to the
//!   main text rather than an annex, when exactly one main definition exists; otherwise to the only structurally
//!   equal definition; otherwise it is an error to be decided and recorded;
//! - a reference to a name of the MMS base types, or to a plain INTEGER, is resolved by the table below.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::Deserialize;

/// The two data files: the notation label and the path of its file, from the repository root.
const SOURCES: [(&str, &str); 2] = [
    ("6-802", "docs/spec/6-802/data/mms_types.yaml"),
    ("6-503", "docs/spec/6-503/data/mms_types.yaml"),
];

/// The module of each notation in the generated code.
const MODULES: [&str; 2] = ["iso802", "iso503"];

/// The standard of each notation, as it is written in the documents.
const STANDARDS: [&str; 2] = ["IEC 60870-6-802", "IEC 60870-6-503"];

/// The types of the MMS base types and of the scalar types that the data files name without defining them.
/// Each entry: the name in the data file, and the Rust path of the type in `crate::builtin`.
const BUILTINS: [(&str, &str); 5] = [
    ("MMS Identifier", "crate::builtin::MmsIdentifier"),
    ("Integer16", "crate::builtin::Integer16"),
    ("Integer32", "crate::builtin::Integer32"),
    ("Unsigned8", "crate::builtin::Unsigned8"),
    // Decision D-005: the plain INTEGER of the data files is a 32-bit integer, the width of every integer of the files.
    ("integer", "crate::builtin::Integer32"),
];

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Num {
    Int(i64),
    Text(String),
}

#[derive(Deserialize, Debug, Clone)]
struct Bit {
    name: String,
    bit: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum TypeRef {
    Name(String),
    #[allow(dead_code)]
    Inline(Box<Inline>),
}

/// A type written inside a definition, a component or an array.
#[derive(Deserialize, Debug, Clone)]
struct Inline {
    kind: Option<String>,
    /// The base type of an enumeration (`Boolean`: its values are the MMS Boolean, decision D-020).
    base: Option<String>,
    #[serde(rename = "type")]
    ty: Option<TypeRef>,
    width: Option<Num>,
    format_width: Option<i64>,
    exponent_width: Option<i64>,
    bits: Option<Vec<Bit>>,
    values: Option<BTreeMap<String, Num>>,
    of: Option<Box<Inline>>,
    size: Option<Num>,
    varying: Option<bool>,
    /// A constraint of the values, written `range least .. greatest` (the data files give it for some integers).
    constraint: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct Component {
    name: String,
    #[serde(rename = "type")]
    ty: Inline,
}

#[derive(Deserialize, Debug, Clone)]
struct Def {
    name: String,
    page: Option<u32>,
    clause: Option<String>,
    kind: Option<String>,
    object: Option<String>,
    class: Option<String>,
    base: Option<String>,
    size: Option<Num>,
    width: Option<Num>,
    varying: Option<bool>,
    format_width: Option<i64>,
    exponent_width: Option<i64>,
    bits: Option<Vec<Bit>>,
    components: Option<Vec<Component>>,
    of: Option<Inline>,
    #[serde(rename = "type")]
    ty: Option<TypeRef>,
    type_description: Option<String>,
    scope: Option<String>,
}

#[derive(Deserialize)]
struct File {
    definitions: Vec<Def>,
}

/// One definition of the data files, with where it was read.
#[derive(Debug, Clone)]
struct Entry {
    notation: usize,
    def: Def,
}

impl Entry {
    fn clause(&self) -> &str {
        self.def.clause.as_deref().unwrap_or("")
    }

    /// The annex of the clause (B, G or another letter), or None for the main text.
    fn annex(&self) -> Option<char> {
        let clause = self.clause();
        let first = clause.chars().next()?;
        first.is_ascii_alphabetic().then_some(first)
    }
}

/// The shape of a type, as it is written in the generated code.
#[derive(Debug, Clone)]
enum Shape {
    Float {
        format: i64,
        exponent: i64,
    },
    Integer {
        width: i64,
        range: Option<(i64, i64)>,
    },
    Unsigned {
        width: i64,
    },
    Boolean,
    /// An enumeration whose base is the MMS Boolean: each name stands for true or false (decision D-020).
    BooleanEnumerated {
        values: Vec<(String, bool)>,
    },
    Visible {
        max: i64,
        varying: bool,
    },
    Octets {
        max: Option<i64>,
    },
    Array {
        size: Option<i64>,
        element: String,
    },
    Bits {
        bits: Vec<Bit>,
    },
    Enumerated {
        values: Vec<(String, i64)>,
    },
    Structure {
        fields: Vec<(String, String)>,
    },
    Alias {
        path: String,
    },
}

/// A generated item: its Rust name, its shape, and the clause and page where it is defined.
#[derive(Debug, Clone)]
struct Item {
    name: String,
    shape: Shape,
    notation: usize,
    clause: String,
    page: u32,
    kind: &'static str,
}

/// The name of a definition, where it lives, and its Rust name.
#[derive(Clone)]
struct Placement {
    module: String,
    rust: String,
}

/// The whole of the generation: every definition, its placement, and the errors found.
#[derive(Clone)]
struct Model {
    entries: Vec<Entry>,
    placement: Vec<Placement>,
    by_name: BTreeMap<String, Vec<usize>>,
    errors: Vec<String>,
}

pub fn run(check: bool) -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let out_dir = root.join("crates/igs-iec-tase2-model/src/generated");
    let files = generate(&root)?;
    if check {
        for (name, text) in &files {
            let path = out_dir.join(name);
            let committed = std::fs::read_to_string(&path).unwrap_or_default();
            if committed != *text {
                return Err(format!(
                    "{} is stale: run `cargo xtask codegen`",
                    path.display()
                ));
            }
        }
        println!("codegen: {} files up to date", files.len());
        return Ok(());
    }
    std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    for (name, text) in &files {
        let path: PathBuf = out_dir.join(name);
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("codegen: wrote {}", path.display());
    }
    Ok(())
}

/// Reads the data files and generates the files of the model, as (file name, text).
fn generate(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut entries = Vec::new();
    for (notation, (_, path)) in SOURCES.iter().enumerate() {
        let text = std::fs::read_to_string(root.join(path)).map_err(|e| format!("{path}: {e}"))?;
        let file: File = serde_saphyr::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
        for def in file.definitions {
            entries.push(Entry { notation, def });
        }
    }
    let model = Model::new(entries);
    if !model.errors.is_empty() {
        return Err(format!(
            "{} references or definitions need a decision:\n  {}",
            model.errors.len(),
            model.errors.join("\n  ")
        ));
    }
    let mut files = Vec::new();
    for (notation, module) in MODULES.iter().enumerate() {
        let text = render_module(&model, notation)?;
        files.push((format!("{module}.rs"), rustfmt(&text)?));
    }
    files.push(("mod.rs".to_string(), rustfmt(&render_root())?));
    Ok(files)
}

impl Model {
    fn new(entries: Vec<Entry>) -> Self {
        let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (index, entry) in entries.iter().enumerate() {
            by_name
                .entry(entry.def.name.clone())
                .or_default()
                .push(index);
        }
        let mut model = Self {
            placement: Vec::new(),
            entries,
            by_name,
            errors: Vec::new(),
        };
        let mut placement = Vec::with_capacity(model.entries.len());
        for index in 0..model.entries.len() {
            let entry = &model.entries[index];
            let duplicated = model
                .by_name
                .get(&entry.def.name)
                .is_some_and(|v| v.len() > 1);
            let module = if duplicated {
                format!(
                    "clause_{}",
                    entry.clause().replace(['.', '-'], "_").to_lowercase()
                )
            } else {
                String::new()
            };
            placement.push(Placement {
                module,
                rust: pascal(&entry.def.name),
            });
        }
        model.placement = placement;
        model
    }

    fn path_of(&self, index: usize) -> String {
        let entry = &self.entries[index];
        let placement = &self.placement[index];
        let module = MODULES[entry.notation];
        if placement.module.is_empty() {
            format!("crate::generated::{module}::{}", placement.rust)
        } else {
            format!(
                "crate::generated::{module}::{}::{}",
                placement.module, placement.rust
            )
        }
    }

    /// The Rust path of a reference, from a definition: a builtin, an alias, or a definition.
    fn resolve(&mut self, target: &str, from: usize) -> Option<String> {
        if let Some((_, path)) = BUILTINS.iter().find(|(name, _)| *name == target) {
            return Some((*path).to_string());
        }
        let candidates = self.by_name.get(target).cloned().unwrap_or_default();
        let from_entry = self.entries[from].clone();
        let chosen = match candidates.len() {
            0 => None,
            1 => candidates.first().copied(),
            _ => {
                let same_clause: Vec<usize> = candidates
                    .iter()
                    .copied()
                    .filter(|&c| {
                        self.entries[c].notation == from_entry.notation
                            && self.entries[c].clause() == from_entry.clause()
                    })
                    .collect();
                if same_clause.len() == 1 {
                    same_clause.first().copied()
                } else {
                    let main: Vec<usize> = candidates
                        .iter()
                        .copied()
                        .filter(|&c| self.entries[c].annex().is_none())
                        .collect();
                    if main.len() == 1 {
                        main.first().copied()
                    } else {
                        let first = self.key(candidates[0]);
                        let all_equal = candidates.iter().all(|&c| self.key(c) == first);
                        if all_equal {
                            candidates.first().copied()
                        } else {
                            None
                        }
                    }
                }
            }
        };
        match chosen {
            Some(index) => Some(self.path_of(index)),
            None => {
                self.errors.push(format!(
                    "{} {}: reference to {target}, defined {} times",
                    STANDARDS[from_entry.notation],
                    from_entry.clause(),
                    self.by_name.get(target).map_or(0, Vec::len)
                ));
                None
            }
        }
    }

    /// The structure of a definition, for comparing the definitions of a repeated name.
    fn key(&self, index: usize) -> String {
        let def = &self.entries[index].def;
        format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
            def.kind,
            def.width,
            def.format_width,
            def.exponent_width,
            def.bits,
            def.components,
            def.of,
            def.ty
        )
    }
}

/// CamelCase of a data name: separators removed, each part capitalised, the rest of the letters kept.
fn pascal(name: &str) -> String {
    name.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// snake_case of a field name, with the Rust keywords escaped.
fn snake(name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (index, &c) in chars.iter().enumerate() {
        if !c.is_ascii_alphanumeric() {
            if !out.ends_with('_') {
                out.push('_');
            }
            continue;
        }
        let previous = index.checked_sub(1).and_then(|i| chars.get(i)).copied();
        let next = chars.get(index + 1).copied();
        let boundary = c.is_ascii_uppercase()
            && previous.is_some_and(|p| {
                p.is_ascii_lowercase()
                    || p.is_ascii_digit()
                    || (p.is_ascii_uppercase() && next.is_some_and(|n| n.is_ascii_lowercase()))
            });
        if boundary && !out.ends_with('_') && !out.is_empty() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    let trimmed = out.trim_matches('_').to_string();
    const KEYWORDS: [&str; 11] = [
        "type", "match", "ref", "loop", "move", "use", "mod", "fn", "impl", "as", "in",
    ];
    if KEYWORDS.contains(&trimmed.as_str()) {
        format!("r#{trimmed}")
    } else if trimmed == "self" || trimmed == "super" || trimmed == "crate" {
        format!("{trimmed}_")
    } else {
        trimmed
    }
}

fn number(value: &Option<Num>) -> Option<i64> {
    match value {
        Some(Num::Int(v)) => Some(*v),
        Some(Num::Text(text)) => text.trim().parse::<i64>().ok(),
        None => None,
    }
}

/// The length of a visible string written as `32 characters`.
fn leading_number(text: &str) -> Option<i64> {
    text.split_whitespace().next()?.parse().ok()
}

impl Model {
    /// The Rust name of an inline type declared under `parent` for `field`, and its shape.
    fn inline_shape(
        &mut self,
        inline: &Inline,
        from: usize,
        declared: &mut Vec<Item>,
        owner: &str,
    ) -> Option<Shape> {
        let kind = inline.kind.as_deref()?;
        Some(match kind {
            "integer" => Shape::Integer {
                width: number(&inline.width)?,
                range: parse_range(inline.constraint.as_deref()),
            },
            "unsigned" => Shape::Unsigned {
                width: number(&inline.width)?,
            },
            "floating-point" => Shape::Float {
                format: inline.format_width?,
                exponent: inline.exponent_width?,
            },
            "boolean" => Shape::Boolean,
            "visiblestring" => Shape::Visible {
                max: number(&inline.width)?,
                varying: inline.varying.unwrap_or(false),
            },
            "octetstring" => Shape::Octets {
                max: number(&inline.width),
            },
            "bit-string" => Shape::Bits {
                bits: inline.bits.clone()?,
            },
            "enumerated" if inline.base.as_deref() == Some("Boolean") => Shape::BooleanEnumerated {
                values: inline
                    .values
                    .as_ref()?
                    .iter()
                    .filter_map(|(name, value)| {
                        let set = match value {
                            Num::Int(v) => *v != 0,
                            Num::Text(t) => {
                                let t = t.trim();
                                t == "non-zero" || t.parse::<i64>().ok()? != 0
                            }
                        };
                        Some((name.clone(), set))
                    })
                    .collect(),
            },
            "enumerated" => Shape::Enumerated {
                values: inline
                    .values
                    .as_ref()?
                    .iter()
                    .filter_map(|(name, value)| {
                        let v = match value {
                            Num::Int(v) => *v,
                            Num::Text(t) => t.trim().parse().ok()?,
                        };
                        Some((name.clone(), v))
                    })
                    .collect(),
            },
            "array" => {
                let element_inline = inline.of.as_deref()?;
                let element = self.element_type(element_inline, from, declared, owner)?;
                Shape::Array {
                    size: number(&inline.size),
                    element,
                }
            }
            "reference" => {
                let target = match &inline.ty {
                    Some(TypeRef::Name(name)) => name.clone(),
                    _ => return None,
                };
                Shape::Alias {
                    path: self.resolve(&target, from)?,
                }
            }
            _ => {
                self.errors.push(format!(
                    "unknown inline kind {kind} in {}",
                    self.entries[from].def.name
                ));
                return None;
            }
        })
    }

    /// The Rust type of an element of an array, or of a component: a reference, or a declared inline type.
    fn element_type(
        &mut self,
        inline: &Inline,
        from: usize,
        declared: &mut Vec<Item>,
        owner: &str,
    ) -> Option<String> {
        if inline.kind.as_deref() == Some("reference") {
            let target = match &inline.ty {
                Some(TypeRef::Name(name)) => name.clone(),
                _ => return None,
            };
            return self.resolve(&target, from);
        }
        let shape = self.inline_shape(inline, from, declared, owner)?;
        let name = format!("{owner}Element");
        let entry = &self.entries[from];
        declared.push(Item {
            name: name.clone(),
            shape,
            notation: entry.notation,
            clause: entry.clause().to_string(),
            page: entry.def.page.unwrap_or(0),
            kind: "inline",
        });
        Some(name)
    }

    /// The Rust type of a component, declaring an inline type named after the parent and the field.
    fn component_type(
        &mut self,
        component: &Component,
        from: usize,
        declared: &mut Vec<Item>,
        parent: &str,
    ) -> Option<String> {
        let owner = format!("{parent}{}", pascal(&component.name));
        if component.ty.kind.as_deref() == Some("reference") {
            let target = match &component.ty.ty {
                Some(TypeRef::Name(name)) => name.clone(),
                _ => return None,
            };
            return self.resolve(&target, from);
        }
        let shape = self.inline_shape(&component.ty, from, declared, &owner)?;
        let entry = &self.entries[from];
        declared.push(Item {
            name: owner.clone(),
            shape,
            notation: entry.notation,
            clause: entry.clause().to_string(),
            page: entry.def.page.unwrap_or(0),
            kind: "inline",
        });
        Some(owner)
    }

    /// The items of a definition, or nothing for a variable (an object, not a type).
    fn items_of(&mut self, index: usize) -> Vec<Item> {
        let entry = self.entries[index].clone();
        let def = &entry.def;
        let mut declared = Vec::new();
        let rust = pascal(&def.name);
        let base = |kind: &'static str, shape: Shape| Item {
            name: rust.clone(),
            shape,
            notation: entry.notation,
            clause: entry.clause().to_string(),
            page: def.page.unwrap_or(0),
            kind,
        };
        if def.object.as_deref() == Some("variable") {
            return Vec::new();
        }
        let shape = match def.kind.as_deref() {
            Some("floating-point") => {
                let (format, exponent) = (
                    def.format_width.unwrap_or(0),
                    def.exponent_width.unwrap_or(0),
                );
                if (format, exponent) != (32, 8) {
                    self.errors.push(format!(
                        "{}: floating point {format}/{exponent} is not the single precision format",
                        def.name
                    ));
                    return Vec::new();
                }
                Shape::Float { format, exponent }
            }
            Some("integer") => Shape::Integer {
                width: number(&def.width).unwrap_or(0),
                range: None,
            },
            Some("unsigned") => Shape::Unsigned {
                width: number(&def.width).unwrap_or(0),
            },
            Some("boolean") => Shape::Boolean,
            Some("visiblestring") => Shape::Visible {
                max: number(&def.width).unwrap_or(0),
                varying: def.varying.unwrap_or(false),
            },
            Some("octetstring") => Shape::Octets {
                max: number(&def.width),
            },
            Some("array") => {
                let Some(of) = def.of.clone() else {
                    self.errors
                        .push(format!("array {} without elements", def.name));
                    return Vec::new();
                };
                let element = self.element_type(&of, index, &mut declared, &rust);
                match element {
                    Some(element) => Shape::Array {
                        size: number(&def.size),
                        element,
                    },
                    None => return Vec::new(),
                }
            }
            Some("bit-string") => Shape::Bits {
                bits: def.bits.clone().unwrap_or_default(),
            },
            Some("reference") => {
                let target = match &def.ty {
                    Some(TypeRef::Name(name)) => name.clone(),
                    _ => {
                        self.errors.push(format!("{} has no target", def.name));
                        return Vec::new();
                    }
                };
                match self.resolve(&target, index) {
                    Some(path) => Shape::Alias { path },
                    None => return Vec::new(),
                }
            }
            Some("structure") => {
                let mut fields = Vec::new();
                for component in def.components.clone().unwrap_or_default() {
                    if let Some(ty) = self.component_type(&component, index, &mut declared, &rust) {
                        fields.push((snake(&component.name), ty));
                    }
                }
                Shape::Structure { fields }
            }
            Some(other) => {
                self.errors
                    .push(format!("{}: unknown kind {other}", def.name));
                return Vec::new();
            }
            None => {
                if def.class.as_deref() == Some("VISIBLE STRING") {
                    let max = def.size.as_ref().and_then(|s| match s {
                        Num::Text(t) => leading_number(t),
                        Num::Int(v) => Some(*v),
                    });
                    match max {
                        Some(max) => Shape::Visible {
                            max,
                            varying: false,
                        },
                        None => return Vec::new(),
                    }
                } else if let Some(base) = def.base.as_deref() {
                    match BUILTINS.iter().find(|(name, _)| *name == base) {
                        Some((_, path)) => Shape::Alias {
                            path: (*path).to_string(),
                        },
                        None => {
                            self.errors
                                .push(format!("{}: base {base} is not a builtin", def.name));
                            return Vec::new();
                        }
                    }
                } else {
                    self.errors.push(format!("{}: no kind", def.name));
                    return Vec::new();
                }
            }
        };
        let mut items = declared;
        items.push(base(
            match def.kind.as_deref() {
                Some("structure") => "structure",
                Some("reference") => "reference",
                Some("bit-string") => "bit-string",
                _ => "type",
            },
            shape,
        ));
        items
    }
}

/// The variables of the 503 file: an object of the bilateral table, with its type and scope.
fn variables(model: &mut Model, notation: usize) -> Vec<(String, String, u32, String, String)> {
    let mut out = Vec::new();
    for index in 0..model.entries.len() {
        let entry = model.entries[index].clone();
        if entry.notation != notation || entry.def.object.as_deref() != Some("variable") {
            continue;
        }
        let target = match (&entry.def.ty, &entry.def.type_description) {
            (Some(TypeRef::Name(name)), _) => name.clone(),
            (_, Some(text)) => text.clone(),
            _ => continue,
        };
        if let Some(path) = model.resolve(&target, index) {
            out.push((
                entry.def.name.clone(),
                entry.clause().to_string(),
                entry.def.page.unwrap_or(0),
                entry.def.scope.clone().unwrap_or_default(),
                path,
            ));
        }
    }
    out
}

/// Renders the module of one notation: the metadata, the top-level items, then the clause modules.
fn render_module(source: &Model, notation: usize) -> Result<String, String> {
    let mut model = source.clone();
    model.errors.clear();
    let mut top: Vec<Item> = Vec::new();
    let mut modules: BTreeMap<String, Vec<Item>> = BTreeMap::new();
    let mut rows = Vec::new();
    // The items that each module holds: a reference to a type of the other file produces none.
    let mut defined: BTreeSet<(String, String)> = BTreeSet::new();
    for index in 0..model.entries.len() {
        if model.entries[index].notation != notation {
            continue;
        }
        let module = model.placement[index].module.clone();
        let items = model.items_of(index);
        defined.extend(items.iter().map(|item| (module.clone(), item.name.clone())));
        let entry = model.entries[index].clone();
        let components = match &items.last() {
            Some(Item {
                shape: Shape::Structure { fields },
                ..
            }) => fields.len(),
            _ => 0,
        };
        let bits = match &items.last() {
            Some(Item {
                shape: Shape::Bits { bits },
                ..
            }) => bits.iter().map(|b| b.bit + 1).max().unwrap_or(0) as usize,
            _ => 0,
        };
        let kind = items.last().map_or("variable", |i| i.kind);
        rows.push(format!(
            "    crate::value::Definition {{ name: \"{}\", standard: \"{}\", clause: \"{}\", page: {}, kind: \"{}\", components: {}, bits: {} }},",
            entry.def.name,
            STANDARDS[notation],
            entry.clause(),
            entry.def.page.unwrap_or(0),
            kind,
            components,
            bits
        ));
        if module.is_empty() {
            top.extend(items);
        } else {
            modules.entry(module).or_default().extend(items);
        }
    }
    let mut out = String::new();
    writeln!(
        out,
        "// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors"
    )
    .unwrap();
    writeln!(out, "//").unwrap();
    writeln!(out, "// SPDX-License-Identifier: Apache-2.0").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "//! Types of {}, generated from its data file by `cargo xtask codegen`.",
        STANDARDS[notation]
    )
    .unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! DO NOT EDIT. The clauses are cited on each type; the text of the standard is not in this file.").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "/// Every definition of this file: its name, clause, page, kind, components and bits."
    )
    .unwrap();
    writeln!(
        out,
        "pub const DEFINITIONS: &[crate::value::Definition] = &["
    )
    .unwrap();
    for row in &rows {
        writeln!(out, "{row}").unwrap();
    }
    writeln!(out, "];").unwrap();
    if notation == 1 {
        let vars = variables(&mut model, notation);
        writeln!(out).unwrap();
        writeln!(out, "/// The variables of the bilateral table: name, clause, page, scope, and the type of the variable.").unwrap();
        writeln!(
            out,
            "pub const VARIABLES: &[crate::value::VariableDef] = &["
        )
        .unwrap();
        for (name, clause, page, scope, path) in &vars {
            writeln!(
                out,
                "    crate::value::VariableDef {{ name: \"{name}\", clause: \"{clause}\", page: {page}, scope: \"{scope}\", type_path: \"{path}\" }},"
            )
            .unwrap();
        }
        writeln!(out, "];").unwrap();
        if !model.errors.is_empty() {
            return Err(model.errors.join("\n"));
        }
    }
    for item in &top {
        out.push_str(&render_item(item));
    }
    for (module, items) in &modules {
        writeln!(out).unwrap();
        writeln!(
            out,
            "/// The definitions of the clause {}.",
            module.trim_start_matches("clause_").replace('_', ".")
        )
        .unwrap();
        writeln!(out, "pub mod {module} {{").unwrap();
        for item in items {
            out.push_str(&render_item(item));
        }
        writeln!(out, "}}").unwrap();
    }
    // A name defined in several clauses, with exactly one definition in the main text, is also exported at the root
    // of its file: the main text prevails over the annexes, which describe earlier editions (decision D-022).
    for indices in source.by_name.values().filter(|indices| indices.len() > 1) {
        let normative: Vec<usize> = indices
            .iter()
            .copied()
            .filter(|&index| source.entries[index].annex().is_none())
            .collect();
        if let [index] = normative.as_slice() {
            let placed = &source.placement[*index];
            if source.entries[*index].notation == notation
                && defined.contains(&(placed.module.clone(), placed.rust.clone()))
            {
                writeln!(out, "pub use {}::{};", placed.module, placed.rust).unwrap();
            }
        }
    }
    if !model.errors.is_empty() {
        return Err(model.errors.join("\n"));
    }
    Ok(out)
}

/// The range written by a constraint, `range least .. greatest`; no constraint gives none.
fn parse_range(constraint: Option<&str>) -> Option<(i64, i64)> {
    let text = constraint?.strip_prefix("range")?;
    let (least, greatest) = text.split_once("..")?;
    Some((least.trim().parse().ok()?, greatest.trim().parse().ok()?))
}

/// An integer type: a newtype whose least and greatest values are those of its width, or of its constraint when the
/// data files give one (the values of the constraint are then the only ones it holds).
fn render_integer(
    out: &mut String,
    doc: &str,
    name: &str,
    width: i64,
    range: Option<(i64, i64)>,
    signed: bool,
) {
    let (min, max): (i128, i128) = match range {
        Some((least, greatest)) => (i128::from(least), i128::from(greatest)),
        None if signed => {
            let half = 1i128 << (width - 1).max(0);
            (-half, half - 1)
        }
        None => (0, (1i128 << width) - 1),
    };
    let ty = if signed { "i64" } else { "u64" };
    let field = if signed { "i64" } else { "u64" };
    writeln!(out, "{doc}").unwrap();
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct {name}({field});"
    )
    .unwrap();
    writeln!(
        out,
        "impl {name} {{\n    /// The width in bits.\n    pub const WIDTH: u32 = {width};\n    /// The least value.\n    pub const MIN: {ty} = {min};\n    /// The greatest value.\n    pub const MAX: {ty} = {max};\n    /// The value, if it is in range.\n    pub fn new(value: {ty}) -> Result<Self, crate::value::OutOfRange> {{\n        if (Self::MIN..=Self::MAX).contains(&value) {{\n            Ok(Self(value))\n        }} else {{\n            Err(crate::value::OutOfRange {{ value: i128::from(value), min: {min}, max: {max} }})\n        }}\n    }}\n    /// The value.\n    pub fn get(self) -> {ty} {{\n        self.0\n    }}\n}}"
    )
    .unwrap();
}

fn render_item(item: &Item) -> String {
    let mut out = String::new();
    let doc = format!(
        "/// {} {} (page {}). {} definition.",
        STANDARDS[item.notation], item.clause, item.page, item.kind
    );
    let name = &item.name;
    match &item.shape {
        Shape::Float { format, exponent } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Copy, PartialEq)]\npub struct {name}(pub f64);"
            )
            .unwrap();
            writeln!(out, "impl {name} {{\n    /// The format width, in bits.\n    pub const FORMAT_WIDTH: u32 = {format};\n    /// The exponent width, in bits.\n    pub const EXPONENT_WIDTH: u32 = {exponent};\n}}").unwrap();
        }
        Shape::Integer { width, range } => {
            render_integer(&mut out, &doc, name, *width, *range, true)
        }
        Shape::Unsigned { width } => render_integer(&mut out, &doc, name, *width, None, false),
        Shape::Boolean => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct {name}(pub bool);"
            )
            .unwrap();
        }
        Shape::BooleanEnumerated { values } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct {name}(pub bool);"
            )
            .unwrap();
            let mut consts = String::new();
            for (value_name, set) in values {
                writeln!(
                    consts,
                    "    /// The value `{value_name}`.\n    pub const {}: bool = {set};",
                    snake(value_name).to_uppercase()
                )
                .unwrap();
            }
            writeln!(out, "impl {name} {{\n{consts}}}").unwrap();
        }
        Shape::Visible { max, varying } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct {name}(String);"
            )
            .unwrap();
            writeln!(
                out,
                "impl {name} {{\n    /// The greatest number of characters (a maximum: decision D-004).\n    pub const MAX_LEN: usize = {max};\n    /// Whether the length is variable (the data file says so).\n    pub const VARYING: bool = {varying};\n    /// The string, if it has at most the greatest number of visible characters (decision D-014).\n    pub fn new(text: impl Into<String>) -> Result<Self, crate::value::StringError> {{\n        let text = text.into();\n        crate::value::check_visible(&text, Self::MAX_LEN)?;\n        Ok(Self(text))\n    }}\n    /// The string.\n    pub fn as_str(&self) -> &str {{\n        &self.0\n    }}\n}}"
            )
            .unwrap();
        }
        Shape::Octets { max } => {
            let max_text = match max {
                Some(max) => format!("Some({max})"),
                None => "None".to_string(),
            };
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct {name}(pub Vec<u8>);"
            )
            .unwrap();
            writeln!(out, "impl {name} {{\n    /// The greatest number of octets, when the data file gives one.\n    pub const MAX_LEN: Option<usize> = {max_text};\n}}").unwrap();
        }
        Shape::Array { size, element } => {
            let size_text = match size {
                Some(size) => format!("Some({size})"),
                None => "None".to_string(),
            };
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, PartialEq)]\npub struct {name}(pub Vec<{element}>);"
            )
            .unwrap();
            writeln!(out, "impl {name} {{\n    /// The number of elements, when the data file gives one.\n    pub const SIZE: Option<usize> = {size_text};\n}}").unwrap();
        }
        Shape::Bits { bits } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct {name}(pub u64);"
            )
            .unwrap();
            let count = bits.iter().map(|b| b.bit + 1).max().unwrap_or(0);
            let mut seen = BTreeSet::new();
            let mut methods = String::new();
            let mut consts = String::new();
            for bit in bits {
                let flag = snake(&bit.name);
                let constant = flag.to_uppercase();
                let mask: u64 = if bit.bit < 64 { 1u64 << bit.bit } else { 0 };
                if bit.name == "reserved" || !seen.insert(flag.clone()) {
                    continue;
                }
                writeln!(consts, "    /// The mask of the flag `{}`.\n    pub const {constant}: u64 = {mask:#x};", bit.name).unwrap();
                writeln!(methods, "    /// Whether the flag `{}` is set.\n    pub fn is_{flag}(self) -> bool {{\n        self.0 & Self::{constant} != 0\n    }}", bit.name).unwrap();
            }
            writeln!(out, "impl {name} {{\n    /// The number of bits of the string.\n    pub const BIT_COUNT: u32 = {count};\n{consts}}}\nimpl {name} {{\n{methods}}}").unwrap();
        }
        Shape::Enumerated { values } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct {name}(pub i64);"
            )
            .unwrap();
            let mut consts = String::new();
            for (value_name, value) in values {
                writeln!(
                    consts,
                    "    /// The value `{value_name}`.\n    pub const {}: i64 = {value};",
                    snake(value_name).to_uppercase()
                )
                .unwrap();
            }
            writeln!(out, "impl {name} {{\n{consts}}}").unwrap();
        }
        Shape::Structure { fields } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, PartialEq)]\npub struct {name} {{"
            )
            .unwrap();
            for (field, ty) in fields {
                writeln!(
                    out,
                    "    /// The component `{field}`.\n    pub {field}: {ty},"
                )
                .unwrap();
            }
            writeln!(out, "}}").unwrap();
        }
        Shape::Alias { path } => {
            writeln!(out, "{doc}").unwrap();
            writeln!(out, "pub type {name} = {path};").unwrap();
        }
    }
    out.push_str(&mms_impl(item));
    out
}

/// The conversion of a scalar type to and from its MMS value (`crate::mms::MmsValue`).
fn mms_impl(item: &Item) -> String {
    let name = &item.name;
    let data = "igs_iso9506_mms_codec::generated::types::Data";
    let body = match &item.shape {
        Shape::Integer { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::Integer(i128::from(self.0))\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Integer(value) => {{\n                let value = i64::try_from(*value).map_err(|_| crate::value::OutOfRange {{ value: *value, min: i128::from(Self::MIN), max: i128::from(Self::MAX) }})?;\n                Ok(Self::new(value)?)\n            }}\n            other => Err(crate::mms::ValueError::kind_of(\"integer\", other)),\n        }}\n    }}\n"),
        Shape::Unsigned { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::Unsigned(u128::from(self.0))\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Unsigned(value) => {{\n                let value = u64::try_from(*value).map_err(|_| crate::value::OutOfRange {{ value: i128::try_from(*value).unwrap_or(i128::MAX), min: i128::from(Self::MIN), max: i128::from(Self::MAX) }})?;\n                Ok(Self::new(value)?)\n            }}\n            other => Err(crate::mms::ValueError::kind_of(\"unsigned\", other)),\n        }}\n    }}\n"),
        Shape::Boolean | Shape::BooleanEnumerated { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::Boolean(self.0)\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Boolean(value) => Ok(Self(*value)),\n            other => Err(crate::mms::ValueError::kind_of(\"boolean\", other)),\n        }}\n    }}\n"),
        Shape::Visible { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::VisibleString(self.as_str().to_string())\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::VisibleString(text) => Ok(Self::new(text.clone())?),\n            other => Err(crate::mms::ValueError::kind_of(\"visible string\", other)),\n        }}\n    }}\n"),
        Shape::Float { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        let [a, b, c, d] = (self.0 as f32).to_be_bytes();\n        {data}::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(vec![8, a, b, c, d]))\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::FloatingPoint(igs_iso9506_mms_codec::generated::types::FloatingPoint(octets)) => match octets.as_slice() {{\n                [8, a, b, c, d] => Ok(Self(f64::from(f32::from_be_bytes([*a, *b, *c, *d])))),\n                _ => Err(crate::mms::ValueError::Format(\"a floating point value must be single precision: exponent width 8 and four octets\")),\n            }},\n            other => Err(crate::mms::ValueError::kind_of(\"floating point\", other)),\n        }}\n    }}\n"),
        Shape::Bits { .. } => {
            format!(
                "    fn to_mms(&self) -> {data} {{\n        let octets = (Self::BIT_COUNT as usize).div_ceil(8);\n        let mut bytes = vec![0u8; octets];\n        for position in 0..Self::BIT_COUNT {{\n            if self.0 & 1u64.checked_shl(position).unwrap_or(0) != 0 {{\n                if let Some(octet) = bytes.get_mut((position / 8) as usize) {{\n                    *octet |= 0x80u8 >> (position % 8);\n                }}\n            }}\n        }}\n        let unused = u8::try_from(octets.saturating_mul(8).saturating_sub(Self::BIT_COUNT as usize)).unwrap_or(0);\n        {data}::BitString(igs_iso9506_mms_codec::schema::BitString {{ unused, bytes }})\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::BitString(bits) => {{\n                let length = bits.bytes.len().saturating_mul(8).saturating_sub(usize::from(bits.unused));\n                if length < Self::BIT_COUNT as usize {{\n                    return Err(crate::mms::ValueError::Format(\"a bit string shorter than the flags of its type\"));\n                }}\n                let mut value = 0u64;\n                for position in 0..length {{\n                    let octet = bits.bytes.get(position / 8).copied().unwrap_or(0);\n                    if octet & (0x80u8 >> (position % 8)) != 0 {{\n                        if position >= Self::BIT_COUNT as usize {{\n                            return Err(crate::mms::ValueError::Format(\"a bit beyond the flags of its type is set\"));\n                        }}\n                        value |= 1u64.checked_shl(position as u32).unwrap_or(0);\n                    }}\n                }}\n                Ok(Self(value))\n            }}\n            other => Err(crate::mms::ValueError::kind_of(\"bit string\", other)),\n        }}\n    }}\n"
            )
        }
        Shape::Enumerated { values } => {
            let list: Vec<String> = values.iter().map(|(_, v)| v.to_string()).collect();
            format!(
                "    fn to_mms(&self) -> {data} {{\n        {data}::Integer(i128::from(self.0))\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Integer(value) => match i64::try_from(*value) {{\n                Ok(value) if [{}].contains(&value) => Ok(Self(value)),\n                _ => Err(crate::mms::ValueError::Format(\"not a value of the enumeration\")),\n            }},\n            other => Err(crate::mms::ValueError::kind_of(\"integer\", other)),\n        }}\n    }}\n",
                list.join(", ")
            )
        }
        Shape::Octets { .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::OctetString(self.0.clone())\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::OctetString(octets) => {{\n                if Self::MAX_LEN.is_some_and(|max| octets.len() > max) {{\n                    return Err(crate::mms::ValueError::Format(\"an octet string longer than its type\"));\n                }}\n                Ok(Self(octets.clone()))\n            }}\n            other => Err(crate::mms::ValueError::kind_of(\"octet string\", other)),\n        }}\n    }}\n"
        ),
        Shape::Array { element, .. } => format!(
            "    fn to_mms(&self) -> {data} {{\n        {data}::Array(self.0.iter().map(crate::mms::MmsValue::to_mms).collect())\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Array(items) => {{\n                if Self::SIZE.is_some_and(|size| items.len() != size) {{\n                    return Err(crate::mms::ValueError::Format(\"an array of another number of elements\"));\n                }}\n                let elements = items\n                    .iter()\n                    .map(<{element} as crate::mms::MmsValue>::from_mms)\n                    .collect::<Result<Vec<_>, _>>()?;\n                Ok(Self(elements))\n            }}\n            other => Err(crate::mms::ValueError::kind_of(\"array\", other)),\n        }}\n    }}\n"
        ),
        Shape::Structure { fields } => {
            if item.name == "TimeStampExtended" {
                // Written by hand in time.rs: the milliseconds are checked there (decision D-011).
                return String::new();
            }
            let to_items: Vec<String> = fields
                .iter()
                .map(|(field, _)| format!("crate::mms::MmsValue::to_mms(&self.{field})"))
                .collect();
            let patterns: Vec<String> = (0..fields.len()).map(|i| format!("item{i}")).collect();
            let assigned: Vec<String> = fields
                .iter()
                .enumerate()
                .map(|(i, (field, ty))| format!("{field}: <{ty} as crate::mms::MmsValue>::from_mms(item{i})?"))
                .collect();
            format!(
                "    fn to_mms(&self) -> {data} {{\n        {data}::Structure(vec![{}])\n    }}\n    fn from_mms(data: &{data}) -> Result<Self, crate::mms::ValueError> {{\n        match data {{\n            {data}::Structure(items) => match items.as_slice() {{\n                [{}] => Ok(Self {{ {} }}),\n                _ => Err(crate::mms::ValueError::Format(\"a structure of another number of components\")),\n            }},\n            other => Err(crate::mms::ValueError::kind_of(\"structure\", other)),\n        }}\n    }}\n",
                to_items.join(", "),
                patterns.join(", "),
                assigned.join(", ")
            )
        }
        _ => return String::new(),
    };
    format!("impl crate::mms::MmsValue for {name} {{\n{body}}}\n")
}

fn render_root() -> String {
    let mut out = String::new();
    writeln!(
        out,
        "// SPDX-FileCopyrightText: 2026 The igs-iec-tase2 contributors"
    )
    .unwrap();
    writeln!(out, "//").unwrap();
    writeln!(out, "// SPDX-License-Identifier: Apache-2.0").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "//! Generated by `cargo xtask codegen`. DO NOT EDIT.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "pub mod iso802;").unwrap();
    writeln!(out, "pub mod iso503;").unwrap();
    out
}

/// Formats a generated file with rustfmt, so that the committed text is the formatted one.
fn rustfmt(text: &str) -> Result<String, String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2021", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("rustfmt: {e}"))?;
    child
        .stdin
        .take()
        .ok_or("rustfmt: no stdin")?
        .write_all(text.as_bytes())
        .map_err(|e| format!("rustfmt: {e}"))?;
    let output = child
        .wait_with_output()
        .map_err(|e| format!("rustfmt: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "rustfmt failed on the generated code: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
