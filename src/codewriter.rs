use std::fmt::{Display, Formatter};

pub struct CodeWriter {
    indent_level: usize,
    output: String,
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter {
            indent_level: 0,
            output: String::new(),
        }
    }

    pub fn write_line(&mut self, line: &str) {
        self.output.reserve(2 * self.indent_level + line.len() + 1);
        for _i in 0..self.indent_level {
            self.output += "  ";
        }

        self.output += line;

        self.output += "\n";
    }

    pub fn line<F>(&mut self, composer: F)
    where
        F: FnOnce(&mut LineWriter),
    {
        let mut writer = LineWriter::new();
        composer(&mut writer);
        self.write_line(writer.line.as_str());
    }

    pub fn blank_line(&mut self) {
        self.output += "\n";
    }

    pub fn write_block<F>(&mut self, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.indent_level += 1;
        cb(self);
        self.indent_level -= 1;
    }

    pub fn as_str(&mut self) -> &str {
        self.output.as_str()
    }
}

pub struct LineWriter {
    line: String,
}

impl LineWriter {
    fn new() -> Self {
        LineWriter {
            line: String::new()
        }
    }

    pub fn write(&mut self, content: &str) {
        self.line += content;
    }
}

impl Into<String> for CodeWriter {
    fn into(self) -> String {
        self.output
    }
}

impl Display for CodeWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.output))
    }
}