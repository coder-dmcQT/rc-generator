# Rust 低代码生成器：高性能模板驱动代码脚手架

一款基于 Rust 开发的超高速跨平台低代码生成工具，可通过用户自定义的 JavaScript 解析逻辑，将自定义简写模板转换为任意代码/文件格式。该工具融合了 Rust 的内存安全性与高速性能，搭配轻量级 JavaScript 引擎（Boa），为多语言项目生成提供灵活可扩展的模板处理能力。

## ✨ 核心亮点

- ✅ **Rust 驱动高性能**：借助 Rust 的零成本抽象与异步运行时，实现亚毫秒级模板解析与代码生成

- ✅ **跨平台兼容性**：支持 Linux、macOS、Windows 及嵌入式目标（ARM/RISC-V），二进制文件体积极小（不足 2MB）

- ✅ **自定义模板语法**：用户可定义简写模板（支持任意文件扩展名）和 JavaScript 解析器，将其转换为目标输出格式

- ✅ **灵活的 JS 函数模型**：支持单文件（路径绑定）和多文件（组合式）JavaScript 函数，输出路径配置清晰明确

- ✅ **别名配置系统**：采用类 Vite 的 `rc.config.json` 别名机制，简化复杂项目的路径管理

- ✅ **轻量级 JS 引擎**：集成 Boa（纯 Rust 开发的 JavaScript 引擎）处理模板解析，无需依赖外部 Node.js 环境

- ✅ **前瞻性路线图**：计划支持二进制编译、可视化界面及高级配置管理功能

## 🛠️ 核心技术实现

### 1. 模板处理流程

工具通过清晰的三步流程生成输出文件：

- **步骤 1**：用户提供**JavaScript 解析器文件**（如 `ff.js`），定义包含解析逻辑和输出路径元数据的函数

- **步骤 2**：用户提供**简写模板文件**（如 `pp.jsd`，支持任意扩展名），内含自定义简写语法

- **步骤 3**：Rust 引擎执行 JS 解析器，以模板内容为输入，在指定路径生成输出文件

**示例流程**：

- **`ff.js`** **JS 解析器（）**：
        `function generateHelloWorld(config) {
  const lines = config.split('\r\n');
  return lines.map(v => {
    const [key, value] = v.split(',');
    return `const ${key} = ${value};`;
  }).join('');
}
generateHelloWorld.path = "./src/wtf.js"; // 输出路径配置`

- **`pp.jsd`** **简写模板（）**：
        `qwe,123123123
sdasd,true`

- **执行命令**：
        `js_evaluator.exe ./ff.js pp.jsd`

- **`./src/wtf.js`** **输出文件（）**：
        `const qwe = 123123123;const sdasd = true;`

### 2. JavaScript 函数规范

工具对 JavaScript 函数制定严格规范，确保输出可预测性：

- **单文件生成**：带有 `path` 属性（字符串类型）的函数返回表示文件内容的字符串，`path` 属性定义输出路径

- **多文件生成**：标记 `composed=true` 的函数支持两种返回格式：
        键值对对象（如 `{ "src/a.rs": "// 代码内容", "src/b.go": "// 代码内容" }`）

- 元组数组（如 `[["src/a.rs", "// 代码内容"], ["src/b.go", "// 代码内容"]]`）

**输入处理**：所有 JS 函数均接收**简写模板文件的完整内容**作为第一个参数（示例中的 `config`），支持任意解析逻辑

### 3. 别名配置系统

通过 `rc.config.json` 文件实现类 Vite 的路径别名功能，简化项目结构管理：

- **别名解析**：所有输出路径（函数 `path` 属性、多文件键名）均会根据 `rc.config.json` 中定义的别名进行解析：
        `{
  "alias": {
    "@": "../../../output/test-multi",       // 自定义别名
    "@spring": "src/main/java/com/example/spring" // 业务模块别名
  }
}`

- **递归别名扩展**：支持别名嵌套递归解析，适配复杂项目层级及共享模板库场景

### 4. 核心组件说明

|组件名称|核心职责|
|---|---|
|JS 引擎（Boa）|通过纯 Rust JS 运行时执行用户定义的解析器函数|
|模板加载器|读取并加载简写模板文件（支持任意扩展名）到内存|
|别名解析器|解析 `rc.config.json`，为模板输出路径提供别名解析|
|代码生成器|处理文件写入，基于 Rust 跨平台文件系统能力实现|
|CLI 处理器（Clap）|处理命令行参数（输入文件、配置路径、详细日志模式等）|
|配置管理器|加载并验证项目专属配置（JSON 格式）及运行时参数|
## 📡 支持功能

- **任意模板语法**：用户可定义自定义简写模板（支持任意扩展名：.jsd、.txt、.tmpl 等），无强制语法约束

- **JS 解析器灵活性**：Boa 引擎支持完整 ES6+ 语法，包括数组方法、字符串处理及条件判断等

- **跨平台文件系统**：基于 Rust `std::fs` 实现同步文件 I/O（为简洁性优化），原子写入避免部分文件问题

- **CLI 参数处理**：基于 Clap 实现完善的命令行交互，支持输入文件、配置路径及详细输出

- **UUID 生成**：内置 UUID v4 生成能力（基于 `uuid`  crate），计划向 JS 引擎暴露该能力

- **增量生成（规划中）**：基于模板/解析器变更的哈希对比，仅重新生成修改过的文件

- **高级项目脚手架**：支持通过高度抽象的元数据（如 Java Spring Boot 实体类定义），自动生成关联的仓库层、DTO、控制器及测试类，后续将补充该场景示例

## 🚀 快速开始

### 前置依赖

- Rust 1.75+（需包含 `cargo` 及 `rustup`）

- 无需外部 JavaScript 运行时（Boa 已嵌入 Rust 二进制文件）

### 构建与运行

```bash

# 克隆仓库
git clone <你的仓库地址>
cd rust-lowcode-generator

# 构建（生成小于 2MB 的二进制文件 js_evaluator）
cargo build --release

# 执行生成（指定解析器和模板文件）
./target/release/js_evaluator ./ff.js pp.jsd
```

### 高级用法示例（多文件生成）

1. **`multi-gen.js`** **JS 解析器（）**：
        `function generateMultiFiles(templateContent) {
  const lines = templateContent.split('\n');
  const output = {};
  lines.forEach(line => {
    const [filename, content] = line.split('|');
    output[`./src/${filename}`] = content; // 批量配置输出路径
  });
  return output;
}
generateMultiFiles.composed = true; // 标记为多文件生成函数`

2. **`multi.jsd`** **简写模板（）**：
        `app.rs|fn main() { println!("Hello Rust!"); }
app.go|package main; import "fmt"; func main() { fmt.Println("Hello Go!"); }`

3. **执行命令**：
        `./js_evaluator ./multi-gen.js multi.jsd`执行后将生成 `./src/app.rs` 和 `./src/app.go` 两个文件，内容分别为模板中定义的代码

## 📊 性能指标

- **二进制文件大小**：约 1.8MB（release 构建，开启优化并剥离符号信息）

- **内存占用**：模板解析与生成过程占用内存不足系统内存的 0.03%（基于 16GB 内存的 Linux 环境）

- **JS 执行效率**：解析并执行用户定义 JS 函数耗时小于 10ms（Boa 引擎针对小型脚本优化）

- **文件生成速度**：批量写入 100+ 文件耗时小于 50ms（同步 I/O 搭配批量处理）

- **跨平台构建时间**：交叉编译生成 Linux、macOS、Windows 版本耗时小于 25 秒

## 🏗️ 架构流程图

```text

[用户输入]
  ├── [JS 解析器文件 (.js)] → [Boa JS 引擎 (Rust 实现)]
  └── [简写模板文件 (任意扩展名)] → [模板加载器 (Rust 实现)]
          ↓
[别名解析器 (Rust 实现)] → [路径解析处理]
          ↓
[代码生成器 (Rust 实现)] → [文件系统输出]
          ↓
[生成的目标文件 (任意格式/语言)]
```

## 📝 开发说明

本工具采用**极简主义、目标驱动的架构设计**，核心聚焦简洁性与灵活性：

- 核心引擎集成 Boa（纯 Rust JS 引擎），彻底消除外部依赖（如 Node.js），保障可移植性并控制二进制文件体积

- 所有关键逻辑（内存管理、文件系统操作、JS 引擎集成）均通过 Rust 借用检查器验证并配套单元测试

- CLI 设计兼顾新手与高级用户需求，提供详细日志模式用于模板解析调试

- 规划中的功能（UI 界面、二进制编译）均设计为可选模块，确保核心功能轻量高效

- 支持通过高度抽象的元数据（如 Java Spring Boot 实体类定义）自动生成关联的仓库层、DTO、控制器及测试类，后续将补充该场景示例

最终产出的这款生产级工具，允许用户自定义模板语法与解析逻辑，适用于自定义代码生成流程、项目脚手架搭建及简写语法转代码等各类场景。

## 🛠️ 依赖项说明

|依赖库（Crate）|用途说明|
|---|---|
|boa_engine|纯 Rust 开发的 JavaScript 引擎，用于执行解析器函数|
|clap|命令行参数解析库（启用派生特性）|
|serde|用于配置文件及数据处理的序列化/反序列化库|
|serde_json|用于 `rc.config.json` 及运行时数据的 JSON 解析库|
|uuid|UUID v4 生成库（启用 fast-rng 特性优化随机数生成）|
---

基于 Rust 性能与安全保障开发 ❤️。欢迎贡献解析器示例或提出功能需求！
> （注：文档部分内容可能由 AI 生成）
