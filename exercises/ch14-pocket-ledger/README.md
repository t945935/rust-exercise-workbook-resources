# pocket-ledger 練習專案

Rust 2024 的合成資料 CLI 練習。資料格式為每行「描述<Tab>整數分」，例如 `Coffee<Tab>850`。本工具不連接銀行、不讀取預設私人檔案、不提供財務建議。

使用 Rust 1.99.1（或符合 Rust 2024 Edition 的工具鏈）：

```sh
cargo test
cargo run -- check tests/fixtures/sample.tsv
cargo run -- total tests/fixtures/sample.tsv
```

起始碼含 `todo!()`，所以完成練習前測試預期失敗。完成後，`check` 應顯示筆數，`total` 輸出總分（整數分）。測試只使用本目錄的合成 fixture。

## 題目

14-76 資料模型與單筆解析；14-77 多行解析與錯誤行號；14-78 checked-add 加總；14-79 檔案讀取與命令處理；14-80 二進位錯誤路由、程序測試及文件完成。
