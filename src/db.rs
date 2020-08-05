use crate::task_list::TaskList;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("task list with name ‘{0}’ does not exist")]
    NonExistentTaskList(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Db {
    task_lists: IndexMap<String, TaskList>,
    current_list: String,
}

impl Db {
    pub fn add_task_list(&mut self, name: String, task_list: TaskList) {
        self.task_lists.insert(name, task_list);
    }

    pub fn set_current(&mut self, new_current_list: String) -> Result<(), Error> {
        if self.task_lists.contains_key(&new_current_list) {
            self.current_list = new_current_list;
            Ok(())
        } else {
            Err(Error::NonExistentTaskList(new_current_list))
        }
    }

    pub fn get_current_task_list_mut(&mut self) -> Option<&mut TaskList> {
        self.task_lists.get_mut(&self.current_list)
    }
}

impl Default for Db {
    fn default() -> Self {
        Self {
            task_lists: {
                let mut task_lists = IndexMap::new();
                task_lists.insert("Tasks".to_string(), TaskList::default());
                task_lists
            },
            current_list: "Tasks".to_string(),
        }
    }
}

impl fmt::Display for Db {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_task_list(
            current_list: &str,
            name: &str,
            task_list: &TaskList,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            if name == current_list {
                writeln!(f, "{} (current)", name)?;
            } else {
                writeln!(f, "{}", name)?;
            }

            if task_list.is_empty() {
                write!(f, "  No tasks have been added to this task list yet")
            } else {
                // Indent each line of output by two spaces by splitting by line, adding the
                // indentation, and collecting back again.
                write!(
                    f,
                    "{}",
                    task_list
                        .to_string()
                        .lines()
                        .map(|line| format!("  {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        };

        for (name, task_list) in self.task_lists.iter().take(self.task_lists.len() - 1) {
            print_task_list(&self.current_list, name, task_list, f)?;
            writeln!(f, "\n")?;
        }

        if let Some((name, task_list)) = self.task_lists.iter().last() {
            print_task_list(&self.current_list, name, task_list, f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;

    #[test]
    fn task_lists_can_be_added() {
        let mut db = Db::default();

        let shopping_list = {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Milk".to_string()));
            tl.add_task(Task::new("Frozen pizza".to_string()));
            tl.add_task(Task::new("Yoghurt".to_string()));

            tl
        };

        let school_tasks = {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Finish history homework".to_string()));
            tl.add_task(Task::new("Write english essay".to_string()));
            tl.add_task(Task::new("Study for chemistry test".to_string()));

            tl
        };

        db.add_task_list("Shopping List".to_string(), shopping_list.clone());
        db.add_task_list("School".to_string(), school_tasks.clone());

        assert_eq!(
            db,
            Db {
                task_lists: {
                    let mut task_lists = IndexMap::new();
                    task_lists.insert("Tasks".to_string(), TaskList::default());
                    task_lists.insert("Shopping List".to_string(), shopping_list);
                    task_lists.insert("School".to_string(), school_tasks);

                    task_lists
                },
                current_list: "Tasks".to_string(),
            }
        );
    }

    #[test]
    fn display_implementation_shows_all_task_lists_and_current_task_list() {
        let mut db = Db::default();

        let default_task_list = db.get_current_task_list_mut().unwrap();

        default_task_list.add_task(Task::new("Buy laptop sleeve".to_string()));
        default_task_list.add_task(Task::new("Vacuum".to_string()));

        let novel_tasks = {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Write acknowledgements".to_string()));
            tl.add_task(Task::new("Follow up publisher".to_string()));
            tl.add_task(Task::new("Do full read-through".to_string()));

            tl
        };

        let useless_skills_tasks = {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Study next 100 digits of π".to_string()));
            tl.add_task(Task::new("Memorise 100 biggest cities".to_string()));
            tl.add_task(Task::new("Learn to speak backwards".to_string()));

            tl
        };

        db.add_task_list("Novel".to_string(), novel_tasks);
        db.add_task_list("Useless skills".to_string(), useless_skills_tasks);

        db.set_current("Novel".to_string()).unwrap();

        assert_eq!(
            format!("{}", db),
            "\
Tasks
  [  0] • Buy laptop sleeve
  [  1] • Vacuum

Novel (current)
  [  0] • Write acknowledgements
  [  1] • Follow up publisher
  [  2] • Do full read-through

Useless skills
  [  0] • Study next 100 digits of π
  [  1] • Memorise 100 biggest cities
  [  2] • Learn to speak backwards"
        );
    }

    #[test]
    fn display_implementation_shows_note_for_empty_task_lists() {
        let db = Db::default();

        assert_eq!(
            format!("{}", db),
            "\
Tasks (current)
  No tasks have been added to this task list yet"
        );
    }

    #[test]
    fn current_task_list_can_be_set() {
        let mut db = Db::default();

        db.add_task_list("Work".to_string(), TaskList::default());
        db.add_task_list("Guitar".to_string(), TaskList::default());

        db.set_current("Work".to_string()).unwrap();
        assert_eq!(db.current_list, "Work".to_string());

        db.set_current("Guitar".to_string()).unwrap();
        assert_eq!(db.current_list, "Guitar".to_string());
    }

    #[test]
    fn setting_current_task_list_to_one_that_does_not_exist_gives_error() {
        let mut db = Db::default();

        assert_eq!(
            db.set_current("Non-existent".to_string()),
            Err(Error::NonExistentTaskList("Non-existent".to_string()))
        );
    }

    #[test]
    fn current_task_list_can_be_obtained_and_mutated() {
        let mut db = Db::default();

        let mut refactoring_tasks = {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Clean up FooBar’s Display impl".to_string()));

            tl
        };

        db.add_task_list("Refactoring".to_string(), refactoring_tasks.clone());

        db.add_task_list("Code review".to_string(), TaskList::default());

        db.set_current("Refactoring".to_string()).unwrap();

        let current_task_list = db.get_current_task_list_mut();
        assert_eq!(current_task_list, Some(&mut refactoring_tasks));

        let current_task_list = current_task_list.unwrap();

        current_task_list.add_task(Task::new("Refactor foo.rs".to_string()));

        assert_eq!(db.task_lists["Refactoring"], {
            let mut tl = TaskList::default();
            tl.add_task(Task::new("Clean up FooBar’s Display impl".to_string()));
            tl.add_task(Task::new("Refactor foo.rs".to_string()));

            tl
        });
    }
}
