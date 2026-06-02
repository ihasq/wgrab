# wgrab 第8Aフェーズログ

## 結論

- RC docs: 作成
- CHANGELOG: 作成
- README: successor 方針追記済み
- API snapshot: 取得、phase7S snapshot との差分なし
- forbidden grep: 空
- target checks: Windows / macOS x86 / macOS arm すべて成功
- push CI: commit/push 後に確認
- ready for owner RC review: push CI 確認後に判定

## Branch

- branch: `wgpu29-phase5c-marker-hardening`
- commit: `b684e64746970884e0bb5a0715e2d413f9d74049`
- status: clean at phase start

## CI

- wgpu runtime: pending after phase8a push
- wgpu linux vulkan smoke: pending after phase8a push

## API compatibility

- public item diff: `logs/phase8a_public_items_diff.txt` は空
- suspected breaking changes: none identified before phase8a checks

## Static checks

- public item snapshot: `logs/phase8a_public_items_snapshot.txt`、820 items
- forbidden grep: `logs/phase8a_forbidden_grep.txt`、空
- target check error index: `logs/phase8a_check_error_index.txt`、空
- examples rustfmt check: 成功
- workflow summary: `logs/phase8a_workflow_summary.txt`

## Package metadata

- name: `crabgrab`
- version: `0.4.0`
- repository: `https://github.com/ihasq/wgrab`
- license: `MIT OR Apache-2.0`
- publish decision: deferred

## Owner decisions

- RC tag name: `wgrab-wgpu29-rc1`
- Cargo.toml repository: update to `https://github.com/ihasq/wgrab`
- crates.io publish: deferred

## Deferred

- tag creation
- crates.io publish
- full objc2 migration
- metal API deprecation
- typed error API
- full formatting cleanup

## 判断を求めたい点

1. release candidate tag 名を `wgrab-wgpu29-rc1` と `successor-baseline-wgpu29-rc1` のどちらにするか。
2. package `repository` metadata を後継 repository に更新するか、互換性重視で現状維持するか。
3. crates.io publish 方針を既存 package 継続、rename、新規 publish 見送りのどれにするか。
