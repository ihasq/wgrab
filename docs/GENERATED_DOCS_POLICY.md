# Generated docs policy

## Goal

Avoid stale generated documentation in the repository unless there is a clear reason to track it.

## Policy options

### Option A: regenerate before breaking release

Use if generated docs are intentionally tracked.

Requirements:

- document generation command
- commit regenerated docs
- ensure docs match current public API

### Option B: remove generated docs

Use if docs are stale snapshots from the archived upstream or are not maintained.

Requirements:

- confirm they are not used by GitHub Pages or release process
- remove in a dedicated cleanup phase
- update README links

### Option C: keep temporarily

Use if ownership is unclear.

Requirements:

- mark as stale / pending review
- do not treat as release docs

## Phase 18P decision

No generated docs are deleted in Phase 18P.

They are audited and classified only.

## Current generated docs candidates

| Path | Size | Decision |
|---|---:|---|
| `docs/macos_docs` | 5.3M | keep temporarily, pending generated docs ownership review |
| `docs/windows_docs` | 4.8M | keep temporarily, pending generated docs ownership review |

## Phase 20P decision

Generated docs are kept temporarily.

Current classification:

- `docs/macos_docs`: stale pending review
- `docs/windows_docs`: stale pending review

No generated docs are removed or regenerated in Phase 20P.
