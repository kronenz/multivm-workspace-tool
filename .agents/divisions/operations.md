# 2.4 Operations Division (운영 부문)

**인덱스**: `2.4`  
**계층**: Division (부문 계층)

---

## 관할 (Jurisdiction)

- 빌드 파이프라인
- CI/CD
- 릴리스

---

## 부문 규칙 (Division Rules)

### 1. 빌드 검증

**필수**: 다음 명령어 모두 성공해야 머지 가능

```bash
npm run build
cargo build --release
```

**실패 시**: 즉시 롤백 또는 수정

---

### 2. 크로스 플랫폼

**필수**: 3개 OS 타겟 지원

| OS | 최소 버전 | 근거 |
|----|----------|------|
| macOS | 11+ | NFR-5 |
| Ubuntu | 20.04+ | NFR-5 |
| Windows | 10+ | NFR-5 |

**검증 절차**:
1. 각 OS에서 빌드 성공 확인
2. 각 OS에서 Feature 1-4 테스트 실행
3. 모든 OS에서 통과 시에만 릴리스

---

### 3. 번들 크기

**필수**: Tauri 번들 <10MB 유지

**근거**: ADR-001 (Tauri 선택 이유 — Electron 대비 작은 번들)

**검증 방법**:
```bash
npm run tauri build
ls -lh src-tauri/target/release/bundle/
```

**초과 시**: 의존성 제거 또는 최적화

---

### 4. 의존성 감사

**필수**: 새 crate/npm 패키지 추가 시 다음 검토

| 검토 항목 | 확인 방법 |
|----------|----------|
| 라이선스 | MIT/Apache 2.0 호환 여부 |
| 보안 | `cargo audit` / `npm audit` |
| 크기 | 번들 크기 영향 측정 |

**승인 필요**: 1.2 Technical Director

---

## 빌드 실패 대응 절차 (Build Failure Response)

```
[발견자] → 2.4 Operations Division 통보
  ↓
[2.4] 원인 파일 확인
  ├─ src-tauri/ → 2.1 Backend Division 통보
  └─ src/ → 2.2 Frontend Division 통보
  ↓
[해당 부문장] 해당 팀(3.x)에 수정 지시
  ↓
[팀(3.x)] 30분 내 수정
  ├─ 수정 완료 → 빌드 확인 → 머지
  └─ 30분 내 불가 → 부문장 에스컬레이션
```

**참조**: `.agents/protocols/hotfix.md` (9.3)

---

## 릴리스 체크리스트 (Release Checklist)

```
□ 모든 Feature Done Criteria 통과
□ 모든 NFR 충족
□ 3개 OS 빌드 성공
□ 번들 크기 <10MB
□ 의존성 감사 통과
□ 보안 체크리스트 통과 (5.2)
□ 문서 업데이트 (README, CHANGELOG)
```

---

## 관련 문서 (Related Documents)

- `.agents/protocols/hotfix.md` (9.3) — 긴급 수정 프로토콜
- `.agents/support/security.md` (5.2) — 보안 체크리스트
