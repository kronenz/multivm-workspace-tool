# SPIKE-1 실행 계획: Tauri + xterm.js Latency 검증

## Document Information

| Field | Value |
|------|-------|
| Document Type | Technical Spike Plan |
| Spike ID | SPIKE-1 |
| Title | Tauri + xterm.js Latency |
| Priority | CRITICAL |
| Status | Draft |
| Date | 2026-02-07 |
| Related Risk | RISK-1 (Tauri WebView + xterm.js latency), RISK-4 (IPC 병목) |
| Related Docs | `docs/engineering/spikes/README.md`, `docs/engineering/architecture.md`, `docs/qa/mvp-spec.md` |

---

## 목적 (Objective)

Tauri(WebView) 환경에서 xterm.js(가능하면 WebGL 렌더러)의 터미널 UX가 **MVP 요구 성능**을 충족하는지 검증한다.

- 사용자 체감 핵심: 키 입력 응답성, 대량 출력 렌더링, TUI(vim/htop) 호환성
- 기술 검증 핵심: WebView별 WebGL 성능/호환성, IPC 경로의 오버헤드

---

## 성공 기준 (Success Criteria)

아래 기준을 **3개 OS 모두에서** 만족해야 “성공”으로 판정한다.

- [ ] 키 입력 응답 <50ms (권장 지표: p95)
- [ ] 10,000줄 출력 후 스크롤/렌더링 지연 <100ms (권장 지표: p95)
- [ ] vim, htop, nano 정상 동작 (키 입력 누락/화면 깨짐/커서 이상 없음)
- [ ] WebGL 렌더러 동작 확인 (불가 시 Canvas 폴백 시나리오도 “명확히” 정리)

---

## 범위 (Scope)

### In Scope

- 단일 터미널(1 pane)에서의 입력/출력/스크롤 성능
- 다중 터미널(최소 4개, 권장 10개) 동시 출력 시 성능 하락 폭
- WebGL 렌더러 vs Canvas 폴백의 성능/호환성 비교
- IPC 경로(Backend -> Frontend) 포함/제외 비교(가능한 경우)

### Out of Scope

- Workset CRUD / Grid Layout / File Browser / Markdown Viewer / Resource Poller UI
- 장시간(수 시간) 메모리 누수 분석(단, 30분 러닝에서 급격한 증가 여부는 관찰)
- SSH 라이브러리/재접속 정책의 품질(이는 SPIKE-2 주관)

---

## 테스트 매트릭스 (Test Matrix)

| Dimension | Values |
|---|---|
| OS | macOS, Windows, Ubuntu Linux |
| WebView | WKWebView / WebView2 / WebKitGTK |
| Renderer | WebGL (기본), Canvas (폴백) |
| Terminal Count | 1, 4, 10 |
| Output Mode | 유휴, 소량 스트림, 대량(10,000줄) |

---

## 측정 항목 및 수집 방법 (Metrics)

### 핵심 지표 정의

| Metric | 정의 | Pass/Fail 기준 |
|---|---|---|
| Key Input Latency | 키 입력 이벤트 -> 화면에 반영되었다고 판단되는 시점까지 | p95 < 50ms |
| Large Output Render/Scroll | 10,000줄 출력 후 스크롤/렌더링 반응 시간 | p95 < 100ms |
| TUI Compatibility | vim/htop/nano 사용 중 이상 증상 유무 | 치명 이슈 0건 |
| CPU Overhead (Local) | 출력 스트림/렌더링 중 로컬 CPU 사용률 | 관찰/기록(판정은 “이상 급등” 여부) |
| Memory Footprint | 1/4/10 터미널 기준 메모리 | 관찰/기록(급격한 증가/누수 징후 체크) |

### 데이터 기록 테이블 (작성용)

| OS | Renderer | Terminal Count | Key Latency p50/p95/p99 (ms) | Large Output p50/p95 (ms) | TUI 결과 | CPU(대략) | Mem(대략) | Notes |
|---|---|---:|---|---|---|---|---|---|
| macOS | WebGL | 1 |  |  |  |  |  |  |
| macOS | WebGL | 4 |  |  |  |  |  |  |
| macOS | WebGL | 10 |  |  |  |  |  |  |
| macOS | Canvas | 4 |  |  |  |  |  |  |
| Windows | WebGL | 1 |  |  |  |  |  |  |
| Windows | WebGL | 4 |  |  |  |  |  |  |
| Windows | WebGL | 10 |  |  |  |  |  |  |
| Ubuntu | WebGL | 1 |  |  |  |  |  |  |
| Ubuntu | WebGL | 4 |  |  |  |  |  |  |
| Ubuntu | WebGL | 10 |  |  |  |  |  |  |
| Ubuntu | Canvas | 4 |  |  |  |  |  |  |

---

## 테스트 시나리오 (Test Scenarios)

### S1. 키 입력 응답성 (Interactive Typing)

- [ ] 일반 타이핑(연속 입력)에서 입력 누락/지연 체감이 없는지 확인
- [ ] TUI(vim)에서 방향키/모드 전환 시 키 입력이 밀리지 않는지 확인
- [ ] 터미널 개수 증가(1 -> 4 -> 10) 시 지연 증가폭 기록

### S2. 대량 출력 (10,000 lines)

- [ ] 10,000줄 수준의 출력이 발생했을 때 UI 프리즈 여부 확인
- [ ] 스크롤/페이지 업다운이 부드럽게 동작하는지 확인
- [ ] 출력 중에도 입력이 유효한지(키보드 이벤트 드랍 여부) 확인

### S3. 렌더러 호환성 (WebGL / Canvas fallback)

- [ ] WebGL 렌더러가 OS별로 실제 동작하는지 확인
- [ ] WebGL 실패/비활성 시 Canvas 폴백이 기능적으로 문제가 없는지 확인
- [ ] WebGL vs Canvas 성능 차이를 기록(특히 Ubuntu)

---

## 판정 기준 (Decision)

| Outcome | 조건 | 다음 단계 |
|---|---|---|
| 성공 | 성공 기준 전부 Pass | ADR-002(xterm.js) “Accepted”로 승격 후보 |
| 부분 성공 | OS 1개에서만 경계치 초과 / Canvas 폴백만 가능 | IPC 최적화/출력 배치/렌더러 강제 전략을 MVP 설계에 반영 |
| 실패 | 키 입력/대량 출력/ TUI 중 1개 이상이 실사용 불가 수준 | 터미널 대안(hterm/커스텀), 프레임워크 재검토(Electron 포함) |

---

## 체크리스트 (Execution Checklist)

- [ ] 3개 OS에서 동일한 테스트 매트릭스 수행
- [ ] 최소 1회는 10터미널 동시 출력 부하를 수행
- [ ] 결과 테이블을 모두 채우고, “실패 모드”를 명확히 기록
- [ ] 결론에 “MVP에서 강제할 기본 렌더러/폴백 정책”을 한 줄로 명시

---

**Last Updated**: 2026-02-07
