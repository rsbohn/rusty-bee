// Randall Bohn 2023
// License: MIT
// Answer queries from /var/ndb/*
use std::io::Write;
use std::sync::{Arc, Mutex};

struct Context {
    domain: String,
    running: bool   
}
fn print_context(context: &Arc<Mutex<Context>>) {
    let guard = context.lock().unwrap();

    println!("context: domain={}, running={}", 
        guard.domain, 
        guard.running
    );
}

fn ndbquery(query: &str, domain: &String) -> String {
    format!("ndb/query {} dom={}\n", query, domain)  
}

fn read_eval_print(context: &Arc<Mutex<Context>>) {
    let mut guard = context.lock().unwrap();
    print!("ndb> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let query = input.trim();


    if query.starts_with("dom=") {
        guard.domain = query[4..].to_string();
        return
    }
    if query.starts_with("quit") {
        guard.running = false;
        return
    }

    let response = ndbquery(&query, &guard.domain);
    println!("{}", response);
 
}

fn main() {
    println!("RustyBee 0.1");

    let context = Arc::new(Mutex::new(Context {
        domain: "*".to_string(),
        running: true
    }));
    while context.lock().unwrap().running {
        read_eval_print(&context);
        print_context(&context)
    }
    println!("/SK")
}
