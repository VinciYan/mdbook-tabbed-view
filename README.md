# mdbook-tabbed-view[中文](./README_ZH.md)

An mdbook plugin for Tabbed View.

## Installation

Add it as a preprocessor to your `book.toml`:

```toml
[preprocessor.tabbed-view]
command = "mdbook-tabbed-view"
```

## Usage

<pre>
{{tab-wrap}}
{{tab_content Title="第一部分标题"}}
第一部分内容
```rust
let mut result = String::new();
```
{{/tab_content}}
{{tab_content Title="第二部分标题"}}
第二部分内容
```c#
using DevExpress.Mvvm;
```
{{/tab_content}}
{{/tab-wrap}}
</pre>

## Reference

- [CSS-Tabs](https://github.com/johnuberbacher/CSS-Tabs.git)