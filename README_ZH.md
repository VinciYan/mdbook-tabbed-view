# mdbook-tabbed-view

一个mdbook插件用于实现Tabbed View

## 安装

将如下配置添加到`book.toml`:

```toml
[preprocessor.tabbed-view]
command = "mdbook-tabbed-view"
```

## 使用

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

## 开发参考

- [CSS-Tabs](https://github.com/johnuberbacher/CSS-Tabs.git)
