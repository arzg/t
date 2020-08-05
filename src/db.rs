use crate::TaskList;
use indexmap::IndexMap;
use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub struct Db {
    task_lists: IndexMap<String, TaskList>,
}

impl Db {
    pub fn add_task_list(&mut self, name: String, task_list: TaskList) {
        self.task_lists.insert(name, task_list);
    }
}

impl fmt::Display for Db {
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
    use crate::Task;

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
                }
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
}
