# Multi-VM AI Agent Workspace Tool

## 이 프로젝트는 무엇인가

개발자가 2-10개의 원격 VM에서 AI 코딩 에이전트(Claude Code, OpenCode)를 동시에 운용할 수 있는 **Tauri 기반 크로스플랫폼 데스크톱 앱**. 10개 이상의 터미널 창을 하나의 통합 워크스페이스로 대체한다.

## 필수 참조 문서

구조, 규칙, 컨벤션의 상세 정의:

- **[project-structure.md](./project-structure.md)** — 프로젝트 구조, 규칙, 네이밍 컨벤션
- **[AGENTS.md](./AGENTS.md)** — 프로젝트 지식 베이스 (아키텍처, 기술 스택, MVP 기능)

## 현재 상태

**Planning Complete** — 5개 기획 문서 완성, 기술 스파이크 및 MVP 구현 대기 중.

## 핵심 규칙 요약

1. **Trust Boundary**: 시스템 리소스 접근(SSH, 파일, OS Keystore)은 반드시 Rust Core에서 처리. Frontend는 샌드박스.
2. **IPC Only**: Frontend↔Backend 통신은 Tauri Commands / Events만 사용. 직접 소켓 금지.
3. **SSH 키 보안**: Workset JSON에 키 **경로**만 저장. 키 내용 저장 절대 금지 (NFR-12).
4. **비밀번호 보안**: OS 네이티브 보안 저장소에만 저장 (NFR-13).
5. **용어 일관성**: 모든 문서는 `docs/glossary.md`의 23개 핵심 용어 정의를 따른다.
6. **대량 터미널 출력**: 바이너리 IPC 또는 배치 처리를 고려 (RISK-4 완화).
7. **AI CLI 실행 ≠ 오케스트레이션**: 이 제품은 AI CLI를 **실행**하지만, AI 에이전트의 동작을 제어하지 않는다.

## 코드 작성 시 반드시 지킬 것

- **Rust Core**: SSH 연결, 파일 접근, 리소스 수집, Workset 저장, OS Keystore 연동
- **Frontend**: xterm.js 터미널 렌더링, Grid Layout, File Browser UI, Markdown Viewer UI
- **IPC**: Tauri Commands(FE→BE) / Events(BE→FE)만 사용
- **보안**: SSH 키 내용 JSON 저장 금지, 비밀번호 OS Keystore 전용
- **터미널**: WebGL 렌더러 기본, Canvas 폴백, 10K줄 스크롤백

## AI 에이전트 탐색 가이드

- 프로젝트 전체 요약: `AGENTS.md`
- 구조, 규칙, 컨벤션: `project-structure.md`
- 용어 정의: `docs/glossary.md`
- 아키텍처: `docs/engineering/architecture.md` (C4 다이어그램, 9 컴포넌트, 3 ADR)
- MVP 기능: `docs/qa/mvp-spec.md` (10 기능, 138 체크박스, E2E 시나리오)
- 경쟁 분석: `docs/product/market-research.md` (8 경쟁사, 4 시장 공백)
- 제품 요구사항: `docs/product/prd.md` (2 페르소나, MoSCoW 우선순위)ㅇ