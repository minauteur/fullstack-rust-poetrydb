extern crate reqwest;
extern crate hyper;
// extern crate rusqlite;
extern crate serde;
// extern crate poetrydb_scraper;
// use std::io::Read;
use std::io::{BufReader, BufWriter, Read, Write, BufRead};
extern crate serde_json;
mod db;
mod gen;
const db_path: &'static str = &"C:\\Users\\Minauteur\\Desktop\\poems.db";
use serde::*;
use reqwest::{Client, ClientBuilder};
use std::path::PathBuf;
use rusqlite::*;
use markov::*;
use iron::*;
use iron::{Iron, Handler, Request, Response, IronResult, Chain, status};
use iron::method::Method;
use iron::mime::Mime;
use router::Router;
use gen::*;
use db::{Poem, SQLPoem, insert_poem, get_conn,  dump, sz, execute, build_query, PoemQueryArgs, QueryOptsAndArgs};
use iron_cors::CorsMiddleware;
struct SearchHandler {}
impl Handler for SearchHandler {
    fn handle(&self, req:&mut Request)->IronResult<Response> {
    //READ THE RESPONSE (PARSE request data into JSONQuery, then convert from JSONQuery into PQ before feeding to DB and returning response/seeding lines)
   
        let content_type = "application/json".parse::<Mime>().unwrap();
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let json: JSONQuery = serde_json::from_str(&payload).expect("should always be valid JSON");
        println!("got a valid JSONQuery! Now collect these into Vec<PoemQueryArgs>! \nauthor strings: {}\ntitle strings: {}\nline strings: {}", &json.authors.join("\n"),&json.titles.join("\n"), &json.lines.join("\n") );
        let poem_query_args: QueryOptsAndArgs = QueryOptsAndArgs::from(json);
        let poems = handle_select(poem_query_args).expect("should always get some poems back");
        payload.push_str("\n");
        let mut p_vec: Vec<Poem> = Vec::new();
        for poem in poems.into_iter() {
            p_vec.push(poem);
        }
        let str_val: String = serde_json::to_string(&p_vec).expect("shouldn't ever fail!");
        let mut r = Response::with((content_type, status::Ok, str_val));
        r.headers = Headers::new();
        r.headers.set(headers::AccessControlAllowMethods(vec![Method::Post]));
        r.headers.set(headers::AccessControlAllowOrigin::Any);

        // r.headers = iron::headers::Headers::new();
        // r.headers.set(iron::headers::AccessControlAllowOrigin::Any);
        // r.headers.set(content_type);
        Ok(r)
    }
    
}
fn TEST_ARGS() -> Vec<PoemQueryArgs> { vec!(PoemQueryArgs::Id(12), PoemQueryArgs::Author("Emily Dickinson".to_string()), PoemQueryArgs::Title("I Saw".to_string()))}
#[derive(Serialize, Deserialize, Debug)]
struct AuthorsList {
    authors: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Poems(Vec<Poem>);
pub fn build_chain()->iron::Chain {
    let search_h = SearchHandler {};
    let gen_h = GenHandler {};
    let mut router  = router::Router::new();
    router.get("/", hello, "hello");
    router.post("/search", search_h, "search_h");
    router.post("/generate", gen_h, "gen_h");
    iron::Chain::new(router)
}
use rusqlite::Error as DbError;
fn main()->Result<()> {
    //     let mut ch = markov::Chain::of_order(1);
    // ch.feed_str("I like cats and I like dogs.");
    // for line in ch.iter_for(5) {
    //     println!("{}", line.join(" "));
    // }
    let mut c = build_chain();
    let cors_middle = CorsMiddleware::with_allow_any();
    println!("Cors should be set to allow *");
    c.link_around(cors_middle);

    // router.post("/generate", generate, "generate");
    // router.post("/search", search, "search");


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
    // for poems in dump(&conn) {
    //     for p in poems {
    //     println!("{}", &p.title);
    //     }
    // }
    // let num = sz(&conn);
    // for rec in num {
    //     println!("number of original poems: {}", &rec[0].id);

    // }
    // let mut stmt = build_query(&conn, TEST_ARGS())?;
    // let poems = execute(&conn,&mut stmt)?;
    // let poems_len = poems.len();
    // println!("found {} matching poems!", poems_len);
    // for poem in poems {
    //     println!("got a poem! {}", poem.title);
    // }
    // println!("response text: {}", &json.authors.join("\n"));
    Iron::new(c).http("127.0.0.1:3000").unwrap();
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
    pub fn hello(_:&mut Request)-> iron::IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }
    // pub fn query_handler(req: &mut Request) -> IronResult<Response> {
    //         let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    //         Ok(Response::with((status::Ok, *query)))
    // }

    pub fn handle_select(opts: QueryOptsAndArgs)->Result<Vec<Poem>>{
        let p = PathBuf::from(&db_path);
        let conn = get_conn(&p)?;
        let mut stmt = build_query(&conn, opts)?;
        execute(&conn,&mut stmt)
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct JSONQuery{
        any: bool,
        authors: Vec<String>,
        titles: Vec<String>,
        lines: Vec<String>
    }
    impl JSONQuery {
        pub fn new()->JSONQuery {
            JSONQuery {
                any: false,
                authors: Vec::new(),
                titles: Vec::new(),
                lines: Vec::new(),
            }
        }
    }
    pub struct PQ(Vec<PoemQueryArgs>);
    impl From<JSONQuery> for QueryOptsAndArgs {
        fn from(s:JSONQuery) -> QueryOptsAndArgs {
            let v: PQ = PQ::from(s.clone());
            QueryOptsAndArgs {
                any: s.any,
                args: v.0
            }
        }
    }
    impl From<JSONQuery> for PQ {
        fn from(s: JSONQuery)-> PQ {
            let mut v: PQ = PQ(Vec::new());

            for name in s.authors {
                if name != "" {
                    println!("got author query! {}", &name);
                    let new = PoemQueryArgs::Author(name);
                    v.0.push(new);
                }
            }
            for title in s.titles {
                if title != ""{
                    println!("got title query! {}", &title);
                    let new = PoemQueryArgs::Title(title);
                    v.0.push(new);
                }
            }
            for lines in s.lines {
                if lines != "" { 
                    println!("got lines query! {}", &lines);
                    let new = PoemQueryArgs::Lines(lines);
                    v.0.push(new);
                }
            }
            return v
        }
    }