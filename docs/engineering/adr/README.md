# Architecture Decision Records (ADR)

> ADR은 프로젝트의 주요 기술 결정을 문서화한다. 결정의 컨텍스트, 근거, 대안, 결과를 기록하여 미래의 팀원이 "왜 이렇게 결정했는가"를 이해할 수 있게 한다.

---

## ADR 인덱스

| # | 결정 | 상태 | 문서 |
|---|------|------|------|
| ADR-001 | Tauri 선택 (not Electron) | **Accepted** | [architecture.md § ADR-001](../architecture.md#adr-001-tauri-선택--electron-대신-tauri를-desktop-framework로-사용) |
| ADR-002 | xterm.js 선택 (터미널 렌더러) | **Proposed** | [architecture.md § ADR-002](../architecture.md#adr-002-xtermjs-선택--터미널-에뮬레이터-렌더링-라이브러리) |
| ADR-003 | SSH 처리를 Rust Core에서 수행 | **Accepted** | [architecture.md § ADR-003](../architecture.md#adr-003-ssh-처리를-rust-core에서-수행) |

> **참고**: 현재 ADR은 [architecture.md](../architecture.md)에 통합되어 있다. 향후 ADR이 추가되면 이 폴더에 개별 파일로 작성한다.

---

## ADR 템플릿

새 ADR 작성 시 아래 템플릿을 따른다:

```markdown
# ADR-NNN: [결정 제목]

**상태**: Proposed | Accepted | Deprecated | Superseded by ADR-NNN

**날짜**: YYYY-MM-DD

## 컨텍스트
[어떤 문제를 해결해야 하는가? 어떤 제약 조건이 있는가?]

## 결정
[무엇을 결정했는가?]

## 근거
[왜 이 결정을 내렸는가? 어떤 기준으로 평가했는가?]

## 대안 및 기각 사유
| 대안 | 기각 사유 |
|------|-----------|
| ... | ... |

## 결과 (Consequences)
[이 결정으로 인해 발생하는 긍정적/부정적 영향]

## 참조
[관련 링크, 벤치마크, 문서]
```

---

## 상태 정의

| 상태 | 의미 |
|------|------|
| **Proposed** | 검토 중. 스파이크 결과 또는 팀 논의 대기 |
| **Accepted** | 확정. 구현에 반영 |
| **Deprecated** | 더 나은 대안 발견. 참고용으로 유지 |
| **Superseded** | 새 ADR로 대체됨. `Superseded by ADR-NNN` 기재 |

---

**Last Updated**: 2026-02-07
