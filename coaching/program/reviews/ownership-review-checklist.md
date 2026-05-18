# Ownership Review Checklist

- Who should own each value on this path?
- Is any clone avoiding a design decision?
- Can a temporary borrow replace long-lived ownership?
- Is a boundary forcing ownership for no good reason?
- Is lifetime complexity revealing a structural problem?
- Would moving logic or data simplify the borrow story?
