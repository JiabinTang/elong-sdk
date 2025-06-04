
# 艺龙开放平台 SDK

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-blue)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## 项目简介
该SDK提供与艺龙数据交互的异步接口，支持获取静态数据（如城市、酒店列表）和动态数据（如库存、价格增量）。适用于需要集成艺龙数据的第三方系统或应用。

---

## 功能模块
### 核心模块
| 模块          | 说明                          |
|---------------|-------------------------------|
| `network`     | 处理网络请求和响应的底层实现  |
| `request`     | 定义所有请求类型              |
| `response`    | 定义所有响应类型              |
| `types`       | 定义通用数据类型和结构        |
| `elong`       | 实现与Elong平台交互的核心逻辑 |

---

## 快速开始
### 安装依赖
cargo add elong-sdk


### 使用示例
#### 获取城市列表
```rust
use elong_sdk::Elong;
use request::static_city::StaticCityRequest;

#[tokio::main]
async fn main() {
    let service = ElongService::new();

    let request = StaticCityRequest {
        ..Default::default()
    };

    let result = service.get_static_city(request).await?;

    print!("result: {:?}", result);
}
```

---

## 下载与贡献
### 下载链接
- **GitHub仓库**：[https://github.com/JiabinTang/elong-sdk](https://github.com/JiabinTang/elong-sdk)  
- **CRATE版本**：[https://crates.io/crates/elong-sdk](https://crates.io/crates/elong-sdk)
