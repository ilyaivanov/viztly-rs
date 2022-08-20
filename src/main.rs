use crossterm::execute;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue, style, terminal, Result,
};
use std::io::{self, Stdout, Write};
use std::sync::atomic;

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
}

fn item_leaf(title: &str) -> Item {
    item_with_children(title, Vec::new())
}

fn item_with_children(title: &str, children: Vec<Item>) -> Item {
    Item {
        id: get_id(),
        title: String::from(title),
        is_open: children.len() > 0,
        children,
    }
}
struct Item {
    id: usize,
    title: String,
    is_open: bool,
    children: Vec<Item>,
}

struct Tree {
    root: Item,
    selected_item: usize,
}

fn main() -> Result<()> {
    let mut w = io::stdout();

    let root = item_with_children(
        "Root",
        vec![
            item_with_children(
                "Item 1",
                vec![
                    item_leaf("Item 1.1"),
                    item_with_children(
                        "Item 1.2",
                        vec![
                            item_leaf("Item 1.2.1"),
                            item_leaf("Item 1.2.2"),
                            item_leaf("Item 1.2.3"),
                            item_leaf("Item 1.2.4"),
                        ],
                    ),
                    item_leaf("Item 1.3"),
                ],
            ),
            item_leaf("Item 2"),
            item_leaf("Item 3"),
            item_leaf("Item 4"),
            item_leaf("Item 5"),
            item_leaf("Item 6"),
        ],
    );
    let first_id = root.children[0].id;
    let mut tree = Tree {
        root,
        selected_item: first_id,
    };

    queue!(
        w,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    loop {
        queue!(
            w,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0),
        )?;

        fn traverse(w: &mut Stdout, tree: &Tree, items: &Vec<Item>, level: usize) -> Result<()> {
            for item in items {
                print_item(w, tree, item, level)?;

                if item.is_open {
                    traverse(w, tree, &item.children, level + 1)?;
                }
            }
            Ok(())
        }

        fn print_item(w: &mut Stdout, tree: &Tree, item: &Item, level: usize) -> Result<()> {
            let selected_color = style::Color::Rgb {
                r: 29,
                g: 31,
                b: 35,
            };

            if tree.selected_item == item.id {
                queue!(w, style::SetBackgroundColor(selected_color))?;
            }

            let circle = if item.children.is_empty() {
                "○"
            } else {
                "●"
            };
            println!(
                " {}{} {}{}",
                "  ".repeat(level),
                circle,
                item.title,
                "  ".repeat(30),
            );

            if tree.selected_item == item.id {
                queue!(w, style::ResetColor)?;
            }

            Ok(())
        }

        traverse(&mut w, &tree, &tree.root.children, 0)?;

        w.flush()?;
        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        } else if event == Event::Key(KeyCode::Down.into()) {
            tree.selected_item += 1;
        } else if event == Event::Key(KeyCode::Up.into()) {
            if tree.selected_item > 0 {
                tree.selected_item -= 1;
            }
        }
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    Ok(())
}
