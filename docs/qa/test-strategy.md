# Test Strategy — 테스트 전략

> **상태**: Skeleton (구현 시 상세화 예정)
> **목적**: MVP 품질 보증을 위한 테스트 전략, 도구, 프로세스 정의

---

## 테스트 피라미드

```
        /  E2E  \          ← 적게, 핵심 시나리오만
       / Integration \     ← IPC, SSH 연결, 컴포넌트 간 통신
      /   Unit Tests   \   ← 많이, 빠르게, 격리된
```

| 레벨 | 범위 | 비율 (목표) | 실행 속도 |
|------|------|-------------|-----------|
| Unit | 함수/모듈 단위 | 70% | 초 단위 |
| Integration | 컴포넌트 간 | 20% | 분 단위 |
| E2E | 사용자 시나리오 | 10% | 분 단위 |

---

## 테스트 도구 (예정)

| 영역 | 도구 | 용도 |
|------|------|------|
| Rust Unit | `cargo test` | Rust Core 컴포넌트 단위 테스트 |
| Rust Integration | `cargo test` + mock SSH | IPC, SSH 통합 테스트 |
| Frontend Unit | `vitest` / `jest` | TypeScript 컴포넌트 테스트 |
| Frontend E2E | `Playwright` | 브라우저 기반 E2E |
| Tauri E2E | `tauri-driver` | 데스크톱 앱 E2E |
| Performance | Custom benchmark | NFR 검증 |

---

## 테스트 대상별 전략

### Rust Core

| 컴포넌트 | 테스트 접근 |
|----------|-------------|
| SSH Connection Manager | Mock SSH 서버로 연결/재접속/채널 멀티플렉싱 테스트 |
| Process Manager | Mock PTY로 명령 실행/출력 스트리밍 테스트 |
| Resource Poller | Mock SSH exec 출력으로 파싱 정확도 테스트 |
| Workset Store | 파일 시스템 CRUD, JSON 스키마 검증 |
| File Access Layer | Mock SFTP/SSH exec 응답으로 디렉토리 리스팅 테스트 |

### Frontend

| 컴포넌트 | 테스트 접근 |
|----------|-------------|
| Grid Layout Engine | DOM 렌더링, 리사이즈 이벤트, 프리셋 적용 |
| Terminal Emulator UI | xterm.js 초기화, 입출력 파이프 연결 |
| File Browser UI | 트리 뷰 렌더링, 폴더 확장/축소 |
| Markdown Viewer UI | MD 파싱, 코드 블록 하이라이팅 |
| Workset Manager UI | 폼 유효성 검사, CRUD 플로우 |

### E2E 시나리오

MVP Spec의 8단계 E2E 시나리오를 자동화:

1. Workset 생성 (모든 필드 입력)
2. Workset 활성화 (4VM 2x2 연결)
3. AI CLI 실행 확인
4. File Browser 열기 → MD 파일 클릭
5. Resource Monitor 값 확인
6. 네트워크 인터럽트 → Auto-Reconnect
7. Theme 전환
8. 저장 → 앱 종료 → 재시작 → 복원 확인

---

## CI 통합

| 이벤트 | 실행하는 테스트 |
|--------|----------------|
| 모든 커밋 | Lint + Format + Type Check |
| PR 생성 | Unit + Integration |
| main 머지 | Unit + Integration + E2E (3 OS) |
| 릴리스 | 전체 + 성능 벤치마크 |

---

## 참조

| 문서 | 경로 |
|------|------|
| QA Principles | [PRINCIPLES.md](./PRINCIPLES.md) |
| MVP Spec | [mvp-spec.md](./mvp-spec.md) |
| Architecture (Risks) | [architecture.md](../engineering/architecture.md) |

---

**Last Updated**: 2026-02-07
