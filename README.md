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
- `exercises/ch04-ownership/`：第4章練習 04-16 至 04-21 的起始函式與公開測試；04-22 為編譯錯誤診斷題。
- `exercises/ch05-borrowing/`：第5章練習 05-23 至 05-27 的起始函式與公開測試；05-28、05-29 為借用選擇與錯誤診斷題。
- `exercises/ch06-data-models/`：第6章練習 06-30 至 06-36 的起始函式與公開測試；完成後執行 `cargo test` 驗收。
- `exercises/ch07-collections/`：第7章練習 07-37 至 07-43 的起始函式與公開測試；完成後執行 `cargo test` 驗收。
- `exercises/ch08-errors/`：第8章練習 08-44 至 08-49 的起始函式與公開測試；完成後執行 `cargo test` 驗收。
- `exercises/ch09-iterators/`：第9章練習 09-50 至 09-55 的起始函式與公開測試；完成後執行 `cargo test` 驗收。
- `exercises/ch10-generics/`：第10章練習 10-56 至 10-61 的起始函式與公開測試；完成後執行 `cargo test` 驗收。
- `exercises/ch11-modules/`：第11章練習 11-62 至 11-66 的起始專案、單元／整合／文件測試；完成後執行 `cargo test` 驗收。
- `exercises/ch12-cli/`：第12章練習 12-67 至 12-71 的 Cargo 專案、library、binary 與程序整合測試；完成後執行 `cargo test` 驗收。

- `exercises/ch13-files/`：第13章練習 13-72 至 13-75，涵蓋 TSV 解析、行號錯誤、UTF-8 檔案讀取及相對路徑詞法檢查；四個 starter 測試預期因 `todo!()` RED。
- `exercises/ch14-pocket-ledger/`：整合專案 14-76 至 14-80；含 library/binary 起始碼、公開測試及合成 TSV fixture。完成後執行 `cargo test` 驗收。

## 回報問題

請附 Repo commit、練習編號、作業系統、`rustc --version`、`cargo --version`、執行命令及完整錯誤。不要附密碼、token、私鑰或個資。

## 授權

尚未指定 License。公開 Repo 不代表第三方自動取得再散布、修改或商業使用授權。出版前依出版負責人決策補充授權文件。
