# Design System — 디자인 시스템

> **상태**: Skeleton (구현 시 상세화 예정)
> **목적**: UI 전반의 일관성을 위한 디자인 토큰, 컬러 팔레트, 타이포그래피, 간격 정의

---

## 컬러 팔레트

### Dark Theme (기본)

| 용도 | 토큰명 | 값 (예시) |
|------|--------|-----------|
| 배경 (Primary) | `--bg-primary` | `#1e1e1e` |
| 배경 (Secondary) | `--bg-secondary` | `#252526` |
| 배경 (Tertiary) | `--bg-tertiary` | `#2d2d2d` |
| 텍스트 (Primary) | `--text-primary` | `#d4d4d4` |
| 텍스트 (Secondary) | `--text-secondary` | `#858585` |
| 보더 | `--border` | `#3e3e3e` |
| 활성 보더 | `--border-active` | `#007acc` |

### Light Theme

| 용도 | 토큰명 | 값 (예시) |
|------|--------|-----------|
| 배경 (Primary) | `--bg-primary` | `#ffffff` |
| 배경 (Secondary) | `--bg-secondary` | `#f3f3f3` |
| 텍스트 (Primary) | `--text-primary` | `#333333` |
| 보더 | `--border` | `#e0e0e0` |
| 활성 보더 | `--border-active` | `#007acc` |

### 상태 컬러

| 상태 | 토큰명 | 값 | 용도 |
|------|--------|----|------|
| 정상 | `--status-ok` | `#4caf50` | 리소스 <50% |
| 주의 | `--status-warn` | `#ff9800` | 리소스 50-79% |
| 위험 | `--status-danger` | `#f44336` | 리소스 ≥80% |
| 정보 | `--status-info` | `#2196f3` | 연결 상태 |

---

## 타이포그래피

| 용도 | 폰트 | 크기 | 비고 |
|------|------|------|------|
| 터미널 | Monospace (시스템 기본) | 14px | xterm.js 설정 |
| UI 텍스트 | System Font Stack | 13px | `-apple-system, BlinkMacSystemFont, ...` |
| 제목 | System Font Stack | 16px | Bold |
| 라벨 | System Font Stack | 12px | Secondary 색상 |

---

## 간격 (Spacing)

| 토큰 | 값 | 용도 |
|------|----|------|
| `--space-xs` | 4px | 아이콘-텍스트 간격 |
| `--space-sm` | 8px | 패딩 (작은 컴포넌트) |
| `--space-md` | 16px | 패딩 (카드, 섹션) |
| `--space-lg` | 24px | 섹션 간 간격 |
| `--space-xl` | 32px | 페이지 마진 |

---

## 컴포넌트 토큰 (예정)

구현 시 각 컴포넌트별 디자인 토큰을 정의한다:

- **Sidebar**: 너비, 배경, 보더
- **Pane Divider**: 두께, 색상, 호버 색상
- **Status Bar**: 높이, 배경, 텍스트 크기
- **Dialog/Modal**: 배경, 오버레이, 보더 반경
- **Button**: Primary, Secondary, Danger 변형

---

## 참조

| 문서 | 경로 |
|------|------|
| Design Principles | [PRINCIPLES.md](./PRINCIPLES.md) |
| MVP Spec (Theme) | [mvp-spec.md](../qa/mvp-spec.md) |

---

**Last Updated**: 2026-02-07
