# QA Principles — 품질 보증 원칙

> **역할**: QA Engineer, Test Engineer
> **적용 범위**: 테스트 계획, 수락 기준 검증, 성능 테스트

---

## 핵심 원칙

### 1. Acceptance Criteria 기반 테스트

모든 테스트는 MVP Spec의 수락 기준(AC-1 ~ AC-10)에 기반한다. 수락 기준에 없는 기능은 테스트하지 않는다.

| AC | 기능 | 체크박스 수 |
|----|------|-------------|
| AC-1 | Workset CRUD | 7 |
| AC-2 | SSH Connection | 5 |
| AC-3 | Terminal Emulator | 7 |
| AC-4 | Grid Layout | 6 |
| AC-5 | File Browser | 6 |
| AC-6 | Markdown Viewer | 5 |
| AC-7 | Resource Monitoring | 6 |
| AC-8 | AI CLI Auto-Launch | 6 |
| AC-9 | SSH Auto-Reconnect | 7 |
| AC-10 | Dark/Light Theme | 6 |
| **합계** | | **61 AC 체크박스** |

### 2. Done Criteria 추적

MVP Spec의 138개 Done Criteria 체크박스를 스프린트별로 추적한다. 모든 체크박스가 통과해야 해당 기능이 "Done"이다.

### 3. NFR 성능 기준 검증

| NFR | 테스트 방법 | 합격 기준 |
|-----|-------------|-----------|
| NFR-1 | SSH 연결 타이머 측정 | ≤2초 (로컬), ≤5초 (인터넷) |
| NFR-2 | 10K줄 cat 후 스크롤 | <100ms 지연 |
| NFR-3 | 패인 드래그 리사이즈 | <50ms 응답 |
| NFR-4 | 리소스 업데이트 간격 | 5초 ±1초 |
| NFR-8 | 네트워크 인터럽트 시뮬레이션 | ≥90% 재접속, 15초 이내 |
| NFR-10 | 4VM 2x2 Workset 활성화 | ≤10초 |

### 4. E2E 시나리오 기반 통합 테스트

MVP Spec의 E2E 시나리오(8단계)를 자동화된 통합 테스트로 구현한다:

1. Workset 생성 → 2. 활성화 (4VM 2x2) → 3. AI CLI 실행 → 4. File Browser 검증 → 5. Resource 확인 → 6. Auto-Reconnect → 7. Theme 전환 → 8. 저장/종료

### 5. 스파이크 성공 기준 검증

각 기술 스파이크의 성공 기준을 독립적으로 검증한다:

| 스파이크 | 성공 기준 |
|----------|-----------|
| SPIKE-1 | 키 입력 <50ms, 10K줄 <100ms, vim 정상 동작 |
| SPIKE-2 | 10세션 30분 유지, 재접속 ≥90%, 메모리 <200MB |
| SPIKE-3 | Ubuntu/Alpine/macOS 파싱 성공, 실패 시 N/A |

### 6. 크로스 플랫폼 검증

모든 테스트는 3개 OS에서 실행한다:

| OS | 우선순위 | WebView |
|----|----------|---------|
| macOS 11+ | Primary | WKWebView |
| Ubuntu 20.04+ | Secondary | WebKitGTK |
| Windows 10+ | Tertiary | WebView2 |

### 7. 회귀 테스트 원칙

- 버그 수정 시 반드시 재현 테스트 케이스를 먼저 작성한다
- 기존 테스트를 삭제하여 "통과"시키지 않는다
- 리팩토링 후 전체 테스트 스위트를 실행한다

---

## 테스트 분류

| 분류 | 범위 | 도구 (예정) |
|------|------|-------------|
| 단위 테스트 | Rust Core 컴포넌트 | `cargo test` |
| 통합 테스트 | IPC 통신, SSH 연결 | Tauri test utils |
| E2E 테스트 | 전체 사용자 시나리오 | Playwright / WebDriver |
| 성능 테스트 | NFR 기준 검증 | 커스텀 벤치마크 |
| 호환성 테스트 | 크로스 플랫폼, 크로스 VM | 수동 + 자동화 |

---

## 참조 문서

| 문서 | 경로 | 용도 |
|------|------|------|
| MVP Spec | [mvp-spec.md](./mvp-spec.md) | 10 기능, 138 체크박스, AC |
| Test Strategy | [test-strategy.md](./test-strategy.md) | 테스트 전략 상세 |
| Architecture | [architecture.md](../engineering/architecture.md) | 리스크, 스파이크 |
| Glossary | [glossary.md](../glossary.md) | 용어 정의 |

---

**Last Updated**: 2026-02-07
