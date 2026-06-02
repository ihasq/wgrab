# wgrab 第9Aフェーズログ

## 結論

- repository metadata: `https://github.com/ihasq/wgrab` に更新
- RC tag name: `wgrab-wgpu29-rc1` を候補として固定、tag は未作成
- crates.io publish: deferred
- target checks: Windows / macOS x86 / macOS arm すべて成功
- forbidden grep: 空
- push CI: commit/push 後に確認
- ready for owner tag decision: CI 確認後に判定

## Branch

- branch: `wgpu29-phase5c-marker-hardening`
- commit: `e2185eb0de4f1a11c3f80fe088c766a50648951b`
- status: clean at phase start

## Owner decisions

- tag candidate: `wgrab-wgpu29-rc1`
- repository: `https://github.com/ihasq/wgrab`
- package name: `crabgrab`
- version: `0.4.0`
- crates.io publish: deferred

## CI

- wgpu runtime: pending after phase9a push
- wgpu linux vulkan smoke: pending after phase9a push

## API compatibility

- public API changes: none planned in phase9a
- forbidden grep: `logs/phase9a_forbidden_grep.txt` は空

## Static checks

- Windows target check: 成功
- macOS x86 target check: 成功
- macOS arm target check: 成功
- error index: `logs/phase9a_check_error_index.txt` は空
- examples rustfmt check: 成功

## Deferred

- tag creation
- crates.io publish
- package rename decision
- version bump decision
- owner transfer / crate ownership decision
- full objc2 migration
- public metal API deprecation
- typed error API redesign
- full formatting cleanup

## 判断を求めたい点

1. tag `wgrab-wgpu29-rc1` の作成可否。
2. GitHub release draft を第10Aで作るか。
3. crates.io publish 方針検討を第10A後に開始するか。
