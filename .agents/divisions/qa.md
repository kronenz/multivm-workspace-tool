# 2.3 QA Division (품질 부문)

**인덱스**: `2.3`  
**계층**: Division (부문 계층)

---

## 관할 (Jurisdiction)

- 테스트 전략
- AC (Acceptance Criteria) 검증
- NFR (Non-Functional Requirements) 측정

---

## 참조 문서 (Reference Documents)

- `docs/qa/mvp-spec.md` — 138개 Done Criteria, 10개 AC 섹션

---

## 부문 규칙 (Division Rules)

### 1. Done Criteria 체크리스트

**필수**: 기능 완료 주장 시 해당 Feature의 체크리스트를 하나씩 검증

**절차**:
1. `docs/qa/mvp-spec.md` § Feature X Done Criteria 열기
2. 각 항목을 실제로 테스트
3. 모든 항목 통과 시에만 "완료" 인정

---

### 2. NFR 검증

**필수**: 성능 기준(NFR-1~13) 미달 시 머지 차단

**검증 항목**:
- NFR-1: SSH 연결 지연 ≤2초 (로컬), ≤5초 (인터넷)
- NFR-2: 터미널 렌더링 10K줄 <100ms
- NFR-3: 패인 리사이즈 <50ms
- NFR-8: 자동 재접속 ≥90%, 15초 이내
- NFR-10: Workset 활성화 4VM 2x2 ≤10초

---

### 3. E2E 시나리오

**필수**: `mvp-spec.md`의 8-Step E2E Journey를 재현 가능해야 함

**8-Step Journey**:
1. 앱 실행 → Workset 목록 표시
2. "My Project" Workset 선택 → 4VM 2x2 그리드 활성화
3. 각 터미널에 SSH 연결 → 프롬프트 표시
4. AI CLI 자동 실행 → 각 VM에서 Claude Code 시작
5. 파일 브라우저 열기 → 디렉토리 트리 표시
6. README.md 클릭 → Markdown 뷰어에 렌더링
7. 리소스 모니터 확인 → CPU/RAM 실시간 표시
8. 네트워크 끊김 시뮬레이션 → 자동 재접속 성공

---

### 4. 회귀 방지

**필수**: 기존 Feature 1-4 깨뜨리는 변경은 즉시 롤백

**검증 절차**:
1. 새 Feature 구현 후 Feature 1-4 테스트 재실행
2. 하나라도 실패 시 즉시 롤백
3. 원인 분석 후 수정

---

## AC 체크 매트릭스 (AC Check Matrix)

| Feature | AC 섹션 | 체크 항목 수 | 상태 |
|---------|---------|-------------|------|
| F1: Workset CRUD | AC-1 | 7개 | ✅ |
| F2: SSH Connection | AC-2 | 5개 | ✅ |
| F3: Terminal | AC-3 | 7개 | ✅ |
| F4: Grid Layout | AC-4 | 6개 | ✅ |
| F5: File Browser | AC-5 | 6개 | ⬜ |
| F6: Markdown Viewer | AC-6 | 5개 | ⬜ |
| F7: Resource Monitor | AC-7 | 7개 | ⬜ |
| F8: AI CLI Auto-Launch | AC-8 | 6개 | ⬜ |
| F9: SSH Auto-Reconnect | AC-9 | 6개 | ⬜ |
| F10: Dark/Light Theme | AC-10 | 6개 | ⬜ |

---

## 검증 체크리스트 (Verification Checklist)

### Feature 완료 주장 시 확인 사항

```
□ Done Criteria 모든 항목 통과
□ AC 섹션 테스트 통과
□ 관련 NFR 충족
□ 기존 Feature 1-4 회귀 없음
□ 빌드 성공 (npm run build + cargo build --release)
□ 크로스 플랫폼 테스트 (macOS, Ubuntu, Windows)
```

---

## 관련 문서 (Related Documents)

- `docs/qa/mvp-spec.md` — MVP 기능 명세
- `.agents/protocols/feature-implementation.md` (9.4) — 기능 구현 절차 Phase 5 (검증)
