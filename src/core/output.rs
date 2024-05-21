pub const SHORT_STRING_MAX_LEN: usize = 4;

/// The `ShortString` just gives guarantees that inner string has size less than some setted value.
pub struct ShortString(String);

impl ShortString {
    /// The `new` function returns `None` if size of the string in argument is bit enough.
    pub fn new(string: String) -> Option<ShortString> {
        if string.len() <= SHORT_STRING_MAX_LEN {
            Some(ShortString(string))
        } else {
            None
        }
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

pub enum OtherInfo {
    Table2Column {
        data: Vec<(String, String)>,
    },

    /// There need to be invariant: `data.len == columns * rows`.
    ///
    /// [SS, SS, SS, SS, SS, SS, SS, SS, SS, SS]
    /// |__________________||__________________|
    ///          ROW                ROW
    BigTable {
        columns: Vec<ShortString>,
        rows: Vec<String>,
        data: Vec<ShortString>,
    },
}

pub type InfoLine = String;

pub type Warning = String;

/// The `CoreOutput` structure is buffer the core's commands writes their results to.
///
/// There's the following format of output:
/// - Info lines. What command did, what command create, what command delete, etc.
/// - Warnings. What command couldn't did, what command wants to pay attention to, etc.
/// - Other info. This is for specific commands that outputs some difficult structures.
pub struct CoreOutput {
    info: Vec<InfoLine>,
    warnings: Vec<Warning>,
    other_info: Vec<OtherInfo>,
}

impl CoreOutput {
    /// The `new` function creates empty `CoreOutput`: no info, no warning, no other info, etc.
    pub fn new() -> Self {
        CoreOutput {
            info: Vec::new(),
            warnings: Vec::new(),
            other_info: Vec::new(),
        }
    }

    pub fn push_info(&mut self, info_line: InfoLine) {
        self.info.push(info_line)
    }

    pub fn push_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    pub fn push_other_info(&mut self, other_info: OtherInfo) {
        self.other_info.push(other_info);
    }

    pub fn info(&self) -> &[InfoLine] {
        &self.info
    }

    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    pub fn other_info(&self) -> &[OtherInfo] {
        &self.other_info
    }
}
