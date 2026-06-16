# AGENTS.md

## Tool name mappings

Prompts use generic names; use the installed binaries:

- `ir-dry-checker` → `gherkin-ir-dry-checker`

`gherkin-ir-dry-checker` takes JSON IR from `gherkin-parser`, not the `.feature`:

```
gherkin-parser <feature> <ir.json>
gherkin-ir-dry-checker [--include-exact] <ir.json> <report>
```

`findings: []` = already normalized. Also installed: `gherkin-parser`, `gherkin-mutator`.
