use std::rc::Rc;

#[derive(Clone, Default)]
pub enum Inner<T> {
    #[default]
    Nil,
    Cons(T, List<T>, usize),
}

/// Iteratively drop a `List<T>` to avoid stack overflow on long lists.
///
/// The default recursive drop of `Rc<Inner<T>>` can overflow the stack when
/// the list has thousands of nodes (e.g., A* parse results for large files).
/// This function walks the list iteratively, unlinking each node before it is
/// freed so that no stack frame chains are created.
pub fn drop_list<T>(list: List<T>) {
    let mut current = list;
    loop {
        // If we're the sole owner, move the inner value out so we can
        // mutate it before it is freed.
        match Rc::try_unwrap(current) {
            Err(_) => break, // Other Rc clones exist; let them handle drop.
            Ok(inner) => {
                match inner {
                    Inner::Nil => break,
                    Inner::Cons(_item, tail, _) => {
                        // Continue with the tail.  When this loop iteration
                        // ends, `_item` is dropped (non-recursive because it's
                        // a plain `T`, not a `List`), and the old `inner`
                        // (which now owns only `_item`) is freed — with its
                        // `tail` field already moved out, so no chain.
                        current = tail;
                    }
                }
            }
        }
    }
}
impl<T: PartialEq> PartialEq for Inner<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cons(t1, c1, l1), Self::Cons(t2, c2, l2)) => {
                if l1 != l2 || t1 != t2 {
                    return false;
                }
                if Rc::ptr_eq(c1, c2) {
                    return true;
                }
                c1 == c2
            }
            (Inner::Nil, Inner::Nil) => true,
            _ => false,
        }
    }
}
impl<T: Eq> Eq for Inner<T> {}
impl<T: std::fmt::Debug> std::fmt::Debug for Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "[ ]"),
            Self::Cons(item, cons, _) => {
                write!(f, "[ {:?}", item)?;
                for x in cons.iter() {
                    write!(f, ", {:?}", x)?;
                }
                write!(f, " ]")?;
                Ok(())
            }
        }
    }
}

pub type List<T> = Rc<Inner<T>>;

impl<T> Inner<T> {
    pub fn new() -> Rc<Self> {
        Rc::new(Inner::Nil)
    }

    pub fn len(&self) -> usize {
        match self {
            Inner::Nil => 0,
            Inner::Cons(_, _, x) => *x,
        }
    }

    pub fn prepend(self: &List<T>, value: T) -> List<T> {
        let at = match self.as_ref() {
            Inner::Nil => 1,
            Inner::Cons(_, _, at) => at + 1,
        };
        Rc::new(Inner::Cons(value, self.clone(), at))
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            Inner::Nil => None,
            Inner::Cons(h, _, _) => Some(h),
        }
    }

    pub fn tail(&self) -> Option<&List<T>> {
        match &self {
            Inner::Nil => None,
            Inner::Cons(_, t, _) => Some(t),
        }
    }
    pub fn slice(&self) -> Option<(&T, &List<T>)> {
        match &self {
            Inner::Nil => None,
            Inner::Cons(r, t, _) => Some((r, t)),
        }
    }

    pub fn same(self: &List<T>, other: &List<T>) -> bool {
        Rc::ptr_eq(self, other)
    }

    pub fn iter<'a>(self: &'a List<T>) -> impl Iterator<Item = &'a T> {
        let mut list = self;
        std::iter::from_fn(move || match list.as_ref() {
            Inner::Nil => None,
            Inner::Cons(i, r, _) => {
                list = r;
                Some(i)
            }
        })
    }
}
