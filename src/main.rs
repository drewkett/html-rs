use std::fmt;

trait HTMLElement: fmt::Display {}
trait BodyElement: HTMLElement {}

struct Body {
    children: Vec<Box<BodyElement>>,
}

fn body(children: Vec<Box<BodyElement>>) -> Body {
    Body { children: children }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<body>")
            .and_then(|_| {
                self.children
                    .iter()
                    .map(|c| write!(f, "{}", c))
                    .fold(Ok(()), |b, r| b.and(r))
            })
            .and_then(|_| write!(f, "</body>"))
    }
}

macro_rules! element {
    ($a:ident,$b:ident) => {
        struct $a {
            children: Vec<Box<$b>>,
        }

        impl fmt::Display for $a {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "<{}></{}>",stringify!($a),stringify!($a))
            }
        }
        impl HTMLElement for $a {}
        impl $b for $a {}
        fn $a(children: Vec<Box<$b>>) -> Box<$a> {
            Box::new($a{children: children})
        }
    }
}

element!(span,BodyElement);
element!(div,BodyElement);
element!(p,BodyElement);
fn main() {
    // let html = html(head(vec![]), body(vec![]));
    let html = body(vec![span(vec![]), div(vec![])]);
    println!("{}", html);
}
