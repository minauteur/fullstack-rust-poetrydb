use iron::*;
use std::io::{BufReader, BufWriter, Read, Write, BufRead};
use iron::mime::Mime;
use iron::method::Method;

use serde_json::*;
use serde::{Serialize, Deserialize};
use markov::Chain;
use crate::db::Poem;
#[derive(Serialize, Deserialize, Clone, Debug )]
pub struct GenOpts {
    gen_name: bool,
    len: i32,
    seed_poems: Vec<Poem>
}
impl GenOpts {
    pub fn new()->GenOpts {
        GenOpts {
            gen_name: true,
            len: 12,
            seed_poems: <Vec<Poem>>::new()
        }
    }
}
pub struct GenHandler {}
impl Handler for GenHandler {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        // let mut res = Response::new();
        // res.headers = Headers::new();
        // res.headers.set(&content_type);
        let mut line_ch: Chain<String> = Chain::new();
        let mut auth_ch: Chain<String> = Chain::new();
        let mut title_ch: Chain<String> = Chain::new();
        let mut body = String::new();
        req.body.read_to_string(&mut body).expect("should always get a valid string in response body");
        let opts: GenOpts = serde_json::from_str(&body).expect("should always get valid JSON here");
        let n_lines = opts.len;
        let mut auth_vec: Vec<String> = Vec::new();
        let mut letter_ch: Chain<char> = Chain::new();
        for poem in opts.seed_poems.into_iter() {
            title_ch.feed_str(&poem.title);
            auth_ch.feed_str(&poem.author);
            let a_chars: Vec<char> = poem.author.chars().collect();
            // let chars = poem.author.split(|s|{s==""}).collect(); 
            letter_ch.feed(a_chars.to_vec());
            
            let mut lines = poem.lines.into_iter();
            while let Some(l) = lines.next() {
                line_ch.feed_str(&l);
            };
        }
        let mut new_lines: Vec<String> = Vec::new();
        for l in line_ch.str_iter_for(opts.len as usize) {
            new_lines.push(l);
        }
        let mut other_vec: Vec<char> = Vec::new();
        for c in letter_ch.iter_for(10) {
            let mut c_v = c.into_iter();
            while let Some(ch) = c_v.next(){
                other_vec.push(ch);

            }
        }
        let o_s: String = other_vec.into_iter().collect();
        let newer_a: Vec<&str> = o_s.split_whitespace().into_iter().take(2).collect();
        let newest_a: String = newer_a.join(" ");
        // let mut other_a = letter_ch.generate_str();
        println!("OTHER AUTHOR (FROM LETTER_CH): {}", newest_a);
        let mut new_a = auth_ch.generate_str();
        let mut new_t = title_ch.generate_str();
        let new_poem = Poem {
            title: new_t,
            author: newest_a,
            lines: new_lines,
            linecount: opts.len.to_string(),
        };
        let new_poem_string = serde_json::to_string(&new_poem).expect("should always be parsable!");
        let mut r = Response::with((content_type, status::Ok, new_poem_string));
        r.headers = Headers::new();
        r.headers.set(headers::AccessControlAllowMethods(vec![Method::Post]));
        r.headers.set(headers::AccessControlAllowOrigin::Any);
        Ok(r)
    }
}
static A_JSON: &'static str = r#"{
  "authors": [
    "Adam Lindsay Gordon",
    "Alan Seeger",
    "Alexander Pope",
    "Algernon Charles Swinburne",
    "Ambrose Bierce",
    "Amy Levy",
    "Andrew Marvell",
    "Ann Taylor",
    "Anne Bradstreet",
    "Anne Bronte",
    "Anne Killigrew",
    "Anne Kingsmill Finch",
    "Annie Louisa Walker",
    "Arthur Hugh Clough",
    "Ben Jonson",
    "Charles Kingsley",
    "Charles Sorley",
    "Charlotte Bronte",
    "Charlotte Smith",
    "Christina Rossetti",
    "Christopher Marlowe",
    "Christopher Smart",
    "Coventry Patmore",
    "Edgar Allan Poe",
    "Edmund Spenser",
    "Edward Fitzgerald",
    "Edward Lear",
    "Edward Taylor",
    "Edward Thomas",
    "Eliza Cook",
    "Elizabeth Barrett Browning",
    "Emily Bronte",
    "Emily Dickinson",
    "Emma Lazarus",
    "Ernest Dowson",
    "Eugene Field",
    "Francis Thompson",
    "Geoffrey Chaucer",
    "George Eliot",
    "George Gordon, Lord Byron",
    "George Herbert",
    "George Meredith",
    "Gerard Manley Hopkins",
    "Helen Hunt Jackson",
    "Henry David Thoreau",
    "Henry Vaughan",
    "Henry Wadsworth Longfellow",
    "Hugh Henry Brackenridge",
    "Isaac Watts",
    "James Henry Leigh Hunt",
    "James Thomson",
    "James Whitcomb Riley",
    "Jane Austen",
    "Jane Taylor",
    "John Clare",
    "John Donne",
    "John Dryden",
    "John Greenleaf Whittier",
    "John Keats",
    "John McCrae",
    "John Milton",
    "John Trumbull",
    "John Wilmot",
    "Jonathan Swift",
    "Joseph Warton",
    "Joyce Kilmer",
    "Julia Ward Howe",
    "Jupiter Hammon",
    "Katherine Philips",
    "Lady Mary Chudleigh",
    "Lewis Carroll",
    "Lord Alfred Tennyson",
    "Louisa May Alcott",
    "Major Henry Livingston, Jr.",
    "Mark Twain",
    "Mary Elizabeth Coleridge",
    "Matthew Arnold",
    "Matthew Prior",
    "Michael Drayton",
    "Oliver Goldsmith",
    "Oliver Wendell Holmes",
    "Oscar Wilde",
    "Paul Laurence Dunbar",
    "Percy Bysshe Shelley",
    "Philip Freneau",
    "Phillis Wheatley",
    "Ralph Waldo Emerson",
    "Richard Crashaw",
    "Richard Lovelace",
    "Robert Browning",
    "Robert Burns",
    "Robert Herrick",
    "Robert Louis Stevenson",
    "Robert Southey",
    "Robinson",
    "Rupert Brooke",
    "Samuel Coleridge",
    "Samuel Johnson",
    "Sarah Flower Adams",
    "Sidney Lanier",
    "Sir John Suckling",
    "Sir Philip Sidney",
    "Sir Thomas Wyatt",
    "Sir Walter Raleigh",
    "Sir Walter Scott",
    "Stephen Crane",
    "Thomas Campbell",
    "Thomas Chatterton",
    "Thomas Flatman",
    "Thomas Gray",
    "Thomas Hood",
    "Thomas Moore",
    "Thomas Warton",
    "Walt Whitman",
    "Walter Savage Landor",
    "Wilfred Owen",
    "William Allingham",
    "William Barnes",
    "William Blake",
    "William Browne",
    "William Cowper",
    "William Cullen Bryant",
    "William Ernest Henley",
    "William Lisle Bowles",
    "William Morris",
    "William Shakespeare",
    "William Topaz McGonagall",
    "William Vaughn Moody",
    "William Wordsworth"
  ]
}"#;