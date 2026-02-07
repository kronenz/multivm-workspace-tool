# SPIKE-2 실행 계획: SSH 연결 풀링 Stress Test

## Document Information

| Field | Value |
|------|-------|
| Document Type | Technical Spike Plan |
| Spike ID | SPIKE-2 |
| Title | SSH Connection Pooling Stress Test |
| Priority | HIGH |
| Status | Draft |
| Date | 2026-02-07 |
| Related Risk | RISK-2 (다중 SSH 안정성/재접속), RISK-4 (IPC 병목) |
| Related Docs | `docs/engineering/spikes/README.md`, `docs/engineering/architecture.md`, `docs/qa/mvp-spec.md` |

---

## 목적 (Objective)

Rust Core 기반 SSH Connection Manager가 다음을 만족하는지 검증한다.

- 10개 동시 SSH 세션을 **30분 이상** 안정적으로 유지
- 채널 멀티플렉싱(PTY/exec 등 “유사 부하”) 상황에서 오류/자원 누수 없이 동작
- 네트워크 인터럽트 후 **자동 재접속 성공률 ≥90% (15초 이내)**

---

## 성공 기준 (Success Criteria)

- [ ] 10개 세션 30분 유지 (0% 비정상 종료)
- [ ] 재접속 성공률 ≥90% (측정 대상: 네트워크 인터럽트 케이스)
- [ ] 메모리 <200MB (10세션, 관찰치)
- [ ] CPU 오버헤드 <5% (keepalive + 폴링 유사 부하, 관찰치)

---

## 테스트 구성 (Test Setup)

### 대상 환경 옵션

| Option | 설명 | 장점 | 단점 |
|---|---|---|---|
| A (권장) | 10개 서로 다른 VM에 1세션씩 | 현실과 가장 유사 | 준비 비용 |
| B | 1개 VM에 10세션 동시 연결 | 준비 쉬움 | 네트워크/서버 다양성 반영 부족 |

### 세션 부하 모델 (Workload Model)

각 세션에서 아래 부하를 “동시에” 유발한다.

| Workload | 목적 | 주기/강도 |
|---|---|---|
| PTY I/O | 터미널 인터랙션/출력 스트림 유사 부하 | 지속(저/중/고 3단계) |
| Exec Polling | Resource Poller 유사 부하 | 5초 주기 |
| Channel Mix | 채널 멀티플렉싱 안정성 | PTY + exec를 동시 사용 |

---

## 측정 항목 (Metrics)

| Metric | 정의 | Pass/Fail 기준 |
|---|---|---|
| Session Uptime | 30분 동안 세션이 유지되는 비율 | 100% |
| Abnormal Termination | 크래시/패닉/세션 비정상 종료 수 | 0 |
| Reconnect Success Rate | 인터럽트 후 자동 복구된 비율 | ≥90% |
| Time To Reconnect | 끊김 감지 -> “사용 가능”까지 | 15초 이내(대부분) |
| CPU Overhead | 로컬(앱) CPU 증가 | <5%(관찰) |
| Memory Footprint | 프로세스 메모리 사용량 | <200MB(관찰) |

### 데이터 기록 테이블 (작성용)

| Run ID | Topology(A/B) | Sessions | Duration | Interrupt Type | Reconnect Success (x/y) | Reconnect Time p50/p95 (s) | Abnormal Ends | CPU(대략) | Mem(대략) | Notes |
|---|---|---:|---|---|---|---|---:|---|---|---|
|  |  | 10 | 30m | None |  |  |  |  |  |  |
|  |  | 10 | 30m | Local network drop |  |  |  |  |  |  |
|  |  | 10 | 30m | Server-side SSH restart |  |  |  |  |  |  |

---

## 장애 주입(Interrupt) 시나리오 (Failure Injection)

| Scenario | 방법(개념) | 기대 결과 |
|---|---|---|
| F1. 짧은 네트워크 단절 | 5~10초 정도 패킷 드롭/오프라인 | 대부분 자동 복구, 15초 내 정상화 |
| F2. 장기 네트워크 단절 | 30초+ 오프라인 | 자동 재시도 소진 후 “수동 재연결” 상태 |
| F3. 서버 재시작/sshd 재시작 | 원격에서 SSH 서비스 재시작 | 감지 후 재접속 시도, 실패 시 명확한 상태 전환 |
| F4. 인증 실패(의도) | 잘못된 비밀번호/키 | 자동 재시도 “중단” (사용자 개입 필요) |
| F5. 다중 세션 동시 끊김 | Wi-Fi/VPN 리셋 등 | 재접속 폭주를 피하도록 지터 적용 |

---

## 판정 기준 (Decision)

| Outcome | 조건 | 다음 단계 |
|---|---|---|
| 성공 | 핵심 성공 기준 Pass | MVP SSH Connection Manager/Auto-Reconnect 구현 가속 |
| 부분 성공 | 안정성은 OK, 리소스 사용량 과다/재접속 불안정 | keepalive/재접속 정책/채널 사용 방식 개선 후 재실험 |
| 실패 | 크래시/세션 붕괴/재접속 실패 다수 | SSH 구조/라이브러리/커넥션 모델 재검토 |

---

## 체크리스트 (Execution Checklist)

- [ ] 30분 런을 최소 2회 반복(재현성)
- [ ] 최소 1회는 “동시 다중 세션 끊김”을 주입
- [ ] 재접속 결과를 “성공률”과 “복구 시간 분포(p50/p95)”로 기록
- [ ] 실패 케이스는 원인 분류(네트워크/인증/서버/클라이언트)까지 남김

---

**Last Updated**: 2026-02-07
