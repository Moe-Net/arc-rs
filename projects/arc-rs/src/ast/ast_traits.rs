use crate::Arc;

use crate::ast::{KeyNode, KeyPath};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Deref, Index},
};

#[allow(unused_qualifications)]
impl Debug for Arc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Arc::Null => {
                let mut debug_trait_builder = f.debug_tuple("Null");
                debug_trait_builder.finish()
            }
            Arc::Boolean(v) => {
                let mut debug_trait_builder = f.debug_tuple("Boolean");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Number(v) => {
                let mut debug_trait_builder = f.debug_tuple("Number");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Char(v) => {
                let mut debug_trait_builder = f.debug_tuple("Char");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::String(v) => {
                let mut debug_trait_builder = f.debug_tuple("String");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Cite(v) => {
                let mut debug_trait_builder = f.debug_tuple("Cite");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::List(v) => {
                let mut debug_trait_builder = f.debug_tuple("List");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Dict(v) => {
                let mut debug_trait_builder = f.debug_tuple("Dict");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::EmptyLine => unimplemented!(),
            Arc::FreeDict(v) => {
                let mut debug_trait_builder = f.debug_tuple("FreeDict");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Record(_, _) => {
                let mut debug_trait_builder = f.debug_tuple("Record");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Key(_, _) => {
                let mut debug_trait_builder = f.debug_tuple("Key");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::HandlerString(_, _) => {
                let mut debug_trait_builder = f.debug_tuple("HandlerString");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::HandlerNumber(_, _) => {
                let mut debug_trait_builder = f.debug_tuple("HandlerNumber");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Comment(_, _) => {
                let mut debug_trait_builder = f.debug_tuple("Comment");
                // let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
        }
    }
}

impl Display for Arc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Arc::Null => write!(f, "null"),
            Arc::Boolean(ref b) => write!(f, "{}", b),
            Arc::Cite(ref p) => write!(f, "${}", p),
            Arc::Number(ref n) => write!(f, "{}", n),
            Arc::String(ref s) => write!(f, "{:?}", s),
            Arc::List(ref l) => {
                let mut code = vec![];
                let mut length = 0;
                for i in l {
                    let t = &format!("{}", i);
                    code.push(t.clone());
                    length += t.chars().count();
                }
                write!(f, "[{}]", code.join(", "))
            }
            Arc::Dict(ref map) => {
                let mut code = String::new();
                match map.len() {
                    0 => write!(f, "{{}}"),
                    1 => {
                        for (s, o) in map {
                            code.push_str(s);
                            code.push_str(":");
                            code.push_str(&format!("{}", o));
                        }
                        write!(f, "{{{}}}", code)
                    }
                    _ => {
                        for (s, o) in map {
                            code.push_str(s);
                            code.push_str(": ");
                            code.push_str(&format!("{}", o));
                            code.push_str(",\n    ");
                        }
                        write!(f, "{{\n    {}\n}}", code.trim_end())
                    }
                }
            }
            Arc::Char(_) => write!(f, "Char"),
            Arc::EmptyLine => write!(f, "EmptyLine"),
            Arc::FreeDict(_) => write!(f, "FreeDict"),
            Arc::Record(_, _) => write!(f, "Record"),
            Arc::Key(_, _) => write!(f, "Key"),
            Arc::HandlerString(_, _) => write!(f, "HandlerString"),
            Arc::HandlerNumber(_, _) => write!(f, "HandlerNumber"),
            Arc::Comment(_, _) => write!(f, "Comment"),
        }
    }
}

impl Display for KeyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            KeyNode::Key(ref s) => write!(f, "{}", s),
            KeyNode::Index(ref i) => write!(f, "{}", i),
        }
    }
}

impl Display for KeyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let v: Vec<_> = self.0.iter().map(|k| format!("{}", k)).collect();
        write!(f, "{}", v.join("."))
    }
}

impl Default for KeyPath {
    fn default() -> Self {
        Self(vec![])
    }
}

// impl Index<usize> for Arc {
// type Output = Self;
//
// fn index(&self, index: usize) -> &Self {
// match *self {
// Arc::List(ref list) => list.get(index).unwrap_or(&Arc::Null),
// _ => &Arc::Null,
// }
// }
// }
impl Index<isize> for Arc {
    type Output = Self;

    fn index(&self, index: isize) -> &Self {
        match *self {
            Arc::List(ref list) => {
                if index >= 0 {
                    list.get(index as usize).unwrap_or(&Arc::Null)
                }
                else {
                    let i = list.len() as isize + index;
                    list.get(i as usize).unwrap_or(&Arc::Null)
                }
            }
            _ => &Arc::Null,
        }
    }
}

impl<'a> Index<&'a str> for Arc {
    type Output = Self;

    fn index(&self, index: &str) -> &Self {
        match *self {
            Arc::Dict(ref object) => &object[index],
            _ => &Arc::Null,
        }
    }
}

impl Index<String> for Arc {
    type Output = Self;
    fn index(&self, index: String) -> &Self {
        self.index(index.deref())
    }
}

impl PartialEq<bool> for Arc {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Arc::Boolean(arc) => arc == other,
            _ => false,
        }
    }
}

impl PartialEq<str> for Arc {
    fn eq(&self, other: &str) -> bool {
        match self {
            Arc::String(arc) => arc.as_str() == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Arc {
    fn eq(&self, other: &String) -> bool {
        match self {
            Arc::String(arc) => arc == other,
            _ => false,
        }
    }
}
