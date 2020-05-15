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
}

impl Subcommand {
    fn execute(self, db: &mut t::TaskDb) {
        match self {
            Self::Add { title } => db.add_task(t::Task::new(title)),
        }
    }
}

fn read_db(path: impl AsRef<Path>) -> anyhow::Result<t::TaskDb> {
    Ok(serde_json::from_reader(fs::File::open(&path)?)?)
}

fn save_db(path: impl AsRef<Path>, db: &t::TaskDb) -> anyhow::Result<()> {
    Ok(fs::write(path, serde_json::to_vec(db)?)?)
}

fn get_task_db_path() -> anyhow::Result<PathBuf> {
    let base_dirs = xdg::BaseDirectories::new()?;
    Ok(base_dirs.place_data_file("t/task_db.json")?)
}
