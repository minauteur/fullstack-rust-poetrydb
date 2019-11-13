use rusqlite::{Connection, Error as DbError, Row};
use time::{Timespec, get_time};
use std::path::{Path, PathBuf};
use rusqlite::*;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;
#[derive(Debug, Clone)]
pub struct SQLPoem {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub lines: String,
    pub linecount: i32,
    pub created_at: Timespec,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub linecount: String,
}
impl Into<SQLPoem> for Poem {
    fn into(self: Self)->SQLPoem {
        return SQLPoem {
            id: 0,
            title: self.title,
            author: self.author,
            lines: self.lines.join("\n"),
            linecount: self.linecount.parse::<i32>().expect("should always be a parsable i32 here"),
            created_at: get_time()
        }
    }
} 
impl Into<Poem> for SQLPoem {
    fn into(self: Self) -> Poem {
        // let ln_vec: Vec<String> = Vec::new();
        let lns: Vec<String> = self.lines.split("\n").map(|x|x.to_string()).collect();
        let lc = lns.len().to_string();
        return Poem {
            title: self.title,
            author: self.author,
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
    fn map(row: &Row) -> Result<SQLPoem> {
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
    CreatedAt(Timespec)
}
pub fn build_query<'conn>(conn: &Connection, args: Vec<PoemQueryArgs>) -> Result<Statement> {
    
    let mut stmt = conn.prepare("SELECT id, title, author, lines, linecount, created_at FROM poems");
    stmt
}