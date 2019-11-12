use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum StringStack {
    Empty,
    Cons(String, Rc<StringStack>),
}

impl StringStack {
    pub fn copy(&self) -> StringStack {
        self.copy_helper(StringStack::empty())
    }
    fn copy_helper(&self, acc: StringStack) -> StringStack {
        match self {
            StringStack::Cons(a, next) => next.copy_helper(acc.push(a.to_string())),
            StringStack::Empty => acc,
        }
    }

    pub fn empty() -> StringStack {
        StringStack::Empty
    }
    pub fn is_empty(&self) -> bool {
        match *self {
            StringStack::Cons(_, _) => false,
            StringStack::Empty => true,
        }
    }
    pub fn head(&self) -> Option<String> {
        match self {
            StringStack::Empty => None,
            StringStack::Cons(data, _) => Some(data.to_string()),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            StringStack::Empty => 0,
            StringStack::Cons(_, tail) => 1 + tail.len(),
        }
    }
    pub fn push(&self, data: String) -> StringStack {
        match self {
            StringStack::Empty => StringStack::Cons(data, Rc::new(StringStack::Empty)),
            StringStack::Cons(b, next) => StringStack::Cons(
                data,
                Rc::new(StringStack::Cons(b.to_string(), next.clone())),
            ),
        }
    }
    pub fn tail(&self) -> Option<&StringStack> {
        match self {
            StringStack::Empty => None,
            StringStack::Cons(_, next) => Some(next.as_ref()),
        }
    }

    pub fn to_string(&self) -> String {
        self.mk_string("Stack(", ", ", ")")
    }

    pub fn mk_string(
        &self,
        start: &'static str,
        separator: &'static str,
        end: &'static str,
    ) -> String {
        format!("{}{}", self.mk_string_helper(separator, start), end)
    }

    fn mk_string_helper(&self, separator: &'static str, start: &'static str) -> String {
        match self {
            StringStack::Empty => format!("{}", start),
            StringStack::Cons(data, next) => {
                let stack = next.as_ref();
                match stack {
                    StringStack::Empty => format!("{}{}", start, data),
                    StringStack::Cons(_, _) => format!(
                        "{}{}{}",
                        next.as_ref().mk_string_helper(separator, start),
                        separator,
                        data,
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn stack_abc() -> StringStack {
        StringStack::empty()
            .push("a".to_owned())
            .push("b".to_owned())
            .push("c".to_owned())
            .push("d".to_owned())
    }

    #[test]
    fn sharing_with_immutable_cons_compiles() {
        let stack = stack_abc();
        let _x = stack.push("100".to_owned());
        let _y = stack.push("200".to_owned());
    }

    #[test]
    fn shows_its_strings() {
        let stack = stack_abc();
        assert_eq!("Stack(a, b, c, d)", format!("{}", stack.to_string()))
    }

    #[test]
    fn mk_string_shows_correct() {
        let stack = stack_abc();
        assert_eq!(
            "[a, b, c, d]",
            format!("{}", stack.mk_string("[", ", ", "]"))
        )
    }
}