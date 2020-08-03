use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDb {
    tasks: IndexMap<u8, crate::Task>,
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

    pub fn remove_task(&mut self, id: u8) {
        self.tasks.remove(&id);
    }

    pub fn rename_task(&mut self, id: u8, new_title: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.rename(new_title);
        }
    }

    pub fn complete(&mut self, id: u8) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.complete();
        }
    }
}

impl Default for TaskDb {
    fn default() -> Self {
        Self {
            tasks: IndexMap::new(),
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
                tasks: IndexMap::new()
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
                    let mut tasks = IndexMap::new();
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
    fn tasks_can_be_removed_by_id() {
        let mut db = TaskDb::default();

        db.add_task(crate::Task::new("Buy some milk".to_string())); // ID: 0
        db.add_task(crate::Task::new("Learn Haskell".to_string())); // ID: 1
        db.remove_task(0);

        // The task takes the lowest available ID, which is now 0.
        db.add_task(crate::Task::new(
            "Finish Chapter 10 of my novel".to_string(),
        ));
        db.remove_task(1);
        db.remove_task(0);

        assert!(db.tasks.is_empty());
    }

    #[test]
    fn tasks_can_be_renamed_by_providing_an_id_and_new_title() {
        let mut db = TaskDb::default();

        db.add_task(crate::Task::new("Buy some milk".to_string()));
        db.rename_task(0, "Purchase some milk".to_string());

        assert_eq!(
            db.tasks[&0],
            crate::Task::new("Purchase some milk".to_string())
        );
    }

    #[test]
    fn tasks_can_be_completed_by_id() {
        let mut db = TaskDb::default();

        db.add_task(crate::Task::new("Buy some milk".to_string()));
        assert!(!db.tasks[&0].is_complete());

        db.complete(0);
        assert!(db.tasks[&0].is_complete());
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
