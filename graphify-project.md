# 📊 Graphify — Task Manager Project

## 1. 🌳 Sơ đồ cây thư mục (Directory Tree)

```
baitaplon/
├── .gitignore                  # Bỏ qua target/, *.db
├── Cargo.toml                  # Khai báo thư viện phụ thuộc
├── Cargo.lock                  # Lock phiên bản thư viện
├── README.md                   # Mô tả dự án
├── README_4_10.md              # Yêu cầu bài tập lớn
│
├── src/                        # ── MÃ NGUỒN RUST ──
│   ├── main.rs                 # Entry point: khởi tạo DB, Router, Server
│   │
│   ├── models/                 # Tầng Model (dữ liệu)
│   │   ├── mod.rs              #   └── Khai báo module user, task
│   │   ├── user.rs             #   └── Struct User + create(), find_by_username()
│   │   └── task.rs             #   └── Struct Task + create(), get_by_user_id(),
│   │                           #       delete(), update_status()
│   │
│   ├── handlers/               # Tầng Handler (xử lý logic)
│   │   ├── mod.rs              #   └── Khai báo module auth, task
│   │   ├── auth.rs             #   └── register(), login()
│   │   └── task.rs             #   └── create_task(), get_tasks(),
│   │                           #       delete_task(), update_task_status()
│   │
│   └── routes/                 # Tầng Route (định tuyến API)
│       ├── mod.rs              #   └── Khai báo module api
│       └── api.rs              #   └── api_router(): gắn route ↔ handler
│
├── static/                     # ── GIAO DIỆN FRONTEND ──
│   └── index.html              # Single-page app (HTML + CSS + JS)
│
└── task_manager.db             # SQLite database (tự tạo khi chạy)
```

---

## 2. 🏗️ Kiến trúc tổng quan (Architecture Overview)

```mermaid
flowchart TD
    subgraph Frontend["🌐 Frontend - static/index.html"]
        UI["HTML/CSS/JS<br/>Single Page App"]
    end

    subgraph Backend["⚙️ Backend - Rust / Axum"]
        Router["routes/api.rs<br/>API Router"]
        AuthHandler["handlers/auth.rs<br/>register() | login()"]
        TaskHandler["handlers/task.rs<br/>create | get | delete | update_status"]
        UserModel["models/user.rs<br/>User struct"]
        TaskModel["models/task.rs<br/>Task struct"]
    end

    subgraph Database["🗄️ SQLite - task_manager.db"]
        UsersTable["users<br/>id | username | password_hash"]
        TasksTable["tasks<br/>id | user_id | title | content | status | created_at"]
    end

    UI -->|"HTTP Request"| Router
    Router --> AuthHandler
    Router --> TaskHandler
    AuthHandler --> UserModel
    TaskHandler --> TaskModel
    UserModel -->|"SQL Query"| UsersTable
    TaskModel -->|"SQL Query"| TasksTable
```

---

## 3. 🔄 Biểu đồ luồng dữ liệu API (API Data Flow)

```mermaid
flowchart LR
    subgraph Client["🖥️ Client / Browser"]
        C["index.html"]
    end

    subgraph API["📡 API Endpoints"]
        R1["POST /api/register"]
        R2["POST /api/login"]
        R3["POST /api/tasks"]
        R4["GET /api/tasks"]
        R5["DELETE /api/tasks/:id"]
        R6["PUT /api/tasks/:id/status"]
    end

    subgraph Handlers["⚙️ Handlers"]
        H1["auth::register()"]
        H2["auth::login()"]
        H3["task::create_task()"]
        H4["task::get_tasks()"]
        H5["task::delete_task()"]
        H6["task::update_task_status()"]
    end

    subgraph DB["🗄️ SQLite"]
        T1["users"]
        T2["tasks"]
    end

    C --> R1 --> H1 -->|"INSERT user"| T1
    C --> R2 --> H2 -->|"SELECT user + bcrypt verify"| T1
    C --> R3 --> H3 -->|"INSERT task"| T2
    C --> R4 --> H4 -->|"SELECT tasks WHERE user_id"| T2
    C --> R5 --> H5 -->|"DELETE task WHERE id + user_id"| T2
    C --> R6 --> H6 -->|"UPDATE status WHERE id + user_id"| T2
```

---

## 4. 🔐 Luồng xác thực người dùng (Authentication Flow)

```mermaid
%%{init: {'theme': 'dark'}}%%
sequenceDiagram
    actor User as 👤 Người dùng
    participant FE as 🌐 Frontend
    participant BE as ⚙️ Backend
    participant DB as 🗄️ SQLite

    Note over User, FE: === ĐĂNG KÝ ===
    User->>FE: Nhập username + password (chứa @)
    FE->>BE: POST /api/register {username, password}
    BE->>BE: Kiểm tra password chứa '@'
    BE->>BE: Hash password (bcrypt)
    BE->>DB: INSERT INTO users
    DB-->>BE: OK (user_id)
    BE-->>FE: {success: true, user_id}
    FE-->>User: Hiển thị "Đăng ký thành công!"

    Note over User, FE: === ĐĂNG NHẬP ===
    User->>FE: Nhập username + password
    FE->>BE: POST /api/login {username, password}
    BE->>DB: SELECT user WHERE username
    DB-->>BE: User (id, password_hash)
    BE->>BE: bcrypt::verify(password, hash)
    BE-->>FE: {success: true, user_id}
    FE->>FE: Lưu user_id, hiện Task Manager
    FE-->>User: Hiển thị giao diện quản lý công việc
```

---

## 5. 📋 Luồng quản lý công việc (Task Management Flow)

```mermaid
%%{init: {'theme': 'dark'}}%%
sequenceDiagram
    actor User as 👤 Người dùng
    participant FE as 🌐 Frontend
    participant BE as ⚙️ Backend
    participant DB as 🗄️ SQLite

    Note over User, FE: === THÊM CÔNG VIỆC ===
    User->>FE: Nhập title + content
    FE->>BE: POST /api/tasks {user_id, title, content}
    BE->>DB: INSERT INTO tasks
    DB-->>BE: Task mới (với status = pending)
    BE-->>FE: {success: true, task}
    FE-->>User: Cập nhật danh sách

    Note over User, FE: === XEM DANH SÁCH ===
    FE->>BE: GET /api/tasks?user_id=X
    BE->>DB: SELECT * FROM tasks WHERE user_id = X
    DB-->>BE: Danh sách tasks
    BE-->>FE: {success: true, tasks: [...]}
    FE-->>User: Render bảng công việc

    Note over User, FE: === ĐỔI TRẠNG THÁI ===
    User->>FE: Chọn trạng thái mới (dropdown)
    FE->>BE: PUT /api/tasks/:id/status {user_id, status}
    BE->>DB: UPDATE tasks SET status WHERE id AND user_id
    DB-->>BE: OK (1 row affected)
    BE-->>FE: {success: true}
    FE-->>User: Hiển thị "Cập nhật thành công!"

    Note over User, FE: === XÓA CÔNG VIỆC ===
    User->>FE: Nhấn nút Delete → Xác nhận
    FE->>BE: DELETE /api/tasks/:id {user_id}
    BE->>DB: DELETE FROM tasks WHERE id AND user_id
    DB-->>BE: OK (1 row affected)
    BE-->>FE: {success: true}
    FE-->>User: Tải lại danh sách
```

---

## 6. 📊 Bảng tổng hợp API Endpoints

| Method   | Endpoint                | Handler                      | Mô tả                   |
| -------- | ----------------------- | ---------------------------- | ----------------------- |
| `POST`   | `/api/register`         | `auth::register()`           | Đăng ký tài khoản mới   |
| `POST`   | `/api/login`            | `auth::login()`              | Đăng nhập               |
| `POST`   | `/api/tasks`            | `task::create_task()`        | Tạo công việc mới       |
| `GET`    | `/api/tasks?user_id=X`  | `task::get_tasks()`          | Lấy danh sách công việc |
| `DELETE` | `/api/tasks/:id`        | `task::delete_task()`        | Xóa công việc           |
| `PUT`    | `/api/tasks/:id/status` | `task::update_task_status()` | Cập nhật trạng thái     |
