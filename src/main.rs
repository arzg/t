use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    let db_path = get_task_db_path()?;

    let db = if db_path.exists() {
        read_db(&db_path)?
    } else {
        let empty_db = t::TaskDb::default();
        save_db(&db_path, &empty_db)?;

        empty_db
    };

    if let Some(subcommand) = opts.subcommand {
        let mut db = db;
        subcommand.execute(&mut db);

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
    Add { title: String },
    Remove { id: u8 },
    Rename { id: u8, new_title: String },
    Complete { id: u8 },
}

impl Subcommand {
    fn execute(self, db: &mut t::TaskDb) {
        match self {
            Self::Add { title } => db.add_task(t::Task::new(title)),
            Self::Remove { id } => db.remove_task(id),
            Self::Rename { id, new_title } => db.rename_task(id, new_title),
            Self::Complete { id } => db.complete(id),
        }
    }
}

fn read_db(path: impl AsRef<Path>) -> anyhow::Result<t::TaskDb> {
    Ok(serde_json::from_reader(fs::File::open(&path)?)?)
}

fn save_db(path: impl AsRef<Path>, db: &t::TaskDb) -> anyhow::Result<()> {
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

fn get_task_db_path() -> anyhow::Result<PathBuf> {
    use etcetera::app_strategy::AppStrategy;

    let strategy =
        etcetera::app_strategy::choose_app_strategy(etcetera::app_strategy::AppStrategyArgs {
            top_level_domain: "com".to_string(),
            author: "arzg".to_string(),
            app_name: "t".to_string(),
        })?;

    Ok(strategy.data_file("task_db.json"))
}
