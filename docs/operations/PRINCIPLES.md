# Operations Principles — 운영 원칙

> **역할**: DevOps Engineer, Release Engineer
> **적용 범위**: 빌드, 배포, CI/CD, 릴리스 프로세스

---

## 핵심 원칙

### 1. 크로스 플랫폼 빌드

모든 릴리스는 3개 OS 동시 빌드를 지원한다:

| OS | 아키텍처 | 빌드 대상 | 우선순위 |
|----|----------|-----------|----------|
| macOS 11+ | x64, arm64 | `.dmg`, `.app` | Primary |
| Ubuntu 20.04+ | x64 | `.deb`, `.AppImage` | Secondary |
| Windows 10+ | x64 | `.msi`, `.exe` | Tertiary |

### 2. 번들 크기 제한

Tauri 선택의 핵심 이점을 유지한다:

| 메트릭 | 목표 | 근거 |
|--------|------|------|
| 번들 크기 | <10MB | ADR-001 (Tauri vs Electron) |
| 메모리 사용 | 30-50MB (기본) | ADR-001 |
| 메모리 사용 | <200MB (10세션) | SPIKE-2 기준 |

번들 크기가 10MB를 초과하면 의존성을 검토한다.

### 3. CI/CD 자동화

| 단계 | 자동화 항목 |
|------|-------------|
| **Commit** | Lint, Format, Type Check |
| **PR** | 단위 테스트, 통합 테스트, 빌드 검증 |
| **Merge** | 전체 테스트 스위트, 3 OS 빌드 |
| **Release** | 서명, 공증(macOS), 배포 |

### 4. 릴리스 프로세스

```
1. 버전 태그 생성 (Semantic Versioning: vMAJOR.MINOR.PATCH)
2. 3 OS 빌드 (CI/CD)
3. macOS 코드 서명 + 공증 (notarization)
4. GitHub Release 생성
5. Changelog 업데이트
6. 커뮤니티 공지 (Discord, GitHub Discussions)
```

### 5. 보안 운영

| 영역 | 원칙 |
|------|------|
| 의존성 | 정기적 보안 감사 (`cargo audit`, `npm audit`) |
| 빌드 | 재현 가능한 빌드 (deterministic builds) |
| 배포 | 코드 서명 필수 (macOS, Windows) |
| 키 관리 | CI/CD 시크릿으로 서명 키 관리 |

### 6. 모니터링 및 피드백

| 채널 | 용도 |
|------|------|
| GitHub Issues | 버그 리포트, 기능 요청 |
| GitHub Discussions | 질문, 아이디어 |
| Crash Reporter | 앱 크래시 수집 (opt-in) |
| Telemetry | 사용 패턴 (opt-in, 익명화) |

---

## 인프라 구성 (예정)

| 서비스 | 용도 | 대안 |
|--------|------|------|
| GitHub Actions | CI/CD | - |
| GitHub Releases | 배포 | - |
| Code Signing | macOS/Windows 서명 | - |

---

## 참조 문서

| 문서 | 경로 | 용도 |
|------|------|------|
| CI/CD | [ci-cd.md](./ci-cd.md) | CI/CD 파이프라인 설계 |
| Architecture | [architecture.md](../engineering/architecture.md) | 빌드 대상 정의 |
| Contributing | [CONTRIBUTING.md](../../CONTRIBUTING.md) | 기여 가이드 |
| Glossary | [glossary.md](../glossary.md) | 용어 정의 |

---

**Last Updated**: 2026-02-07
