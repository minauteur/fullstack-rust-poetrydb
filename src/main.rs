extern crate reqwest;
// extern crate rusqlite;
extern crate serde;
extern crate serde_json;
mod db;
const db_path: &'static str = &"C:\\Users\\Minauteur\\Desktop\\poems.db";
use serde::*;
use reqwest::{Client, ClientBuilder};
use std::path::PathBuf;
use db::{Poem, SQLPoem, insert_poem, get_conn,  dump};
#[derive(Serialize, Deserialize, Debug)]
struct AuthorsList {
    authors: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Poems(Vec<Poem>);
fn main()->Result<(),rusqlite::Error> {
    // let client = ClientBuilder::new();
    // let e_d = "Emily Dickinson".to_string();
    // let init_url = build_req(None, None);
    let p = PathBuf::from(&db_path);
    let conn = get_conn(&p)?;
    // // let poem = insert_poem(conn, test_poem());
    // let req = reqwest::get(&init_url).expect("should have gotten something").text().unwrap();
    // println!("request text: {}", req);
    // let json_authors: AuthorsList = serde_json::from_str(&req).unwrap();
    // println!("success!");
    // let mut a_l = json_authors.authors.len() as i32;
    // for author in json_authors.authors {
    //     println!("inserting titles for {}, remaining: {}", &author, a_l);

    //     let s_url = format!("http://poetrydb.org/author/{}", author);
    //     let a_req = reqwest::get(&s_url).expect("should have gotten something").text().unwrap();
    //     let json_poems: Vec<Poem> = serde_json::from_str(&a_req).unwrap();
    //     let mut t_l = json_poems.len() as i32;
    //     for poem in json_poems {
    //         insert_poem(&conn, poem);
    //         t_l = t_l-1;
        
    //     }
    //     a_l = a_l-1;
    // }
    // list_poems(&conn)?;
    // let mut stmt = create_dump_statement(&conn);
    for poems in dump(&conn) {
        for p in poems {
        println!("{}", &p.title);
        }
    }
    // println!("response text: {}", &json.authors.join("\n"));
    Ok(())
}
fn build_req(author: Option<&String>,title: Option<&String>) -> String {
    match (author, title) {
            (None, None) => {return "http://poetrydb.org/author".to_string();},
            (Some(a), Some(t)) => {return format!("http://poetrydb.org/author,title/{}/{}", a, t)},
            (Some(a),None) => {return format!("http://poetrydb.org/author/{}", a)},
            (None, Some(t)) => {return format!("http://poetrydb.org/title/{}", t)}
        };
}
fn test_poem() -> Poem {
        Poem {
            title: "Thom's OTHER Poem: YET ANOTHER Test of Fortitude".to_string(),
            author: "Thomas W. Pennington".to_string(),
            lines: ["Thom wrote a poem so sweet".to_string(), "it made sophie tap her feet".to_string(), "when it came time to sing".to_string(), "he cast each &str to a String".to_string(),"and let this stanza ring".to_string()].to_vec(),
            linecount: "5".to_string()
        }
    }