# 📄 PRD — Product Requirements Document
## Hệ thống Quản lý Công việc Cá nhân (Task Manager)

| Thông tin | Chi tiết |
|-----------|----------|
| **Tên dự án** | Task Manager |
| **Phiên bản** | 1.0 |
| **Ngày tạo** | 01/11/2026 |
| **Tác giả** | Senior Business Analyst |
| **Công nghệ** | Rust (Axum) · SQLite (sqlx) · HTML/CSS/JS |

---

## 1. 🎯 Mục tiêu dự án

### 1.1. Tầm nhìn sản phẩm
Xây dựng một **ứng dụng web quản lý công việc cá nhân** đơn giản, cho phép người dùng đăng ký tài khoản, đăng nhập, và thực hiện các thao tác quản lý công việc (thêm, xem, cập nhật trạng thái, xóa) một cách nhanh chóng và trực quan.

### 1.2. Mục tiêu kinh doanh
- Cung cấp công cụ quản lý công việc cá nhân **miễn phí**, **nhẹ**, **không cần cài đặt phức tạp**
- Đảm bảo mỗi người dùng chỉ thấy và quản lý công việc của **chính mình**
- Giao diện đơn giản, dễ sử dụng, **một trang duy nhất** (Single Page Application)

### 1.3. Đối tượng người dùng
| Vai trò | Mô tả |
|---------|-------|
| **Người dùng cuối** | Cá nhân muốn theo dõi và quản lý danh sách công việc hàng ngày |

### 1.4. Phạm vi sản phẩm (Scope)

**Trong phạm vi (In-scope):**
- Module xác thực: Đăng ký, Đăng nhập, Đăng xuất
- Module quản lý công việc: Thêm, Xem danh sách, Cập nhật trạng thái, Xóa
- Giao diện web đơn trang (SPA)

**Ngoài phạm vi (Out-of-scope):**
- Quản lý nhiều người dùng (admin panel)
- Chia sẻ công việc giữa các người dùng
- Thông báo / nhắc nhở tự động
- Ứng dụng di động

---

## 2. 📊 Mô hình dữ liệu

### 2.1. Entity: User (Người dùng)

| Trường | Kiểu dữ liệu | Ràng buộc | Mô tả |
|--------|--------------|-----------|-------|
| `id` | INTEGER | PRIMARY KEY, AUTOINCREMENT | Mã định danh duy nhất |
| `username` | TEXT | UNIQUE, NOT NULL | Tên đăng nhập |
| `password_hash` | TEXT | NOT NULL | Mật khẩu đã được mã hóa bcrypt |

### 2.2. Entity: Task (Công việc)

| Trường | Kiểu dữ liệu | Ràng buộc | Mô tả |
|--------|--------------|-----------|-------|
| `id` | INTEGER | PRIMARY KEY, AUTOINCREMENT | Mã định danh duy nhất |
| `user_id` | INTEGER | NOT NULL, FOREIGN KEY → users.id | Người sở hữu công việc |
| `title` | TEXT | NOT NULL | Tiêu đề công việc |
| `content` | TEXT | NULLABLE | Nội dung chi tiết |
| `status` | TEXT | DEFAULT 'pending' | Trạng thái: `pending` · `in_progress` · `done` |
| `created_at` | DATETIME | DEFAULT CURRENT_TIMESTAMP | Thời điểm tạo |

---

## 3. 📡 Danh sách API Endpoints

| # | Method | Endpoint | Mô tả | Request Body | Response |
|---|--------|----------|-------|-------------|----------|
| 1 | `POST` | `/api/register` | Đăng ký tài khoản | `{username, password}` | `{success, user_id}` |
| 2 | `POST` | `/api/login` | Đăng nhập | `{username, password}` | `{success, user_id}` |
| 3 | `POST` | `/api/tasks` | Tạo công việc mới | `{user_id, title, content}` | `{success, task}` |
| 4 | `GET` | `/api/tasks?user_id=X` | Lấy danh sách công việc | — | `{success, tasks[]}` |
| 5 | `DELETE` | `/api/tasks/:id` | Xóa công việc | `{user_id}` | `{success, message}` |
| 6 | `PUT` | `/api/tasks/:id/status` | Cập nhật trạng thái | `{user_id, status}` | `{success, message}` |

---

## 4. 📝 User Stories & Acceptance Criteria (Gherkin)

---

### 4.1. EPIC: Xác thực người dùng (Authentication)

---

#### US-01: Đăng ký tài khoản mới

> **As a** người dùng chưa có tài khoản,
> **I want to** đăng ký một tài khoản mới bằng username và password,
> **So that** tôi có thể sử dụng hệ thống quản lý công việc.

**Acceptance Criteria:**

```gherkin
Feature: Đăng ký tài khoản

  Scenario: Đăng ký thành công với thông tin hợp lệ
    Given người dùng chưa có tài khoản trong hệ thống
    And người dùng đang ở trang đăng ký
    When người dùng nhập username "nguyenvana"
    And người dùng nhập password "matkhau@123"
    And người dùng nhấn nút "Register"
    Then hệ thống tạo tài khoản mới với mật khẩu được mã hóa bcrypt
    And hệ thống trả về mã trạng thái 201 (Created)
    And hệ thống trả về JSON chứa "success: true" và "user_id"
    And giao diện hiển thị thông báo "Registration successful! Please login."
    And giao diện tự động chuyển sang tab Đăng nhập

  Scenario: Đăng ký thất bại khi mật khẩu không chứa ký tự @
    Given người dùng đang ở trang đăng ký
    When người dùng nhập username "nguyenvana"
    And người dùng nhập password "matkhau123" (không chứa @)
    And người dùng nhấn nút "Register"
    Then hệ thống trả về mã trạng thái 400 (Bad Request)
    And hệ thống trả về JSON chứa "error: Password must contain @ character"
    And giao diện hiển thị thông báo lỗi màu đỏ

  Scenario: Đăng ký thất bại khi username đã tồn tại
    Given đã có tài khoản với username "nguyenvana" trong hệ thống
    When người dùng nhập username "nguyenvana"
    And người dùng nhập password "mk@moi456"
    And người dùng nhấn nút "Register"
    Then hệ thống trả về mã trạng thái 409 (Conflict)
    And hệ thống trả về JSON chứa "error: Username already exists"
    And giao diện hiển thị thông báo lỗi tương ứng
```

**Quy tắc nghiệp vụ:**
- Mật khẩu **bắt buộc** phải chứa ký tự `@`
- Username phải là **duy nhất** trong hệ thống
- Mật khẩu được lưu trữ dưới dạng **bcrypt hash**, không lưu bản rõ

---

#### US-02: Đăng nhập vào hệ thống

> **As a** người dùng đã có tài khoản,
> **I want to** đăng nhập bằng username và password,
> **So that** tôi có thể truy cập và quản lý công việc của mình.

**Acceptance Criteria:**

```gherkin
Feature: Đăng nhập

  Scenario: Đăng nhập thành công
    Given người dùng đã có tài khoản với username "nguyenvana" và password "matkhau@123"
    And người dùng đang ở trang đăng nhập
    When người dùng nhập username "nguyenvana"
    And người dùng nhập password "matkhau@123"
    And người dùng nhấn nút "Login"
    Then hệ thống xác minh password bằng bcrypt
    And hệ thống trả về mã trạng thái 200 (OK)
    And hệ thống trả về JSON chứa "success: true" và "user_id"
    And giao diện ẩn form đăng nhập
    And giao diện hiển thị trang quản lý công việc (Task Manager)
    And giao diện tự động tải danh sách công việc

  Scenario: Đăng nhập thất bại với username không tồn tại
    Given không có tài khoản nào với username "khongcoten"
    When người dùng nhập username "khongcoten"
    And người dùng nhập password "abc@123"
    And người dùng nhấn nút "Login"
    Then hệ thống trả về mã trạng thái 401 (Unauthorized)
    And hệ thống trả về JSON chứa "error: Invalid username or password"
    And giao diện hiển thị thông báo lỗi màu đỏ

  Scenario: Đăng nhập thất bại với mật khẩu sai
    Given người dùng đã có tài khoản với username "nguyenvana"
    When người dùng nhập username "nguyenvana"
    And người dùng nhập password "saimatkhau@"
    And người dùng nhấn nút "Login"
    Then hệ thống xác minh password bằng bcrypt và kết quả không khớp
    And hệ thống trả về mã trạng thái 401 (Unauthorized)
    And hệ thống trả về JSON chứa "error: Invalid username or password"
    And giao diện hiển thị thông báo lỗi màu đỏ
```

**Quy tắc nghiệp vụ:**
- Thông báo lỗi **không phân biệt** giữa sai username hay sai password (bảo mật)
- Xác thực password bằng thuật toán **bcrypt verify**

---

#### US-03: Đăng xuất khỏi hệ thống

> **As a** người dùng đã đăng nhập,
> **I want to** đăng xuất khỏi hệ thống,
> **So that** tôi có thể bảo vệ tài khoản khi không sử dụng.

**Acceptance Criteria:**

```gherkin
Feature: Đăng xuất

  Scenario: Đăng xuất thành công
    Given người dùng đã đăng nhập và đang ở trang quản lý công việc
    When người dùng nhấn nút "Logout"
    Then giao diện xóa thông tin phiên đăng nhập (user_id)
    And giao diện ẩn trang quản lý công việc
    And giao diện hiển thị lại form đăng nhập
    And giao diện hiển thị thông báo "Logged out successfully"
```

---

### 4.2. EPIC: Quản lý công việc (Task Management)

---

#### US-04: Thêm công việc mới

> **As a** người dùng đã đăng nhập,
> **I want to** tạo một công việc mới với tiêu đề và nội dung,
> **So that** tôi có thể theo dõi các việc cần làm.

**Acceptance Criteria:**

```gherkin
Feature: Thêm công việc mới

  Scenario: Thêm công việc thành công
    Given người dùng đã đăng nhập với user_id = 1
    And người dùng đang ở trang quản lý công việc
    When người dùng nhập tiêu đề "Hoàn thành bài tập lớn"
    And người dùng nhập nội dung "Nộp trước ngày 15/11"
    And người dùng nhấn nút "Add Task"
    Then hệ thống tạo công việc mới trong CSDL liên kết với user_id = 1
    And công việc có trạng thái mặc định là "pending"
    And công việc có thời gian tạo là thời điểm hiện tại
    And hệ thống trả về mã trạng thái 201 (Created)
    And giao diện hiển thị thông báo "Task added successfully!"
    And giao diện tự động tải lại danh sách công việc
    And các trường nhập liệu được xóa sạch

  Scenario: Thêm công việc thất bại khi thiếu tiêu đề
    Given người dùng đã đăng nhập
    When người dùng để trống trường tiêu đề
    And người dùng nhấn nút "Add Task"
    Then giao diện hiển thị thông báo lỗi "Task title is required"
    And hệ thống KHÔNG gửi request đến server

  Scenario: Thêm công việc với nội dung tùy chọn
    Given người dùng đã đăng nhập
    When người dùng nhập tiêu đề "Đi chợ"
    And người dùng để trống trường nội dung
    And người dùng nhấn nút "Add Task"
    Then hệ thống tạo công việc mới với content = ""
    And hệ thống trả về mã trạng thái 201 (Created)
```

**Quy tắc nghiệp vụ:**
- Tiêu đề (`title`) là **bắt buộc**
- Nội dung (`content`) là **tùy chọn**
- Trạng thái mặc định luôn là `"pending"`

---

#### US-05: Xem danh sách công việc

> **As a** người dùng đã đăng nhập,
> **I want to** xem toàn bộ danh sách công việc của mình,
> **So that** tôi có thể biết được các việc đang cần xử lý.

**Acceptance Criteria:**

```gherkin
Feature: Xem danh sách công việc

  Scenario: Hiển thị danh sách công việc thành công
    Given người dùng đã đăng nhập với user_id = 1
    And người dùng có 3 công việc trong hệ thống
    When trang quản lý công việc được tải
    Then hệ thống gửi GET request đến /api/tasks?user_id=1
    And giao diện hiển thị bảng với 3 dòng công việc
    And mỗi dòng hiển thị: Tiêu đề, Nội dung, Trạng thái (dropdown), Ngày tạo, Nút Xóa
    And danh sách được sắp xếp theo thời gian tạo giảm dần (mới nhất trước)

  Scenario: Hiển thị thông báo khi không có công việc nào
    Given người dùng đã đăng nhập
    And người dùng chưa tạo công việc nào
    When trang quản lý công việc được tải
    Then giao diện hiển thị thông báo "No tasks yet. Add one above!"

  Scenario: Chỉ hiển thị công việc của người dùng hiện tại
    Given có 2 người dùng: user_id=1 (có 3 task) và user_id=2 (có 5 task)
    When người dùng user_id=1 đăng nhập và xem danh sách
    Then hệ thống chỉ trả về 3 công việc thuộc về user_id=1
    And KHÔNG hiển thị bất kỳ công việc nào của user_id=2
```

**Quy tắc nghiệp vụ:**
- Người dùng **chỉ thấy công việc của chính mình** (lọc theo `user_id`)
- Sắp xếp theo `created_at DESC`

---

#### US-06: Cập nhật trạng thái công việc

> **As a** người dùng đã đăng nhập,
> **I want to** thay đổi trạng thái của một công việc,
> **So that** tôi có thể theo dõi tiến độ hoàn thành.

**Acceptance Criteria:**

```gherkin
Feature: Cập nhật trạng thái công việc

  Scenario: Đổi trạng thái từ "Pending" sang "In Progress"
    Given người dùng đã đăng nhập với user_id = 1
    And có công việc task_id=5 đang ở trạng thái "pending"
    When người dùng chọn trạng thái "In Progress" từ dropdown
    Then hệ thống gửi PUT request đến /api/tasks/5/status
    And hệ thống cập nhật trạng thái trong CSDL thành "in_progress"
    And hệ thống trả về mã trạng thái 200 (OK)
    And giao diện hiển thị thông báo "Task status updated!"

  Scenario: Đổi trạng thái sang "Done"
    Given có công việc task_id=5 đang ở trạng thái "in_progress"
    When người dùng chọn trạng thái "Done" từ dropdown
    Then hệ thống cập nhật trạng thái thành "done"
    And giao diện hiển thị thông báo thành công

  Scenario: Cập nhật thất bại khi task không tồn tại
    Given người dùng gửi request cập nhật cho task_id=999 (không tồn tại)
    Then hệ thống trả về mã trạng thái 404 (Not Found)
    And hệ thống trả về JSON chứa "error: Task not found or not authorized"
    And giao diện hiển thị thông báo lỗi
    And giao diện tải lại danh sách để hoàn tác thay đổi trên dropdown

  Scenario: Không thể cập nhật công việc của người dùng khác
    Given người dùng user_id=1 cố gắng cập nhật task thuộc user_id=2
    Then hệ thống kiểm tra điều kiện WHERE id = ? AND user_id = ?
    And kết quả rows_affected = 0
    And hệ thống trả về mã trạng thái 404
```

**Quy tắc nghiệp vụ:**
- Các giá trị trạng thái hợp lệ: `pending` · `in_progress` · `done`
- Chỉ chủ sở hữu mới có thể cập nhật trạng thái (kiểm tra `user_id`)
- Khi cập nhật thất bại, giao diện **tự động hoàn tác** bằng cách tải lại danh sách

---

#### US-07: Xóa công việc

> **As a** người dùng đã đăng nhập,
> **I want to** xóa một công việc không còn cần thiết,
> **So that** danh sách công việc luôn gọn gàng.

**Acceptance Criteria:**

```gherkin
Feature: Xóa công việc

  Scenario: Xóa công việc thành công
    Given người dùng đã đăng nhập với user_id = 1
    And có công việc task_id=5 thuộc về user_id = 1
    When người dùng nhấn nút "Delete" trên công việc task_id=5
    Then giao diện hiển thị hộp thoại xác nhận "Are you sure you want to delete this task?"
    When người dùng nhấn "OK" để xác nhận
    Then hệ thống gửi DELETE request đến /api/tasks/5
    And hệ thống xóa công việc khỏi CSDL
    And hệ thống trả về mã trạng thái 200 (OK)
    And giao diện hiển thị thông báo "Task deleted successfully!"
    And giao diện tự động tải lại danh sách công việc

  Scenario: Hủy bỏ thao tác xóa
    Given người dùng nhấn nút "Delete" trên một công việc
    When hộp thoại xác nhận hiện ra
    And người dùng nhấn "Cancel"
    Then hệ thống KHÔNG gửi request đến server
    And danh sách công việc giữ nguyên

  Scenario: Xóa thất bại khi task không tồn tại
    Given người dùng gửi request xóa cho task_id=999 (không tồn tại)
    Then hệ thống trả về mã trạng thái 404 (Not Found)
    And hệ thống trả về JSON chứa "error: Task not found or not authorized"

  Scenario: Không thể xóa công việc của người dùng khác
    Given người dùng user_id=1 cố gắng xóa task thuộc user_id=2
    Then hệ thống kiểm tra điều kiện WHERE id = ? AND user_id = ?
    And kết quả rows_affected = 0
    And hệ thống trả về mã trạng thái 404
```

**Quy tắc nghiệp vụ:**
- Yêu cầu **xác nhận trước khi xóa** (confirm dialog)
- Chỉ chủ sở hữu mới có thể xóa (kiểm tra `user_id`)
- Xóa là **vĩnh viễn** (hard delete), không hỗ trợ khôi phục

---

## 5. 🔒 Yêu cầu bảo mật

| # | Yêu cầu | Trạng thái |
|---|---------|-----------|
| 1 | Mật khẩu phải được mã hóa bằng **bcrypt** trước khi lưu | ✅ Đã triển khai |
| 2 | Mật khẩu phải chứa ký tự **@** (quy tắc tùy chỉnh) | ✅ Đã triển khai |
| 3 | Thông báo lỗi đăng nhập **không phân biệt** sai user hay sai pass | ✅ Đã triển khai |
| 4 | Người dùng chỉ thao tác được trên **dữ liệu của chính mình** | ✅ Đã triển khai |
| 5 | Frontend có hàm **escapeHtml()** chống tấn công XSS | ✅ Đã triển khai |

---

## 6. 📋 Tổng hợp User Stories

| Mã | User Story | Độ ưu tiên | Trạng thái |
|----|-----------|-----------|-----------|
| US-01 | Đăng ký tài khoản mới | 🔴 Cao | ✅ Hoàn thành |
| US-02 | Đăng nhập vào hệ thống | 🔴 Cao | ✅ Hoàn thành |
| US-03 | Đăng xuất khỏi hệ thống | 🟡 Trung bình | ✅ Hoàn thành |
| US-04 | Thêm công việc mới | 🔴 Cao | ✅ Hoàn thành |
| US-05 | Xem danh sách công việc | 🔴 Cao | ✅ Hoàn thành |
| US-06 | Cập nhật trạng thái công việc | 🔴 Cao | ✅ Hoàn thành |
| US-07 | Xóa công việc | 🟡 Trung bình | ✅ Hoàn thành |

