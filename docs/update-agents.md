# Updating AGENTS.md

[AGENTS.md](../AGENTS.md) tracks a `last-synced-commit` hash. When the repo moves ahead of it, refresh the domain docs for whichever files changed.

## Procedure

1. Read the `last-synced-commit: <sha>` line near the top of [AGENTS.md](../AGENTS.md).
2. Get the current commit: `git rev-parse HEAD`.
3. If the two match, stop — nothing to do.
4. List files changed between them:
   ```
   git diff --name-only <last-synced-commit> HEAD
   ```
5. For each changed path, map it to a domain doc using the table in [AGENTS.md](../AGENTS.md) (the "Domain map" section). If a path is not mapped, either extend an existing doc or create a new one under [docs/](.) and add it to the map.
6. Skim the actual diff for the changed files (`git diff <last-synced-commit> HEAD -- <path>`) and update the affected doc(s) to reflect the new behavior. Keep each doc tight — do not duplicate content across domains.
7. If new domains were added or old ones removed, also update the "Domain map" section in [AGENTS.md](../AGENTS.md).
8. Overwrite the `last-synced-commit` value in [AGENTS.md](../AGENTS.md) with the current `HEAD` sha.
9. Commit the doc changes together.

## When to trigger

- When the user explicitly asks to refresh / sync AGENTS.md.
- Before starting a non-trivial task, as a sanity check that domain docs are current.
