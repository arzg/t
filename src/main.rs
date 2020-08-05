use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use t::task::Task;
use t::task_list::TaskList;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    let task_list_path = get_task_list_path()?;

    let task_list = if task_list_path.exists() {
        read_task_list(&task_list_path)?
    } else {
        let empty_task_list = TaskList::default();
        save_task_list(&task_list_path, &empty_task_list)?;

        empty_task_list
    };

    if let Some(subcommand) = opts.subcommand {
        let mut task_list = task_list;
        subcommand.execute(&mut task_list);

        save_task_list(&task_list_path, &task_list)?;
    } else {
        // In this case we just print the task list to the user.
        println!("{}", task_list);
    }

    Ok(())
}

#[derive(StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(StructOpt)]
enum Subcommand {
    /// Adds a task to the database
    Add { title: String },
    /// Removes a task from the database
    Remove { id: u8 },
    /// Renames a task
    Rename { id: u8, new_title: String },
    /// Marks a task as completed
    Complete { id: u8 },
    /// Removes all completed tasks
    RemoveCompleted,
}

impl Subcommand {
    fn execute(self, task_list: &mut TaskList) {
        match self {
            Self::Add { title } => task_list.add_task(Task::new(title)),
            Self::Remove { id } => task_list.remove_task(id),
            Self::Rename { id, new_title } => task_list.rename_task(id, new_title),
            Self::Complete { id } => task_list.complete_task(id),
            Self::RemoveCompleted => task_list.remove_completed_tasks(),
        }
    }
}

fn read_task_list(path: impl AsRef<Path>) -> anyhow::Result<TaskList> {
    Ok(serde_json::from_reader(fs::File::open(&path)?)?)
}

fn save_task_list(path: impl AsRef<Path>, task_list: &TaskList) -> anyhow::Result<()> {
    create_dir_if_missing(&path)?;

    Ok(fs::write(path, serde_json::to_vec(task_list)?)?)
}

fn create_dir_if_missing(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();

    if let Some(parent_path) = path.parent() {
        if !parent_path.exists() {
            fs::create_dir_all(parent_path)?;
        }
    }

    Ok(())
}

fn get_task_list_path() -> anyhow::Result<PathBuf> {
    use etcetera::app_strategy::AppStrategy;

    let strategy =
        etcetera::app_strategy::choose_app_strategy(etcetera::app_strategy::AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "arzg".to_string(),
            app_name: "t".to_string(),
        })?;

    Ok(strategy.data_file("task_list.json"))
}
