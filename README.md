# Rust 實例演習｜讀者配套資源

本公開 Repo 提供《Rust 實例演習：從所有權到 CLI 專案的 80 道練習》的起始碼、測試與合成資料，不包含書稿、內部審查或付費章節內容。

## 下載與執行

- ZIP：[下載 main 分支](https://github.com/t945935/rust-exercise-workbook-resources/archive/refs/heads/main.zip)
- Git：`git clone https://github.com/t945935/rust-exercise-workbook-resources.git`
- 工具鏈：Rust 1.99.1、Rust 2024 Edition（2026-10-21 作者驗證基準）；其他版本可能有不同錯誤訊息。

每章獨立練習位於 `exercises/`。例如：

```bash
cd exercises/ch01-error-loop
cargo test
```

起始程式的 `todo!()` 尚待讀者完成，所以初次執行時測試預期失敗；完成該章指定函式後再執行，測試應通過。各題的完整解答與教學說明留在書中，不放在此下載 Repo。

## 目前資源

- `exercises/ch01-error-loop/`：第1章練習 01-01 至 01-03 的起始函式與公開測試；練習 01-04、01-05 為讀錯誤訊息與診斷流程題，答案見書稿。
- `exercises/ch02-values/`：第2章五題的起始函式與公開測試，完成函式後執行 `cargo test` 驗收。
- `exercises/ch03-control-flow/`：第3章五題的起始函式與公開測試；依賴 `ch02-values` 的閏年函式。

後續資源依章稿完成逐步新增。題目、答案與測試需保持同一版本；下載時請查看 `rust-toolchain.toml`。資料均為合成資料，不需要帳號或真實個資。

## 回報問題

請附 Repo commit、練習編號、作業系統、`rustc --version`、`cargo --version`、執行命令及完整錯誤。不要附密碼、token、私鑰或個資。

## 授權

尚未指定 License。公開 Repo 不代表第三方自動取得再散布、修改或商業使用授權。出版前依出版負責人決策補充授權文件。
