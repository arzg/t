use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDb {
    tasks: Vec<crate::Task>,
}

impl TaskDb {
    pub fn add_task(&mut self, task: crate::Task) {
        self.tasks.push(task);
    }
}

impl Default for TaskDb {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

impl fmt::Display for TaskDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.tasks.len();

        let is_at_last_task = |i| i + 1 == len;

        for (i, task) in self.tasks.iter().enumerate() {
            if is_at_last_task(i) {
                write!(f, "{}", task)?;
            } else {
                writeln!(f, "{}", task)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_db_default_value_is_empty() {
        assert_eq!(TaskDb::default(), TaskDb { tasks: vec![] });
    }

    #[test]
    fn tasks_can_be_added() {
        let task_to_add = crate::Task::new("Buy some milk".to_string());

        let mut db = TaskDb::default();
        db.add_task(task_to_add.clone());

        assert_eq!(
            db,
            TaskDb {
                tasks: vec![task_to_add],
            }
        );
    }

    #[test]
    fn task_db_implements_display() {
        let mut db = TaskDb::default();
        db.add_task(crate::Task::new("Buy some milk".to_string()));
        db.add_task(crate::Task::new("Learn Haskell".to_string()));

        assert_eq!(
            format!("{}", db),
            "\
• Buy some milk
• Learn Haskell"
        );
    }
}
