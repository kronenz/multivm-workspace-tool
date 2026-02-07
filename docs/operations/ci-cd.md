# CI/CD Pipeline — CI/CD 파이프라인 설계

> **상태**: Skeleton (구현 시 상세화 예정)
> **목적**: 빌드, 테스트, 배포 자동화 파이프라인 정의

---

## 파이프라인 개요

```
[Commit] → [Lint/Format] → [Unit Test] → [Build] → [Integration Test] → [E2E] → [Release]
```

---

## 단계별 정의

### 1. Lint & Format (모든 커밋)

| 도구 | 대상 | 명령어 (예정) |
|------|------|---------------|
| `rustfmt` | Rust 코드 | `cargo fmt --check` |
| `clippy` | Rust 린트 | `cargo clippy -- -D warnings` |
| `eslint` | TypeScript | `npx eslint src/` |
| `prettier` | TS/CSS/HTML | `npx prettier --check src/` |

### 2. Unit Test (PR)

| 대상 | 명령어 |
|------|--------|
| Rust Core | `cargo test` |
| Frontend | `npm test` |

### 3. Build (PR)

| OS | 명령어 |
|----|--------|
| macOS (x64, arm64) | `npm run tauri build` |
| Ubuntu (x64) | `npm run tauri build` |
| Windows (x64) | `npm run tauri build` |

### 4. Integration & E2E (main 머지)

| 테스트 | 환경 |
|--------|------|
| IPC 통합 | 3 OS Matrix |
| SSH 통합 | Mock SSH 서버 |
| E2E | 3 OS × Playwright/tauri-driver |

### 5. Release (태그 생성)

```yaml
# 예상 릴리스 워크플로우
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        os: [macos-latest, ubuntu-latest, windows-latest]
    steps:
      - Checkout
      - Setup Rust + Node.js
      - Install dependencies
      - Build (npm run tauri build)
      - Sign (macOS notarize, Windows sign)
      - Upload artifacts

  release:
    needs: build
    steps:
      - Create GitHub Release
      - Attach artifacts
      - Update changelog
```

---

## GitHub Actions Matrix

| OS | Runner | 아키텍처 | 출력물 |
|----|--------|----------|--------|
| macOS | `macos-latest` | x64, arm64 | `.dmg`, `.app` |
| Ubuntu | `ubuntu-latest` | x64 | `.deb`, `.AppImage` |
| Windows | `windows-latest` | x64 | `.msi`, `.exe` |

---

## 보안

| 항목 | 방법 |
|------|------|
| macOS 코드 서명 | Apple Developer Certificate (GitHub Secrets) |
| macOS 공증 | `xcrun notarytool` |
| Windows 서명 | Code Signing Certificate (GitHub Secrets) |
| 의존성 감사 | `cargo audit` + `npm audit` (주간 스케줄) |

---

## 참조

| 문서 | 경로 |
|------|------|
| Operations Principles | [PRINCIPLES.md](./PRINCIPLES.md) |
| Architecture | [architecture.md](../engineering/architecture.md) |

---

**Last Updated**: 2026-02-07
