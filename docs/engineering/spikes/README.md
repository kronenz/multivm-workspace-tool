# Technical Spikes — 기술 스파이크 추적

> 기술 스파이크는 MVP 구현 전에 불확실한 기술 선택을 프로토타입으로 검증하는 활동이다. 스파이크 실패 시 대안을 즉시 검토한다.

---

## 스파이크 현황

| # | 스파이크 | 우선순위 | 상태 | 연관 리스크 | 문서 |
|---|----------|----------|------|-------------|------|
| SPIKE-1 | Tauri + xterm.js Latency | **CRITICAL** | 대기 | RISK-1 | [실행 계획](./spike-1-tauri-xterm-latency.md) |
| SPIKE-2 | SSH 연결 풀링 Stress Test | **HIGH** | 대기 | RISK-2, RISK-4 | [실행 계획](./spike-2-ssh-pooling-stress.md) |
| SPIKE-3 | 이기종 VM 리소스 수집 호환성 | **MEDIUM** | 대기 | RISK-3 | (TODO: 실행 계획 문서화) |

> 상세 사양은 [architecture.md § Technical Spikes](../architecture.md#technical-spikes) 및 [mvp-spec.md § Technical Spike Priorities](../../qa/mvp-spec.md#technical-spike-priorities-from-architecture-document) 참조.

---

## 스파이크 성공 기준 요약

### SPIKE-1: Tauri + xterm.js Latency (CRITICAL)

- [ ] 키 입력 응답 <50ms (macOS, Ubuntu, Windows)
- [ ] 10,000줄 출력 시 <100ms 스크롤
- [ ] vim, htop, nano 정상 동작
- [ ] WebGL 렌더러 3개 OS 호환

**실패 시**: 대안 터미널 라이브러리(hterm, 커스텀 Canvas) 검토 또는 Electron 전환 검토

### SPIKE-2: SSH 연결 풀링 (HIGH)

- [ ] 10개 세션 30분 유지 (0% 비정상 종료)
- [ ] 재접속 성공률 ≥90% (15초 이내)
- [ ] 메모리 <200MB (10세션)
- [ ] CPU 오버헤드 <5%

**실패 시**: 더 공격적인 keepalive 또는 연결 상태 체크 구현

### SPIKE-3: 이기종 VM 호환성 (MEDIUM)

- [ ] Ubuntu 22.04 CPU/RAM/Disk 정상 추출
- [ ] Alpine 3.18 (BusyBox) CPU/RAM/Disk 정상 추출
- [ ] macOS 14 CPU/RAM/Disk 정상 추출
- [ ] 명령 미존재 시 N/A 표시 (크래시 없음)

**실패 시**: OS 감지 + per-OS 명령 전략 패턴 구현

---

## 스파이크 보고서 템플릿

스파이크 완료 시 아래 형식으로 결과를 이 폴더에 기록한다:

```markdown
# SPIKE-N 결과 보고서: [스파이크 제목]

**실행일**: YYYY-MM-DD
**결과**: 성공 | 부분 성공 | 실패

## 환경
- OS: [테스트한 OS 목록]
- 도구: [사용한 라이브러리/도구 버전]

## 성공 기준 검증

| 기준 | 결과 | 측정값 |
|------|------|--------|
| ... | Pass/Fail | ... |

## 발견 사항
[예상치 못한 발견, 주의 사항]

## 결론 및 다음 단계
[MVP 구현에 어떤 영향을 미치는가? ADR 업데이트 필요 여부]
```

---

**Last Updated**: 2026-02-07
