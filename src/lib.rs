
extern crate email;

use std::collections::HashMap;
use email::MimeMessage;
use email::results::ParsingResult;

// Based on https://www.jwz.org/doc/threading.html

struct Container {
    message : Option<&'static Message>,
    parent : Option<Box<Container>>,
    child : Option<Box<Container>>,   // first child
    next : Option<Box<Container>>,    // next element in sibling list
}

struct Message {
    subject : String,
    message_id : Id, // the ID of this message
    references : Vec<Id>, // list of IDs of parent messages
}

#[derive(PartialEq, Eq, Hash)]
struct Id { id : i64 }

fn parse_msg(msg: &str) -> ParsingResult<Message> {
    let hdrs = MimeMessage::parse(msg)?.headers;
    return Ok(());
}

fn new_container() -> Container {
    Container { message: None, parent: None, child: None, next: None }
}

fn foo(id_table: HashMap<Id, Container>, id: &Id, msg: &Message) -> () {
    // 1. A.
    match id_table.get_mut(id) {
        Some(x) if x.message.is_none() => { x.message = Some(msg) },
        None => {
            let c = new_container();
            c.message = Some(msg);
            id_table.insert(id, c);
        },
        _ => panic!("wuh oh!"),
    };
    // 1. B.
    for reference in msg.references {
        // 1. B. i.
        let rec_c = match id_table.get_mut(reference) {
            Some(c) => c,
            None => {
                let c = new_container();
                id_table.insert(reference, c);
                c
            }
        }
        // 1. B. ii.

    }
    return ();
}
