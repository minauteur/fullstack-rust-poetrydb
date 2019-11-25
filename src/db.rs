use rusqlite::{Connection, Error as DbError, Row};
use time::{Timespec, get_time};
use std::path::{Path, PathBuf};
use rusqlite::*;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;
// use poetrydb_scraper::QueryOptsAndArgs;

const ORIGINAL_IDS: &'static i32 = &3057;
pub struct QueryOptsAndArgs {
    pub any: bool,
    pub args: Vec<PoemQueryArgs>,
}
#[derive(Debug, Clone)]
pub struct SQLPoem {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub lines: String,
    pub linecount: i32,
    pub created_at: Timespec,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub linecount: String,
}
impl From<Poem> for SQLPoem {
    fn from(p: Poem)->SQLPoem {
        return SQLPoem {
            id: 0,
            title: p.title,
            author: p.author,
            lines: p.lines.join("\n"),
            linecount: p.linecount.parse::<i32>().expect("should always be a parsable i32 here"),
            created_at: get_time()
        }
    }
} 
impl From<SQLPoem> for Poem {
    fn from(s: SQLPoem) -> Poem {
        // let ln_vec: Vec<String> = Vec::new();
        let lns: Vec<String> = s.lines.split("\n").map(|x|x.to_string()).collect();
        let lc = lns.len().to_string();
        return Poem {
            title: s.title,
            author: s.author,
            lines: lns,
            linecount: lc
        }

    }
}


pub fn get_conn(p: &Path) -> Result<(Connection)> {
    println!("initializing");
    let path = Path::new(&p);
    // let p = &db_path;
    let conn = rusqlite::Connection::open(&path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS poems (
            id                  INTEGER PRIMARY KEY,
            title               TEXT NOT NULL,
            author              TEXT NOT NULL,
            lines               TEXT NOT NULL,
            linecount           INTEGER NOT NULL,
            created_at          TEXT NOT NULL,
            CONSTRAINT title_author UNIQUE (title, author)
            )",
        params![],
    )?;
    Ok(conn)
}
pub fn make_v_table(conn: &Connection)->Result<(&Connection)> {
    conn.execute(
        "CREATE VIRTUAL TABLE PoemSearch USING fts4(id, title, author, lines, linecount, created_at)", 
        params![],
    )?;
    conn.execute("INSERT INTO PoemSearch SELECT id, title, author, lines, linecount, created_at FROM poems",
        params![],
    )?;
    Ok(conn)
}
pub fn insert_poem(conn: &Connection, poem: Poem) -> Result<()> {
    let prepared_poem: SQLPoem = poem.into();
    conn.execute(
        "INSERT INTO poems (title, author, lines, linecount, created_at)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
        params![prepared_poem.title, prepared_poem.author, prepared_poem.lines, prepared_poem.linecount, prepared_poem.created_at],
    )?;
    println!("success!");
    Ok(())
}
// 
// pub fn list_poems(conn:&Connection)-> Result<()> {
    // WHERE: A = Fn()->()>

    // let mut poem_iter = &mut stmt.query_map(params![], move |row| {
    //     Ok(SQLPoem {
    //         id: row.get(0)?,
    //         title: row.get(1)?,
    //         author: row.get(2)?,
    //         lines: row.get(3)?,
    //         linecount: row.get(4)?,
    //         created_at: row.get(5)?,
    //     })
    // });
    // let poem_vec: Vec<SQLPoem> = *&poem_iter.collect();
    // let n = poem_vec.len() as i32;
    // let mut n_m = n.clone() as i32;
    // for sql_poem in poem_iter.into_iter() {
    //     let u_poem: SQLPoem = sql_poem.unwrap().clone();
    //     let p: Poem = u_poem.clone().into();
    //     let format = "%a %b %e %T.%f %Y";

    //     let time = time::at(u_poem.created_at);
    //     println!("Found a poem! \n{}, by {}, \nlines: {}\n  inserted at {}\n poems remaining: {} of {}", &p.title, &p.author, &p.lines.join("\n"), &time.strftime("%c").unwrap(), &n_m, &n);
    //     n_m = n_m-1;
    // }
    // Ok(())
// }
// pub fn dump<'c>(&'c self) -> impl Iterator<Item = Result<Poem>> + 'c ->
// pub fn create_dump_statement(conn: &Connection) -> Statement {
    
//     return stmt;
// }
impl SQLPoem {
    pub fn map(row: &Row) -> Result<SQLPoem> {
        Ok(SQLPoem {
            id: row.get(0).unwrap(),
            title: row.get::<_,String>(1)?,
            author: row.get::<_,String>(2)?,
            lines: row.get::<_,String>(3)?,
            linecount: row.get::<_,i32>(4)?,
            created_at: row.get::<_,Timespec>(5)?
        })
    }
}
impl Poem {
    pub fn map(row: &Row) -> Result<Poem> {
        Ok(Poem{
            title: row.get::<_,String>(1)?,
            author: row.get::<_,String>(2)?,
            lines: row.get::<_,String>(3)?.split("\n").map(
                |x| x.to_string()
            ).collect(),
            linecount: row.get::<_,i32>(4)?.to_string()
        })
    }
}
pub fn sz(conn: &Connection) -> Result<Vec<SQLPoem>> {
    let mut stmt = conn.prepare("SELECT * FROM poems ORDER BY id DESC LIMIT 1")?;
    let row = stmt.query_map(params![], SQLPoem::map)?;
    row.collect()
}
pub fn execute(conn: &Connection, stmt: &mut Statement) -> Result<Vec<Poem>> {
    let res = stmt.query_map(params![], Poem::map)?;
    res.collect()
}
pub fn dump(conn: &Connection) -> Result<Vec<SQLPoem>> {
    let mut stmt = conn.prepare("SELECT id, title, author, lines, linecount, created_at FROM poems WHERE author LIKE 'Emily Dickinson'")?;
    let rows = stmt.query_map(params![], SQLPoem::map)?;
    rows.collect()
}
pub enum PoemQueryArgs {
    Id(i32),
    Title(String),
    Author(String),
    Lines(String),
    LineCount(String),
    CreatedAt(Timespec),
}
impl PoemQueryArgs {
    pub fn get_string(self: Self) -> String {
        match self {
            PoemQueryArgs::Id(s) => {format!("id = '{}'", s)},
            PoemQueryArgs::Title(t) => {format!("title MATCH '{}'", t)},
            PoemQueryArgs::Author(a) => {format!("author MATCH '{}'",a)},
            PoemQueryArgs::Lines(l) => {format!("lines MATCH '{}'", l)},
            PoemQueryArgs::LineCount(lc) => {format!("linecount MATCH '{}'",lc)},
            PoemQueryArgs::CreatedAt(ts) => {format!("created_at MATCH '{}'", time::at(ts).strftime("%c").unwrap().to_string())}
        }
    }
}

pub fn build_query<'conn>(conn: &Connection, opts: QueryOptsAndArgs) -> Result<Statement> {
    let mut stmt_str = format!("SELECT id, title, author, lines, linecount, created_at FROM PoemSearch");
    let original_authors_only = format!("id < 3058");    
    let mut s_args: Vec<_> = Vec::new();
    for arg in opts.args.into_iter() {
        s_args.push(arg.get_string());
    }
    let any = |x: bool|-> String {
        if x == true {
            " OR ".to_string()
        } else {
            " AND ".to_string()
        }
    };
    let mut c_args = String::new();
    if s_args.len() > 0 {
        c_args = format!("{} WHERE {};", stmt_str, s_args.join(&any(opts.any)));
    } else {
        c_args = stmt_str;
    }
    let mut stmt = conn.prepare(&c_args)?;
    Ok(stmt)
}