use crate::{generate::*, type_info::*, TypeInfo};
use std::fmt;

/// Definition of a Python function.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: &'static str,
    pub args: Vec<Arg>,
    pub r#return: TypeInfo,
    pub signature: Option<&'static str>,
    pub doc: &'static str,
}

impl Import for FunctionDef {
    fn import(&self) -> HashSet<ModuleRef> {
        let mut import = self.r#return.import.clone();
        for arg in &self.args {
            import.extend(arg.import().into_iter());
        }
        import
    }
}

impl From<&PyFunctionInfo> for FunctionDef {
    fn from(info: &PyFunctionInfo) -> Self {
        Self {
            name: info.name,
            args: info.args.iter().map(Arg::from).collect(),
            r#return: (info.r#return)(),
            doc: info.doc,
            signature: info.signature,
        }
    }
}

impl fmt::Display for FunctionDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indent = indent();

        write!(f, "def {}(", self.name)?;
        if self.args.len() > 0 {
            writeln!(f)?;
            if let Some(sig) = self.signature {
                writeln!(f, "{indent}{}", sig)?;
            } else {
                for arg in self.args.iter() {
                    writeln!(f, "{indent}{},", arg)?;
                }
            }
        }
        write!(f, ") -> {}:", self.r#return)?;

        let doc = self.doc;
        if doc.is_empty() {
            writeln!(f, " ...")?;
        } else {
            writeln!(f)?;
            writeln!(f, r#"{indent}r""""#)?;
            for line in doc.lines() {
                writeln!(f, "{indent}{}", line)?;
            }
            writeln!(f, r#"{indent}""""#)?;
            writeln!(f, "{indent}...")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
