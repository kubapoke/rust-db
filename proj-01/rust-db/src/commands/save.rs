use std::fs;
use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;
use crate::errors::Error;

pub struct SaveCommand<'a> {
    path: String,
    commands: &'a Vec<String>
}

impl<'a> SaveCommand<'a> {
    pub fn new(path: String, commands: &'a Vec<String>) -> Self {
        Self { path, commands }
    }
}

impl Command for SaveCommand<'_> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        let commands = self.commands.join("\n");
        fs::write(self.path.clone(), commands)
            .map_err(|e| Error::IOError(format!("Failed to write to {}: {}", self.path, e)))?;
        Ok(ExecutionSuccessValue::SuccessFileOperation(format!("Saved {} commands", self.commands.len())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_save() {
        let commands = vec![
            "CREATE books KEY id
             FIELDS id: String, year: Int".to_string(),
            "INSERT id = \"1\", year = 2000 INTO books".to_string(),
            "INSERT id = \"2\", year = 2001 INTO books".to_string()
        ];

        let mut cmd = SaveCommand::new("save_test_output.txt".to_string(), &commands);
        
        let result = cmd.execute().unwrap();
        assert!(matches!(result, ExecutionSuccessValue::SuccessFileOperation(_)));

        let file_contents = fs::read_to_string("save_test_output.txt").unwrap();

        assert!(file_contents.contains("CREATE books KEY id"));
        assert!(file_contents.contains("INSERT id = \"1\", year = 2000 INTO books"));
        assert!(file_contents.contains("INSERT id = \"2\", year = 2001 INTO books"));

        fs::remove_file("save_test_output.txt").unwrap();
    }
}
