use crate::task_list::TaskList;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HasCurrentList(String);
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoCurrentList;

pub trait CurrentListState {}
impl CurrentListState for HasCurrentList {}
impl CurrentListState for NoCurrentList {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Db<CurrentList: CurrentListState> {
    task_lists: IndexMap<String, TaskList>,
    current_list: CurrentList,
}

impl<CurrentList: CurrentListState> Db<CurrentList> {
    pub fn add_task_list(&mut self, name: String, task_list: TaskList) {
        self.task_lists.insert(name, task_list);
    }

    #[must_use = "This function has no effect other than returning a new Db instance, so not using it is most likely a mistake."]
    pub fn set_current(self, new_current_list: String) -> Db<HasCurrentList> {
        Db {
            task_lists: self.task_lists,
            current_list: HasCurrentList(new_current_list),
        }
    }
}

impl Db<HasCurrentList> {
    pub fn get_current_task_list_mut(&mut self) -> Option<&mut TaskList> {
        self.task_lists.get_mut(&self.current_list.0)
    }
}

impl Default for Db<NoCurrentList> {
    fn default() -> Self {
        Self {
            task_lists: IndexMap::new(),
            current_list: NoCurrentList,
        }
    }
}

impl<CurrentList: CurrentListState> fmt::Display for Db<CurrentList> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, task_list) in self.task_lists.iter().take(self.task_lists.len() - 1) {
            writeln!(f, "{}", name)?;
            writeln!(f, "{}\n", task_list)?;
        }

        if let Some((name, task_list)) = self.task_lists.iter().last() {
            writeln!(f, "{}", name)?;
            write!(f, "{}", task_list)?;
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
                    task_lists.insert("Shopping List".to_string(), shopping_list);
                    task_lists.insert("School".to_string(), school_tasks);

                    task_lists
                },
                current_list: NoCurrentList,
            }
        );
    }

    #[test]
    fn display_implementation_shows_all_task_lists() {
        let mut db = Db::default();

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

        assert_eq!(
            format!("{}", db),
            "\
Novel
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
    fn current_task_list_can_be_set() {
        let mut db = Db::default();

        db.add_task_list("Work".to_string(), TaskList::default());
        db.add_task_list("Guitar".to_string(), TaskList::default());

        let db = db.set_current("Work".to_string());
        assert_eq!(db.current_list.0, "Work".to_string());

        let db = db.set_current("Personal".to_string());
        assert_eq!(db.current_list.0, "Personal".to_string());
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

        let mut db = db.set_current("Refactoring".to_string());

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
