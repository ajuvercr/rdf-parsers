use std::rc::Rc;

#[derive(Clone, Default)]
pub enum Inner<T> {
    #[default]
    Nil,
    Cons(T, List<T>, usize),
}

/// Drops a list.  Kept for compatibility: dropping a `List` no longer
/// recurses (see the `Drop` impl of `Inner`), so a plain `drop` does the same.
pub fn drop_list<T>(list: List<T>) {
    drop(list)
}

/// Frees a chain of nodes without recursing.  The default drop of an
/// `Rc<Inner<T>>` frees the tail first, one stack frame per node, which
/// overflows on long lists (A* step lists of large files, especially in
/// debug builds).  Instead, while a node is the only owner of the next one,
/// the next node's content is moved out (leaving `Nil`, so no allocation is
/// needed) and freed here in a loop.  A tail that is shared stops the walk:
/// its other owners free it later.
impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        let Inner::Cons(_, tail, _) = self else {
            return;
        };
        let Some(next) = Rc::get_mut(tail) else {
            return;
        };
        let mut current = std::mem::take(next);
        loop {
            let next = match &mut current {
                Inner::Cons(_, tail, _) => match Rc::get_mut(tail) {
                    Some(next) => std::mem::take(next),
                    None => break,
                },
                Inner::Nil => break,
            };
            // the old node's tail now holds `Nil`, so dropping it stops there
            current = next;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn long(n: usize) -> List<usize> {
        (0..n).fold(Inner::new(), |l, i| l.prepend(i))
    }

    #[test]
    fn dropping_a_long_list_does_not_recurse() {
        // a recursive drop of a million nodes overflows the default stack
        drop(long(1_000_000));
    }

    #[test]
    fn a_shared_tail_survives_dropping_one_owner() {
        let tail = long(200_000);
        let a = tail.prepend(1);
        let b = tail.prepend(2);
        drop(tail);
        drop(a);
        assert_eq!(b.len(), 200_001);
        assert_eq!(b.iter().count(), 200_001);
        assert_eq!(b.iter().nth(1), Some(&199_999));
        assert_eq!(b.iter().last(), Some(&0));
    }
}
