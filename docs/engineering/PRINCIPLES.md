# Engineering Principles — 엔지니어링 원칙

> **역할**: Backend Engineer (Rust), Frontend Engineer (TypeScript), Full-stack
> **적용 범위**: 아키텍처 설계, 코드 작성, 기술 의사결정

---

## 핵심 원칙

### 1. Trust Boundary 준수

Tauri의 신뢰 경계 모델을 절대적으로 준수한다:

| 영역 | 권한 | 예시 |
|------|------|------|
| **Rust Core** (Trusted) | 시스템 리소스 접근 가능 | SSH 연결, 파일 접근, OS Keystore, 프로세스 실행 |
| **Web Frontend** (Sandboxed) | UI 렌더링만 가능 | Grid Layout, xterm.js, File Browser UI |
| **IPC Bridge** | 두 영역 간 통신 | Tauri Commands (FE→BE), Events (BE→FE) |

**위반 금지**:
- Frontend에서 시스템 리소스 직접 접근
- IPC 우회 (직접 소켓, WebSocket 등)
- Rust Core에서 직접 DOM 조작

### 2. IPC Only Communication

Frontend↔Backend 통신은 반드시 Tauri IPC만 사용한다:

```
Frontend → Backend: invoke('command_name', { params })
Backend → Frontend: emit('event_name', payload)
Frontend 수신:      listen('event_name', callback)
```

### 3. SSH 보안 원칙

| 규칙 | 상세 |
|------|------|
| SSH 키 경로만 저장 | Workset JSON에 키 **내용**을 절대 저장하지 않음 (NFR-12) |
| 비밀번호 OS Keystore 전용 | macOS Keychain, Linux Secret Service, Windows Credential Manager (NFR-13) |
| 민감 정보 Frontend 노출 금지 | SSH 키, 비밀번호는 Rust Core에서만 처리 |

### 4. ADR 기반 기술 결정

주요 기술 결정은 반드시 ADR(Architecture Decision Record)로 문서화한다:

| 상태 | 의미 |
|------|------|
| **Proposed** | 검토 중, 스파이크 결과 대기 |
| **Accepted** | 확정, 구현 진행 |
| **Deprecated** | 더 나은 대안으로 대체됨 |
| **Superseded** | 새 ADR로 대체됨 |

현재 확정된 ADR: [ADR 인덱스](./adr/README.md)

### 5. 스파이크 우선 검증

불확실한 기술 선택은 MVP 구현 전에 반드시 스파이크로 검증한다:

| 스파이크 | 우선순위 | 상태 |
|----------|----------|------|
| SPIKE-1: Tauri + xterm.js Latency | CRITICAL | 대기 |
| SPIKE-2: SSH 연결 풀링 | HIGH | 대기 |
| SPIKE-3: 이기종 VM 호환성 | MEDIUM | 대기 |

스파이크 실패 시 대안을 즉시 검토한다 (Decision Point 문서화).

### 6. 코드 품질 원칙

| 금지 사항 | 이유 |
|-----------|------|
| `as any`, `@ts-ignore`, `@ts-expect-error` | 타입 안전성 파괴 |
| 빈 catch 블록 `catch(e) {}` | 에러 은폐 |
| 테스트 삭제로 "통과" | 품질 보증 파괴 |
| TODO 주석 방치 | 기술 부채 누적 |

### 7. 성능 기준 준수

| NFR | 기준 | 검증 방법 |
|-----|------|-----------|
| NFR-1 | SSH 연결 ≤2초 (로컬) / ≤5초 (인터넷) | 타이머 측정 |
| NFR-2 | 10,000줄 출력 <100ms 지연 | 벤치마크 |
| NFR-3 | 패인 리사이즈 <50ms | 프레임 측정 |
| NFR-8 | 자동 재접속 ≥90%, 15초 이내 | 스트레스 테스트 |
| NFR-10 | Workset 활성화 ≤10초 (4VM 2x2) | E2E 측정 |

---

## 컴포넌트 소유권

| 컴포넌트 | 영역 | 담당 |
|----------|------|------|
| SSH Connection Manager | Rust Core | Backend |
| Process Manager | Rust Core | Backend |
| Resource Poller | Rust Core | Backend |
| Workset Store | Rust Core | Backend |
| File Access Layer | Rust Core | Backend |
| IPC Bridge | Tauri | Backend + Frontend |
| Grid Layout Engine | Frontend | Frontend |
| Terminal Emulator UI | Frontend | Frontend |
| Workset Manager UI | Frontend | Frontend |

---

## 참조 문서

| 문서 | 경로 | 용도 |
|------|------|------|
| Architecture | [architecture.md](./architecture.md) | C4 다이어그램, 컴포넌트, 리스크 |
| ADR Index | [adr/README.md](./adr/README.md) | 아키텍처 결정 기록 |
| Spikes | [spikes/README.md](./spikes/README.md) | 기술 스파이크 추적 |
| Glossary | [glossary.md](../glossary.md) | 용어 정의 |

---

**Last Updated**: 2026-02-07
