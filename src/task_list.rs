use crate::Task;
use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskList {
    tasks: IndexMap<u8, Task>,
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
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

    pub fn complete_task(&mut self, id: u8) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.complete();
        }
    }

    pub fn remove_completed_tasks(&mut self) {
        self.tasks.retain(|_, task| !task.is_complete());
    }
}

impl fmt::Display for TaskList {
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
    fn tasks_can_be_added() {
        let task_to_add = Task::new("Buy some milk".to_string());

        let mut task_list = TaskList::default();
        task_list.add_task(task_to_add.clone());

        assert_eq!(
            task_list,
            TaskList {
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
        let task0 = Task::new("Buy some milk".to_string());
        let task1 = Task::new("Learn Haskell".to_string());
        let task2 = Task::new("Finish Chapter 10 of my novel".to_string());

        let mut task_list = TaskList::default();
        task_list.add_task(task0.clone());
        task_list.add_task(task1.clone());
        task_list.add_task(task2.clone());

        assert_eq!(task_list.tasks[&0], task0);
        assert_eq!(task_list.tasks[&1], task1);
        assert_eq!(task_list.tasks[&2], task2);
    }

    #[test]
    fn tasks_can_be_removed_by_id() {
        let mut task_list = TaskList::default();

        task_list.add_task(Task::new("Buy some milk".to_string())); // ID: 0
        task_list.add_task(Task::new("Learn Haskell".to_string())); // ID: 1
        task_list.remove_task(0);

        // The task takes the lowest available ID, which is now 0.
        task_list.add_task(Task::new("Finish Chapter 10 of my novel".to_string()));
        task_list.remove_task(1);
        task_list.remove_task(0);

        assert!(task_list.tasks.is_empty());
    }

    #[test]
    fn tasks_can_be_renamed_by_providing_an_id_and_new_title() {
        let mut task_list = TaskList::default();

        task_list.add_task(Task::new("Buy some milk".to_string()));
        task_list.rename_task(0, "Purchase some milk".to_string());

        assert_eq!(
            task_list.tasks[&0],
            Task::new("Purchase some milk".to_string())
        );
    }

    #[test]
    fn tasks_can_be_completed_by_id() {
        let mut task_list = TaskList::default();

        task_list.add_task(Task::new("Buy some milk".to_string()));
        assert!(!task_list.tasks[&0].is_complete());

        task_list.complete_task(0);
        assert!(task_list.tasks[&0].is_complete());
    }

    #[test]
    fn completed_tasks_can_be_removed() {
        let mut task_list = TaskList::default();

        task_list.add_task(Task::new("Go to the dentist".to_string()));
        task_list.add_task(Task::new("Write some tests".to_string()));
        task_list.add_task(Task::new("Refactor code".to_string()));
        task_list.complete_task(1);
        task_list.complete_task(2);

        task_list.remove_completed_tasks();

        assert_eq!(
            task_list.tasks.into_iter().collect::<Vec<_>>(),
            vec![(0, Task::new("Go to the dentist".to_string()))]
        );
    }

    #[test]
    fn task_list_implements_display() {
        let mut task_list = TaskList::default();
        task_list.add_task(Task::new("Buy some milk".to_string()));
        task_list.add_task(Task::new("Learn Haskell".to_string()));

        assert_eq!(
            format!("{}", task_list),
            "\
[  0] • Buy some milk
[  1] • Learn Haskell"
        );
    }
}
