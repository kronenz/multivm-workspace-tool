# 3.3 Process & Resource Team

**인덱스**: `3.3` | **부문**: 2.1 Backend | **담당 기능**: F7, F8

---

## 소유 컴포넌트 (Owned Components)

- Process Manager
- Resource Poller

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src-tauri/src/process/` | AI CLI 자동 실행 | ⬜ Feature 8 |
| `src-tauri/src/resource/` | 리소스 수집 (CPU/RAM/Disk) | ⬜ Feature 7 |

---

## IPC Commands 소유 (미구현)

| Command | 설명 | Feature |
|---------|------|---------|
| `start_polling` | 리소스 수집 시작 | F7 |
| `stop_polling` | 리소스 수집 중지 | F7 |

---

## IPC Events 소유 (미구현)

| Event | 설명 | Feature |
|-------|------|---------|
| `resource_update` | CPU/RAM/Disk 데이터 (5초 간격) | F7 |
| `process_exited` | AI CLI 프로세스 종료 알림 | F8 |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. AI CLI 자동 실행

**명령어 형식**:
```bash
cd <project_folder> && <ai_cli_command>
```

**예시**:
```bash
cd /home/user/my-project && claude-code
cd /var/www/api && opencode
```

**구현**:
```rust
use tokio::process::Command;

async fn launch_ai_cli(vm_id: &str, project_folder: &str, cli_command: &str) -> Result<Child, Error> {
    let full_command = format!("cd {} && {}", project_folder, cli_command);
    
    let child = Command::new("ssh")
        .arg(vm_id)
        .arg(full_command)
        .spawn()?;
    
    Ok(child)
}
```

---

### 2. 리소스 수집: 5초 간격

**수집 항목**:
- CPU 사용률 (%)
- RAM 사용량 (MB / Total MB)
- Disk 사용량 (GB / Total GB)

**수집 방법**: SSH exec

**구현**:
```rust
use tokio::time::{interval, Duration};

async fn poll_resources(vm_id: &str, app_handle: AppHandle) {
    let mut ticker = interval(Duration::from_secs(5));
    
    loop {
        ticker.tick().await;
        
        let cpu = get_cpu_usage(vm_id).await.unwrap_or_else(|_| "N/A".to_string());
        let ram = get_ram_usage(vm_id).await.unwrap_or_else(|_| "N/A".to_string());
        let disk = get_disk_usage(vm_id).await.unwrap_or_else(|_| "N/A".to_string());
        
        app_handle.emit("resource_update", ResourceData { vm_id, cpu, ram, disk })?;
    }
}
```

---

### 3. OS별 수집 전략 패턴

#### Linux (일반)
```bash
# CPU
top -bn1 | grep "Cpu(s)" | awk '{print $2}'

# RAM
free -m | awk 'NR==2{printf "%.2f", $3*100/$2 }'

# Disk
df -h / | awk 'NR==2{print $5}'
```

#### macOS
```bash
# CPU
top -l1 | grep "CPU usage" | awk '{print $3}'

# RAM
vm_stat | perl -ne '/page size of (\d+)/ and $size=$1; /Pages active:\s+(\d+)/ and printf("%.2f\n", $1 * $size / 1048576);'

# Disk
df -h / | awk 'NR==2{print $5}'
```

#### Alpine (BusyBox)
```bash
# CPU
cat /proc/stat | grep "cpu " | awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage}'

# RAM
free | awk 'NR==2{printf "%.2f", $3*100/$2 }'

# Disk
df -h / | awk 'NR==2{print $5}'
```

---

### 4. 명령 실패 시 "N/A" 반환

**원칙**: 크래시 절대 금지

**구현**:
```rust
async fn get_cpu_usage(vm_id: &str) -> Result<String, Error> {
    match ssh_exec(vm_id, "top -bn1 | grep 'Cpu(s)'").await {
        Ok(output) => Ok(parse_cpu(output)),
        Err(_) => Ok("N/A".to_string()), // 실패 시 N/A 반환
    }
}
```

---

### 5. 프로세스 종료 감지 → `process_exited` 이벤트 발행

**구현**:
```rust
use tokio::process::Child;

async fn monitor_process(mut child: Child, vm_id: String, app_handle: AppHandle) {
    let status = child.wait().await.unwrap();
    
    app_handle.emit("process_exited", ProcessExitedEvent {
        vm_id,
        exit_code: status.code(),
    }).unwrap();
}
```

---

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 7 (AC-7), Feature 8 (AC-8)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/backend.md` (2.1) — Backend 부문 규칙
- `docs/engineering/architecture.md` — Process Manager, Resource Poller 아키텍처
