use mdbook::book::Book;
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use uuid::Uuid;

pub struct TabbedViewPreprocessor;

impl Preprocessor for TabbedViewPreprocessor {
    fn name(&self) -> &str {
        "tabbed-view"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                let mut new_content = String::new();
                new_content.push_str(include_str!("css_template.html"));
                new_content.push('\n');
                
                new_content.push_str(&self.process_chapter(&chapter.content));
                new_content.push('\n');

                chapter.content = new_content;
            }
        });
        Ok(book)
    }
}

impl TabbedViewPreprocessor {
    fn process_chapter(&self, content: &str) -> String {
        let processed_content = convert_to_html(content);
        processed_content
    }
}

pub fn convert_to_html(input: &str) -> String {
    let tab_wrap_re = Regex::new(r"(?s)\{\{tab-wrap\}\}(.*?)\{\{/tab-wrap\}\}").unwrap();
    let tab_content_re = Regex::new(r#"(?s)\{\{tab_content Title="(.*?)"\}\}(.*?)\{\{/tab_content\}\}"#).unwrap();

    let mut result = String::new();
    let mut last_end = 0;

    for cap in tab_wrap_re.captures_iter(input) {
        let whole_match = cap.get(0).unwrap();

        // 添加tab-wrap之前的普通文本
        result.push_str(&input[last_end..whole_match.start()]);

        let tab_uuid = Uuid::new_v4();
        let tab_wrap_content = cap.get(1).unwrap().as_str();
        let mut tab_contents = Vec::new();
        let mut tab_labels = Vec::new();

        for (i, content_cap) in tab_content_re.captures_iter(tab_wrap_content).enumerate() {
            let title = content_cap.get(1).unwrap().as_str();
            let content = content_cap.get(2).unwrap().as_str().trim();

            tab_labels.push(format!(
                r#"<input type="radio" id="tab_{}_{}" name="tabGroup_{}" class="tab"{}>"#,
                tab_uuid,
                i + 1,
                tab_uuid,
                if i == 0 { " checked" } else { "" }
            ));
            tab_labels.push(format!(r#"<label for="tab_{}_{}">{}</label>"#, tab_uuid, i + 1, title));

            tab_contents.push(format!(
                r#"<div class="tab__content">

{}

</div>"#,
                content
            ));
        }

        result.push_str(&format!(
            r#"<div class="tab-wrap">
{}
{}
</div>"#,
            tab_labels.join("\n"),
            tab_contents.join("")
        ));

        last_end = whole_match.end();
    }

    // 添加最后一个tab-wrap之后的普通文本
    result.push_str(&input[last_end..]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_html_tab(){
        let input = r#"
first line
{{tab-wrap}}
{{tab_content Title="MainView.xaml"}}
```xml
<UserControl x:Class="Example.View.MainView"
</UserControl>
```
{{/tab_content}}
{{tab_content Title="MainViewModel.cs"}}
```c#
using DevExpress.Mvvm;
```
{{/tab_content}}
{{/tab-wrap}}
second line
{{tab-wrap}}
{{tab_content Title="MainView.xaml"}}
```xml
<UserControl x:Class="Example.View.MainView"
</UserControl>
```
{{/tab_content}}
{{tab_content Title="MainViewModel.cs"}}
```c#
using DevExpress.Mvvm;
```
{{/tab_content}}
{{/tab-wrap}}
third line
"#;
        let expected_output = r#"first line
<div class="tab-wrap">
<input type="radio" id="tab_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa_1" name="tabGroup_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa" class="tab" checked>
<label for="tab_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa_1">MainView.xaml</label>
<input type="radio" id="tab_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa_2" name="tabGroup_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa" class="tab">
<label for="tab_bd2ce0dd-f31f-4604-b0f3-69c77d3677fa_2">MainViewModel.cs</label>
<div class="tab__content">

```xml
<UserControl x:Class="Example.View.MainView"
</UserControl>
```

</div><div class="tab__content">

```c#
using DevExpress.Mvvm;
```

</div>
</div>
second line
<div class="tab-wrap">
<input type="radio" id="tab_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1_1" name="tabGroup_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1" class="tab" checked>
<label for="tab_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1_1">MainView.xaml</label>
<input type="radio" id="tab_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1_2" name="tabGroup_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1" class="tab">
<label for="tab_a81fbd5c-5ac8-4c5b-9d79-9a11f0c4ffc1_2">MainViewModel.cs</label>
<div class="tab__content">

```xml
<UserControl x:Class="Example.View.MainView"
</UserControl>
```

</div><div class="tab__content">

```c#
using DevExpress.Mvvm;
```

</div>
</div>
third line"#;
        let actual_output = convert_to_html(input);
        println!("actual output: {}", actual_output);
        // assert_eq!(actual_output, expected_output);
    }

}