## Work Item: MemMapFs.Create should not auto-create parent directories

**Type:** Bug
**Theme:** Experiment 1: Go tech debt refactor
**Labels:** language:go, lsp:gopls, codebase:existing, scope:tech-debt, experiment:1

**Description:**

When a user calls `fs.Create("/nonexistent/path/file.txt")` on a MemMapFs, the file is created successfully and the parent directory `/nonexistent/path/` is silently auto-created. On OsFs, the same call returns an `ErrNotExist` because `os.Create` does not auto-create parent directories.

This behavioral inconsistency means tests passing on MemMapFs can fail on OsFs in production. A five-year-old open issue (#270) in afero's tracker.

**Acceptance Criteria:**

- [ ] `MemMapFs.Create("/nonexistent-dir/file.txt")` returns an `ErrNotExist` error (matching OsFs)
- [ ] `MemMapFs.OpenFile` with `O_CREATE` on a non-existent parent path also returns `ErrNotExist`
- [ ] Full test suite passes (all existing tests accounted for)
- [ ] TempFile and TempDir utility functions still work (they should ensure the temp dir exists)
- [ ] CacheOnReadFs and CopyOnWriteFs composite filesystems still work

**Tasks:**

- [ ] Blast radius: trace all Create() implementations and registerWithParent() callers
- [ ] Write RED test demonstrating the inconsistency
- [ ] Fix MemMapFs.Create to check parent dir exists before creating
- [ ] Fix TempFile/TempDir to call MkdirAll on the base directory
- [ ] Fix CacheOnReadFs.Create to ensure parent dir on the layer
- [ ] Fix lstater_test.go (implicitly relied on buggy behavior)
- [ ] Full suite verification
- [ ] Write CONCLUSION.md meditation