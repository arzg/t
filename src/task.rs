use crate::Status;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    title: String,
    status: Status,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            status: Status::Incomplete,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn complete(&mut self) {
        self.status = Status::Complete;
    }

    fn rename(&mut self, new_title: String) {
        self.title = new_title;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_a_task_is_created_its_status_is_set_to_incomplete() {
        assert_eq!(
            Task::new("Buy some milk".to_string()).status,
            Status::Incomplete
        );
    }

    #[test]
    fn tasks_have_a_title() {
        assert_eq!(
            Task::new("Buy some milk".to_string()).title(),
            "Buy some milk"
        );
    }

    #[test]
    fn tasks_can_be_completed() {
        let mut task = Task::new("Buy some milk".to_string());
        task.complete();

        assert_eq!(task.status, Status::Complete);
    }

    #[test]
    fn tasks_can_be_renamed() {
        let mut task = Task::new("Buy some milk".to_string());
        task.rename("Purchase some milk".to_string());

        assert_eq!(task.title(), "Purchase some milk");
    }

    #[test]
    fn incomplete_tasks_get_bullet() {
        let task = Task {
            title: "Buy some milk".to_string(),
            status: Status::Incomplete,
        };

        assert_eq!(format!("{}", task), "• Buy some milk");
    }

    #[test]
    fn complete_tasks_get_en_dash() {
        let task = Task {
            title: "Buy some milk".to_string(),
            status: Status::Complete,
        };

        assert_eq!(format!("{}", task), "– Buy some milk");
    }
}
