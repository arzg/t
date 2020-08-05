use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use t::db::Db;
use t::task::Task;
use t::task_list::TaskList;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    let db_path = get_db_path()?;

    let db = if db_path.exists() {
        read_db(&db_path)?
    } else {
        let default_db = Db::default();
        save_db(&db_path, &default_db)?;

        default_db
    };

    if let Some(subcommand) = opts.subcommand {
        let mut db = db;
        subcommand.execute(&mut db)?;

        save_db(&db_path, &db)?;
    } else {
        // In this case we just print the database to the user.
        println!("{}", db);
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
    /// Creates a new empty task list and sets it as current
    AddTaskList { name: String },
    /// Sets the current task list
    SetCurrent { name: String },
}

impl Subcommand {
    fn execute(self, db: &mut Db) -> anyhow::Result<()> {
        let current_task_list = db.get_current_task_list_mut().unwrap();

        match self {
            Self::Add { title } => current_task_list.add_task(Task::new(title)),
            Self::Remove { id } => current_task_list.remove_task(id),
            Self::Rename { id, new_title } => current_task_list.rename_task(id, new_title),
            Self::Complete { id } => current_task_list.complete_task(id),
            Self::RemoveCompleted => current_task_list.remove_completed_tasks(),
            Self::AddTaskList { name } => {
                db.add_task_list(name.clone(), TaskList::default());

                // This cannot fail because we just created a task list with this name, so we know
                // it must exist.
                db.set_current(name).unwrap();
            }
            Self::SetCurrent { name } => db.set_current(name)?,
        }

        Ok(())
    }
}

fn read_db(path: impl AsRef<Path>) -> anyhow::Result<Db> {
    Ok(serde_json::from_reader(fs::File::open(&path)?)?)
}

fn save_db(path: impl AsRef<Path>, db: &Db) -> anyhow::Result<()> {
    create_dir_if_missing(&path)?;

    Ok(fs::write(path, serde_json::to_vec(db)?)?)
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

fn get_db_path() -> anyhow::Result<PathBuf> {
    use etcetera::app_strategy::AppStrategy;

    let strategy =
        etcetera::app_strategy::choose_app_strategy(etcetera::app_strategy::AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "arzg".to_string(),
            app_name: "t".to_string(),
        })?;

    Ok(strategy.data_file("db.json"))
}
