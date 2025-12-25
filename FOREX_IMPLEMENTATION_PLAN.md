# Plan: Implement Forex Data Fetching & Display

## Overview
Implement tính năng fetch dữ liệu Forex từ API và hiển thị trong bảng UI khi user click button chọn group (USD, EUR, VND, CNY, JPY).

**Strategy**: Background task tự động refresh + filter theo group được chọn

**API Endpoint**: `https://103.48.84.52:4443/forex-latest-quote?group=forex.rates.{GROUP}`

---

## Files to Create/Modify

### Files to CREATE:
1. `c:\Tuan5\aim-trading-dev\src\tasks\quantitative\forex.rs` - Background task & API fetch logic
2. `c:\Tuan5\aim-trading-dev\aim_data\src\explorer\aim\forex.rs` - API client functions

### Files to MODIFY:
1. `c:\Tuan5\aim-trading-dev\ui\pages\quantitative\forex.slint` - Add properties & callbacks
2. `c:\Tuan5\aim-trading-dev\src\tasks\quantitative\mod.rs` - Export forex module
3. `c:\Tuan5\aim-trading-dev\src\tasks\mod.rs` - Re-export forex task
4. `c:\Tuan5\aim-trading-dev\src\main.rs` - Spawn task & register handlers
5. `c:\Tuan5\aim-trading-dev\aim_data\src\explorer\aim\mod.rs` - Export forex module & API functions

---

## Implementation Steps

### STEP 1: Define API Client (aim_data crate)

**File**: `c:\Tuan5\aim-trading-dev\aim_data\src\explorer\aim\forex.rs`

**Action**: Create new file với:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexRecord {
    pub symbol: String,
    pub group_name: String,
    pub symbol_name: String,
    pub last_price: f64,
    pub price_change: f64,
    pub percent_change: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForexResponse(pub Vec<ForexRecord>);

// Fetch forex data for specific group
pub async fn fetch_forex_data(group: &str) -> Result<ForexResponse, reqwest::Error> {
    let endpoint = format!("forex-latest-quote?group=forex.rates.{}", group);
    let data: Vec<ForexRecord> = super::fetch_api_data(&endpoint).await?;
    Ok(ForexResponse(data))
}
```

**Notes**:
- Reuse existing `fetch_api_data()` generic function (đã có HTTPS + SSL ignore)
- JSON fields map trực tiếp với struct (không cần #[serde(rename)])

---

### STEP 2: Export Forex Module (aim_data)

**File**: `c:\Tuan5\aim-trading-dev\aim_data\src\explorer\aim\mod.rs`

**Action**: Add sau line 15 (sau các mod declarations):
```rust
mod forex;
```

**Action**: Add vào public exports section (sau line 30):
```rust
pub use forex::{fetch_forex_data, ForexRecord, ForexResponse};
```

---

### STEP 3: Update UI Component (forex.slint)

**File**: `c:\Tuan5\aim-trading-dev\ui\pages\quantitative\forex.slint`

**Action 1**: Update ForexData struct (line 30-42) để match backend:
```slint
export struct ForexData {
    symbol: string,           // "^USDNZD"
    name: string,            // "U.S. Dollar/New Zealand Dollar"
    last: string,            // "1.7126"
    change: string,          // "-0.013"
    percent_change: string,  // "-0.75%"
    open: string,            // "1.7259"
    high: string,            // "1.7273"
    low: string,             // "1.7114"
    time: string,            // "23/12/25"
    is_up: bool,             // true if price_change > 0
}
```

**Action 2**: Update Forex component (line 190-223):
- Remove hardcoded forex_list data
- Add dynamic properties & callbacks

```slint
export component Forex inherits Rectangle {
    // Properties from backend
    in property <[ForexData]> forex_list: [];
    in property <string> selected_group: "USD";
    in property <bool> is_loading: false;

    // Callbacks to backend
    callback on_group_selected(string);

    // ... rest of component ...
}
```

**Action 3**: Update currency buttons (line 114-127) để trigger callback:
```slint
Rectangle {
    background: root.selected_group == "USD" ? #00b894 : #2f3640;
    TouchArea {
        clicked => { root.on_group_selected("USD"); }
    }
    Text { text: "USD"; }
}
// Repeat cho EUR, VND, CNY, JPY
```

**Action 4**: Remove hardcoded data (line 195-216)

---

### STEP 4: Create Background Task (tasks/quantitative/forex.rs)

**File**: `c:\Tuan5\aim-trading-dev\src\tasks\quantitative\forex.rs`

**Action**: Create với pattern từ `mp.rs`:
```rust
use crate::tasks::task_manager::{register_task, TaskStatus, TaskHandle};
use crate::slint_generatedAppWindow::ForexData as SlintForexData;
use crate::AppWindow;
use aim_data::aim::{fetch_forex_data, ForexRecord};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::sync::Arc;
use tokio::sync::Mutex;

// Convert API ForexRecord -> Slint ForexData
fn convert_to_forex_data(record: &ForexRecord) -> SlintForexData {
    // Format timestamp (i64) -> date string
    let datetime = chrono::DateTime::from_timestamp(record.trade_time, 0)
        .unwrap_or_else(|| chrono::Utc::now().into());
    let time_str = datetime.format("%d/%m/%y").to_string();

    SlintForexData {
        symbol: record.symbol.clone().into(),
        name: record.symbol_name.clone().into(),
        last: format!("{:.4}", record.last_price).into(),
        change: format!("{:.3}", record.price_change).into(),
        percent_change: format!("{:.2}%", record.percent_change * 100.0).into(),
        open: format!("{:.4}", record.open_price).into(),
        high: format!("{:.4}", record.high_price).into(),
        low: format!("{:.4}", record.low_price).into(),
        time: time_str.into(),
        is_up: record.price_change > 0.0,
    }
}

pub async fn spawn_forex_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.quantitative.forex".to_string(),
        tx,
        "Forex Data Fetcher".to_string(),
    ).await;

    // Shared state for selected group
    let selected_group = Arc::new(Mutex::new("USD".to_string()));

    // Register callback for group selection
    let selected_group_clone = Arc::clone(&selected_group);
    let ui_handle_callback = ui_handle.clone();
    ui_handle.unwrap().on_on_group_selected(move |group: SharedString| {
        let selected_group = Arc::clone(&selected_group_clone);
        let group_str = group.to_string();
        tokio::spawn(async move {
            let mut g = selected_group.lock().await;
            *g = group_str;
        });
    });

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;

        loop {
            // Check status
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Forex task status changed to: {:?}", status);
                    task_status = status;
                }
            }

            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Get current selected group
            let group = {
                let g = selected_group.lock().await;
                g.clone()
            };

            // Fetch data for selected group
            match fetch_forex_data(&group).await {
                Ok(response) => {
                    let forex_data: Vec<SlintForexData> = response
                        .0
                        .iter()
                        .map(convert_to_forex_data)
                        .collect();

                    log::info!("Fetched {} forex records for group {}", forex_data.len(), group);

                    // Update UI
                    let group_clone = group.clone();
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        ui.set_forex_list(ModelRc::new(VecModel::from(forex_data)));
                        ui.set_selected_group(group_clone.into());
                    });
                }
                Err(e) => {
                    log::error!("Failed to fetch forex data for {}: {}", group, e);
                }
            }

            // Refresh every 5 seconds
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    task_handle
}
```

**Notes**:
- Sử dụng `Arc<Mutex<String>>` để share selected_group giữa callback và task loop
- Auto refresh mỗi 5 giây
- Callback `on_on_group_selected` để update group khi user click button

---

### STEP 5: Export Task Module

**File**: `c:\Tuan5\aim-trading-dev\src\tasks\quantitative\mod.rs`

**Action**: Add sau line cuối:
```rust
mod forex;
pub use forex::spawn_forex_task;
```

---

**File**: `c:\Tuan5\aim-trading-dev\src\tasks\mod.rs`

**Action**: Add vào use statement (around line 23):
```rust
pub use quantitative::{
    ...,
    spawn_forex_task,  // ADD THIS
};
```

---

### STEP 6: Register Task in main.rs

**File**: `c:\Tuan5\aim-trading-dev\src\main.rs`

**Action 1**: Add import (line 23-34):
```rust
use tasks::{
    ...,
    spawn_forex_task,  // ADD THIS
};
```

**Action 2**: Spawn task (sau line 520, trước `ui.run()`):
```rust
// Forex data fetcher
let _forex_task = spawn_forex_task(&ui).await;
log::info!("Forex task spawned");
```

**Notes**:
- Task tự động bắt đầu fetch data khi app start
- Callback được register trong `spawn_forex_task()`, không cần thêm code

---

## Data Flow Diagram

```
[App Start]
    ↓
[main.rs] spawn_forex_task(&ui)
    ↓
[forex.rs] Task starts → Fetch group "USD" (default)
    ↓
[aim_data] fetch_forex_data("USD") → API call
    ↓
[forex.rs] Convert ForexRecord → SlintForexData
    ↓
[forex.rs] ui.set_forex_list(data)
    ↓
[forex.slint] Table updates (ListView renders)

--- User Interaction ---

[User clicks EUR button]
    ↓
[forex.slint] TouchArea.clicked → on_group_selected("EUR")
    ↓
[forex.rs] Callback updates Arc<Mutex<selected_group>>
    ↓
[forex.rs] Next loop iteration fetches "EUR" data
    ↓
[forex.slint] Table updates with EUR pairs
```

---

## Testing Checklist

### Phase 1: Build & Compilation
- [ ] `cargo build` thành công
- [ ] Không có warnings trong forex.rs
- [ ] Slint compile không lỗi

### Phase 2: Initial Load
- [ ] App start → Forex tab hiển thị
- [ ] Default group "USD" được fetch
- [ ] Bảng hiển thị dữ liệu USD pairs
- [ ] USD button được highlight (màu #00b894)

### Phase 3: Group Switching
- [ ] Click EUR button → data thay đổi sang EUR pairs
- [ ] Click VND button → data thay đổi sang VND pairs
- [ ] Click CNY button → data thay đổi sang CNY pairs
- [ ] Click JPY button → data thay đổi sang JPY pairs
- [ ] Selected button được highlight đúng

### Phase 4: Auto Refresh
- [ ] Data tự động refresh mỗi 5 giây
- [ ] Console log hiển thị "Fetched X forex records for group Y"
- [ ] UI không bị flicker khi update

### Phase 5: Error Handling
- [ ] API offline/timeout → log error, không crash
- [ ] Invalid group → graceful error handling
- [ ] Empty response → hiển thị bảng rỗng (không crash)

### Phase 6: Data Display
- [ ] Giá hiển thị đúng format (4 số thập phân)
- [ ] Percent change hiển thị đúng (2 số thập phân + %)
- [ ] Màu xanh cho price_change > 0
- [ ] Màu đỏ cho price_change < 0
- [ ] Timestamp format đúng (dd/mm/yy)

---

## Critical Implementation Notes

1. **HTTPS SSL**: API sử dụng self-signed cert → `fetch_api_data()` đã có `.danger_accept_invalid_certs(true)`

2. **Group Format**: API endpoint cần `forex.rates.{GROUP}` → format string trong `fetch_forex_data()`

3. **Callback Timing**: Register callback TRƯỚC khi spawn tokio task → tránh race condition

4. **Thread Safety**: Dùng `Arc<Mutex<>>` để share selected_group giữa callback thread và task loop

5. **UI Update**: LUÔN dùng `upgrade_in_event_loop()` để update UI từ async task

6. **Error Logging**: Match Result → log error với context → continue loop (KHÔNG crash)

---

## File Summary

| File | Action | Lines | Priority |
|------|--------|-------|----------|
| `aim_data/src/explorer/aim/forex.rs` | CREATE | ~30 | HIGH |
| `aim_data/src/explorer/aim/mod.rs` | MODIFY | +2 | HIGH |
| `src/tasks/quantitative/forex.rs` | CREATE | ~120 | HIGH |
| `src/tasks/quantitative/mod.rs` | MODIFY | +2 | MEDIUM |
| `src/tasks/mod.rs` | MODIFY | +1 | MEDIUM |
| `ui/pages/quantitative/forex.slint` | MODIFY | ~30 | HIGH |
| `src/main.rs` | MODIFY | +3 | HIGH |

**Total LOC**: ~190 lines of new code + ~40 lines of modifications

---

## Potential Issues & Solutions

### Issue 1: API Response Array vs Object
**Problem**: API có thể trả về `{"data": [...]}` thay vì `[...]`
**Solution**: Kiểm tra response, thêm wrapper struct nếu cần

### Issue 2: Group Name Case Sensitivity
**Problem**: API cần "USD" hay "usd"?
**Solution**: Test API, convert `.to_uppercase()` nếu cần

### Issue 3: Rate Limit
**Problem**: API có thể rate limit với 5s interval
**Solution**: Tăng interval lên 10-15s nếu cần, hoặc implement backoff

### Issue 4: Empty Response
**Problem**: Group không có data → empty array
**Solution**: Handle gracefully, hiển thị "No data available"

---

## Next Steps After Implementation

1. **Performance**: Monitor memory usage với large datasets
2. **Caching**: Consider cache data để giảm API calls
3. **Sorting**: Thêm sort columns nếu user request
4. **Search**: Thêm search box nếu cần (hiện tại không cần)
5. **Chart Integration**: Tích hợp biểu đồ vào ChartBox (future enhancement)

---

**END OF PLAN**
