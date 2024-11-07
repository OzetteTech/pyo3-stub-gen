use crate::{generate::*, type_info::*};
use std::fmt;

/// Definition of a Python class.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    pub name: &'static str,
    pub doc: &'static str,
    pub new: Option<NewDef>,
    pub members: Vec<MemberDef>,
    pub methods: Vec<MethodDef>,
}

impl Import for ClassDef {
    fn import(&self) -> HashSet<ModuleRef> {
        let mut import = HashSet::new();
        if let Some(new) = &self.new {
            import.extend(new.import());
        }
        for member in &self.members {
            import.extend(member.import());
        }
        for method in &self.methods {
            import.extend(method.import());
        }
        import
    }
}

impl From<&PyClassInfo> for ClassDef {
    fn from(info: &PyClassInfo) -> Self {
        // Since there are multiple `#[pymethods]` for a single class, we need to merge them.
        // This is only an initializer. See `StubInfo::gather` for the actual merging.
        Self {
            name: info.pyclass_name,
            new: None,
            doc: info.doc,
            members: info.members.iter().map(MemberDef::from).collect(),
            methods: Vec::new(),
        }
    }
}

impl fmt::Display for ClassDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "class {}:", self.name)?;
        let indent = indent();
        let doc = self.doc.trim();
        if !doc.is_empty() {
            writeln!(f, r#"{indent}r""""#)?;
            for line in doc.lines() {
                if !line.is_empty() {
                    writeln!(f, "{indent}{}", line)?;
                } else {
                    writeln!(f)?;
                }
            }
            writeln!(f, r#"{indent}""""#)?;
            writeln!(f)?;
        }
        for member in &self.members {
            member.fmt(f)?;
        }
        if let Some(new) = &self.new {
            new.fmt(f)?;
        }
        for method in &self.methods {
            method.fmt(f)?;
        }
        if self.members.is_empty() && self.methods.is_empty() {
            writeln!(f, "{indent}...")?;
        }
        writeln!(f)?;
        Ok(())
    }
}
