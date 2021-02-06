use std::collections::HashMap;

//first irst-fay
//apple apple-hay

fn main() {
    let mut table: HashMap<String, String> = HashMap::new();
    loop {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s = s.trim_end().to_owned();
            s
        };
        if executor(&s, &mut table) {
            break;
        }
    }
}

fn executor(s: &str, mut table: &mut HashMap<String, String>) -> bool {
    let commands: Vec<&str> = s.split(' ').collect();
    let order = commands[0];

    if order == "add" {
        if commands.len() >= 3 {
            let entity = commands[1];
            let depart = commands[2];
            add_entity(&mut table, entity, depart);
        }
    } else if order == "show" {
        show(&table);
    } else {
        return true;
    }
    return false;
}

fn add_entity(table: &mut HashMap<String, String>, entity: &str, depart: &str) {
    table.entry(entity.to_owned()).or_insert(depart.to_owned());
}

fn show(table: &HashMap<String, String>) {
    for (key, value) in table {
        println!("entity:{}  depart:{}", key, value);
    }
}
