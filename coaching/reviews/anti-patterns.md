# Anti-Patterns

## Common OOP-To-Rust Mistakes

### Unnecessary getters and setters

If every field gets a getter by default, the type probably has no real invariants. In Rust, access patterns should exist to preserve meaning, not to imitate class encapsulation rituals.

### Weak wrapper structs

A wrapper that only renames a primitive without enforcing invariants, clarifying behavior, or improving ergonomics is usually ceremony, not design.

### Inheritance-style thinking

Rust does not want class hierarchies hidden behind traits. Start with data and capabilities. Add traits only when multiple concrete types truly share behavior needed by callers.

### Trait abuse

Traits are not interfaces to sprinkle everywhere. They are capability contracts. If there is only one implementation and no useful abstraction pressure, a trait may be unnecessary.

### `Arc<Mutex<T>>` overuse

This often appears when architecture is unclear. First ask whether shared mutable state is actually needed. Many problems disappear when ownership is rethought.

### Excessive cloning

`.clone()` can be correct, but it is also a common escape hatch. If cloning keeps appearing, inspect the data flow and API boundaries.

### Service-container thinking

Rust applications usually do not need heavyweight dependency injection patterns. Pass the few dependencies explicitly and keep construction simple.

### Java-style layering

Not every project needs a controller-service-repository stack. Layers should arise from real concerns, not imported habits.

### Over-abstraction

If a beginner cannot explain why the abstraction exists, it is probably too early. Rust rewards precise abstractions, not maximal abstraction.

### Procedural orchestration

When one function manually coordinates every tiny step, the architecture is probably under-modeled. Good types reduce orchestration noise.

### Runtime polymorphism abuse

Reach for enums or concrete types first. Dynamic dispatch is useful, but not as the default architecture.
