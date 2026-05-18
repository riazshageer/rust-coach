# Anti-Patterns To Watch For

- Renamed primitive: a newtype that adds a name but no guarantee.
- Trait by reflex: adding a trait because the concept "might vary later."
- Clone as escape hatch: using `.clone()` to avoid understanding ownership.
- Formatter logic creep: pushing real business logic into presentation code.
- App-layer gravity: dumping every new behavior into orchestration code.
- Cargo-cult async: introducing async because it feels more advanced.
- Layer diagram theater: reproducing architecture patterns that the app has not earned.
- Test shallowness: verifying happy paths while missing the invariant that matters.
- Boundary leakage: letting raw external data flow into trusted core types unchanged.
- Iterator golf: turning a readable loop into a puzzle.
