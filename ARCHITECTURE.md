# Architecture — Mazerion (first-dev)

## High-level overview
Mazerion is built around a single principle:

> **All calculations live in one shared engine**.  
> Every UI or integration surface calls the same calculator registry.

### Data flow
```text
CLI / GUI / TUI / API / FFI
            |
            v
      mazerion_core
   (registry + types)
            |
            v
   mazerion_calculators
 (calculator implementations)
