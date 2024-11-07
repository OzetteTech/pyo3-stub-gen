use crate::{generate::*, type_info::*, TypeInfo};
use std::{collections::HashSet, fmt};

/// Definition of a class method.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDef {
    pub name: &'static str,
    pub args: Vec<Arg>,
    pub signature: Option<&'static str>,
    pub r#return: TypeInfo,
    pub doc: &'static str,
    pub is_static: bool,
    pub is_class: bool,
}

impl Import for MethodDef {
    fn import(&self) -> HashSet<ModuleRef> {
        let mut import = self.r#return.import.clone();
        for arg in &self.args {
            import.extend(arg.import().into_iter());
        }
        import
    }
}

impl From<&MethodInfo> for MethodDef {
    fn from(info: &MethodInfo) -> Self {
        Self {
            name: info.name,
            args: info.args.iter().map(Arg::from).collect(),
            signature: info.signature,
            r#return: (info.r#return)(),
            doc: info.doc,
            is_static: info.is_static,
            is_class: info.is_class,
        }
    }
}

impl fmt::Display for MethodDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indent = indent();
        if self.is_static {
            writeln!(f, "{indent}@staticmethod")?;
        } else if self.is_class {
            writeln!(f, "{indent}@classmethod")?;
        }

        if self.is_static && self.args.len() == 0 {
            // No arguments
            write!(f, "{indent}def {}() -> {}:", self.name, self.r#return)?;
        } else {
            // Some arguments
            writeln!(f, "{indent}def {}(", self.name)?;
            if self.is_class {
                writeln!(f, "{indent}{indent}cls,")?;
            } else if !self.is_static {
                writeln!(f, "{indent}{indent}self,")?;
            }
            if self.args.len() > 0 {
                if let Some(signature) = self.signature {
                    writeln!(f, "{indent}{indent}{}", signature)?;
                } else {
                    for arg in &self.args {
                        writeln!(f, "{indent}{indent}{},", arg)?;
                    }
                }
            }
            write!(f, "{indent}) -> {}:", self.r#return)?;
        }

        let doc = self.doc;
        if doc.is_empty() {
            writeln!(f, " ...")?;
        } else {
            writeln!(f)?;
            writeln!(f, r#"{indent}{indent}r""""#)?;
            for line in doc.lines() {
                if !line.is_empty() {
                    writeln!(f, "{indent}{indent}{}", line)?;
                } else {
                    writeln!(f)?;
                }
            }
            writeln!(f, r#"{indent}{indent}""""#)?;
            writeln!(f, "{indent}{indent}...")?;
        }
        Ok(())
    }
}
