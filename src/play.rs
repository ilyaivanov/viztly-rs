use std::sync::atomic;

fn main() {
    let mut root = Item {
        id: get_id(),
        children: vec![
            item("One"),
            item("Two"),
            item("Three"),
            item("Four"),
            item("Five"),
            item("Six"),
        ],
        title: String::from("Root"),
    };

    for item in &root.children {
        println!("{}", item.title);
    }

    append_title_on_root(&mut root, 3, "     ----- modifying?");
    // append_title(&mut items, 4, "     ----- modifying!!!");

    println!("");
    println!("");
    println!("");

    for item in &root.children {
        println!("{}", item.title);
    }
}

fn append_title_on_root(root: &mut Item, id: usize, title: &str) {
    let item = find_item(&mut root.children, id);

    if !item.title.starts_with("T") {
        item.title.push_str(title);
    } else {
        item.title.push_str("     ------ oh no!!!");
    }
}

fn find_item(items: &mut Vec<Item>, id: usize) -> &mut Item {
    for item in items {
        if item.id == id {
            return item;
        }
    }
    panic!("Couldn't find item with id {}", id);
}

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
}

fn item(title: &str) -> Item {
    Item {
        id: get_id(),
        title: String::from(title),
        children: Vec::new(),
    }
}

struct Item {
    id: usize,
    title: String,
    children: Vec<Item>,
}
