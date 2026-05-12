# Rust Idiomatic Checklist

- Are my types expressing meaningful domain concepts?
- Are invalid states prevented or merely detected late?
- Did I use `Result` and enums to preserve information?
- Is the code explicit without becoming noisy?
- Are function boundaries small and cohesive?
- Did I use iterators where they clarify transformation?
- Did I avoid trait abstraction unless a capability boundary exists?
- Did I avoid inheritance-shaped design?
- Did I keep mutable state localized?
- Does the code read like Rust or like another language translated into Rust syntax?
