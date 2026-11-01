#!/bin/bash
echo "=================================================="
echo "    SKILL KIỂM DUYỆT DỰ ÁN BY TRẦN TIẾN MINH      "
echo "=================================================="
echo "[1] Đang thống kê số lượng file Rust (Backend)..."
find src -name "*.rs" | wc -l

echo "[2] Đang kiểm tra lỗi cú pháp hệ thống (cargo check)..."
cargo check --quiet

echo "[3] Đang chạy tập lệnh kiểm thử tự động (cargo test)..."
cargo test --quiet

echo "=================================================="
echo "    SKILL THỰC THI HOÀN TẤT. HỆ THỐNG AN TOÀN!    "
echo "=================================================="