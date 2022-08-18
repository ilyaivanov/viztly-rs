use crossterm::execute;
pub use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent},
    queue, style, terminal, Command, Result,
};
use std::io::{self, Stdout, Write};

fn item_leaf(title: &str) -> Item {
    Item {
        title: String::from(title),
        is_open: false,
        children: Vec::new(),
    }
}

fn item_with_children(title: &str, children: Vec<Item>) -> Item {
    Item {
        title: String::from(title),
        is_open: true,
        children,
    }
}
struct Item {
    title: String,
    is_open: bool,
    children: Vec<Item>,
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

        fn traverse(w: &mut Stdout, items: &Vec<Item>, level: usize) -> Result<()> {
            for item in items {
                print_item(w, item, level)?;

                if item.is_open {
                    traverse(w, &item.children, level + 1)?;
                }
            }
            Ok(())
        }

        fn print_item(w: &mut Stdout, item: &Item, level: usize) -> Result<()> {
            if item.title == "Item 2" {
                queue!(w, style::SetForegroundColor(style::Color::Red))?;
            }

            let circle = if item.children.is_empty() {
                "○"
            } else {
                "●"
            };
            println!(" {}{} {}", "  ".repeat(level), circle, item.title);

            if item.title == "Item 2" {
                queue!(w, style::SetForegroundColor(style::Color::White))?;
            }

            Ok(())
        }

        traverse(&mut w, &root.children, 0)?;

        w.flush()?;
        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
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
