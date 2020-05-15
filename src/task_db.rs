use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDb {
    tasks: HashMap<u8, crate::Task>,
}

impl TaskDb {
    pub fn add_task(&mut self, task: crate::Task) {
        let mut id_candidate = 0;

        loop {
            match self.tasks.entry(id_candidate) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(task);
                    return;
                }
                Entry::Occupied(_) => id_candidate += 1,
            }
        }
    }
}

impl Default for TaskDb {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl fmt::Display for TaskDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.tasks.len();

        let is_at_last_task = |i| i + 1 == len;

        for (i, (id, task)) in self.tasks.iter().enumerate() {
            write!(f, "[{:>3}] {}", id, task)?;

            if !is_at_last_task(i) {
                f.write_str("\n")?;
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
        assert_eq!(
            TaskDb::default(),
            TaskDb {
                tasks: HashMap::new()
            }
        );
    }

    #[test]
    fn tasks_can_be_added() {
        let task_to_add = crate::Task::new("Buy some milk".to_string());

        let mut db = TaskDb::default();
        db.add_task(task_to_add.clone());

        assert_eq!(
            db,
            TaskDb {
                tasks: {
                    let mut tasks = HashMap::new();
                    tasks.insert(0, task_to_add);
                    tasks
                }
            }
        );
    }

    #[test]
    fn ids_are_chosen_by_the_lowest_available_one() {
        let task0 = crate::Task::new("Buy some milk".to_string());
        let task1 = crate::Task::new("Learn Haskell".to_string());
        let task2 = crate::Task::new("Finish Chapter 10 of my novel".to_string());

        let mut db = TaskDb::default();
        db.add_task(task0.clone());
        db.add_task(task1.clone());
        db.add_task(task2.clone());

        assert_eq!(db.tasks[&0], task0);
        assert_eq!(db.tasks[&1], task1);
        assert_eq!(db.tasks[&2], task2);
    }

    #[test]
    fn task_db_implements_display() {
        let mut db = TaskDb::default();
        db.add_task(crate::Task::new("Buy some milk".to_string()));
        db.add_task(crate::Task::new("Learn Haskell".to_string()));

        assert_eq!(
            format!("{}", db),
            "\
[  0] • Buy some milk
[  1] • Learn Haskell"
        );
    }
}
