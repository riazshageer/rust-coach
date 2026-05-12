# Ownership Review Checklist

- Did I introduce any `.clone()` calls? Why is each one necessary?
- Could a function take `&T` or `&str` instead of owning `T` or `String`?
- Am I returning owned data because it is truly the correct boundary?
- Did I design the data flow so ownership moves are obvious?
- Am I fighting the borrow checker because my architecture is awkward?
- Did I create temporary allocations that a borrowed view could avoid?
- Did I choose simplicity over overly clever borrowing tricks?
- Does my API force callers to allocate unnecessarily?
- Am I using `Arc`, `Rc`, `Mutex`, or `RefCell` because I need them or because I am stuck?
- If ownership feels hard here, is the real problem responsibility placement?
