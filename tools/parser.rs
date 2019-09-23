use std::env;
use std::fmt;
use std::fs;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Node {
    Tree(String, Vec<Node>),
    Leaf(String),
}

impl fmt::Display for Node {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Tree(s, ..) => b.write_str(s),
            Node::Leaf(s) => b.write_str(s),
        }
    }
}

struct Parser<P> {
    state: P,
}

impl<I: Iterator<Item = char>> Parser<Peekable<I>> {
    pub fn new(state: Peekable<I>) -> Parser<Peekable<I>> {
        Parser { state }
    }

    fn eat(&mut self, ch: char) {
        assert_eq!(self.state.next(), Some(ch))
    }

    fn skip(&mut self) {
        self.state.next().expect("unexpected end of file");
    }

    fn peek(&mut self) -> char {
        *self.state.peek().expect("unexpected end of file")
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.skip();
        }
    }

    pub fn parse_word(&mut self) -> String {
        let mut word = String::new();
        self.skip_whitespace();
        loop {
            let ch = self.peek();
            if ch == ')' || ch.is_whitespace() {
                return word;
            } else {
                self.skip();
                word.push(ch);
            }
        }
    }

    pub fn parse_list(&mut self) -> Node {
        self.skip_whitespace();
        self.eat('(');
        let label = self.parse_word();
        let mut children = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                ')' => break,
                '(' => children.push(self.parse_list()),
                _ => panic!("unexpected word occurence"),
            }
        }
        self.skip_whitespace();
        self.eat(')');
        if children.is_empty() {
            Node::Leaf(label)
        } else {
            Node::Tree(label, children)
        }
    }
}

pub fn print_def(node: &Node, level: usize, arg_name: &str, arg_start: usize) {
    let indent = "    ".repeat(level);
    if let Node::Tree(node, children) = node {
        println!("{}pub mod *{}* {{", indent, node);
        let mut enumdef = vec![
            format!("{}    define_enum! {{", indent),
            format!(
                "{}        pub enum #{}# {}[{}] {{",
                indent,
                node,
                arg_name,
                arg_start + level,
            ),
        ];

        for child in children {
            match child {
                Node::Leaf(s) => {
                    enumdef.push(format!(r#"{}            "{}" => #{}#,"#, indent, s, s))
                }
                Node::Tree(s, _) => {
                    enumdef.push(format!(r#"{}            "{}" => #{}#(..),"#, indent, s, s));
                    print_def(child, level + 1, arg_name, arg_start);
                }
            }
        }

        enumdef.push(format!("{}        }}", indent));
        enumdef.push(format!("{}    }}", indent));
        println!("{}", enumdef.join("\n"));
        println!("{}}}", indent);
        println!("{}pub use self::*{}*::#{}#;", indent, node, node);
        println!();
    } else {
        panic!("dfs called for a leaf");
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let arg_name = env::args().nth(2).unwrap_or_else(|| "features".to_string());
    let arg_start = env::args().nth(3).and_then(|a| a.parse().ok()).unwrap_or(0);
    let content = fs::read_to_string(file_name).unwrap();
    let mut parser = Parser::new(content.chars().peekable());
    let list = parser.parse_list();
    print_def(&list, 0, &arg_name, arg_start);
}
