
## 选项卡视图

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
{{tab_content Title="第三部分标题"}}
第三部分内容
```js
console.log("Hello world!");
```
{{/tab_content}}
{{/tab-wrap}}

更复杂的例子

{{tab-wrap}}
{{tab_content Title="MainViewModel.cs"}}
```c#
namespace Example.ViewModel {
    public class MainViewModel {
        public virtual string Parameter1 { get; set; }
        public virtual string Parameter2 { get; set; }
        public void ChangeParameter1() {
            counter1++;
            Parameter1 = "Parameter #" + counter1.ToString();
        }
        public void ChangeParameter2() {
            counter2++;
            Parameter2 = "Parameter #" + counter2.ToString();
        }

        int counter1 = 0;
        int counter2 = 0;
    }
}
```
{{/tab_content}}
{{tab_content Title="ChildViewModelBase.cs"}}
```c#
using DevExpress.Mvvm;

namespace Example.ViewModel {
    public class ChildViewModelBase : ViewModelBase {
        IMessageBoxService MessageBoxService { get { return GetService<IMessageBoxService>(ServiceSearchMode.PreferParents); } }
        protected override void OnParameterChanged(object parameter) {
            base.OnParameterChanged(parameter);
            if(parameter is string)
                MessageBoxService.Show("ChildViewModelBase: Parameter = " + (string)parameter);
        }
    }
}
```
{{/tab_content}}
{{tab_content Title="ChildPOCOViewModel.cs"}}
```c#
using DevExpress.Mvvm;
using DevExpress.Mvvm.DataAnnotations;

namespace Example.ViewModel {
    public class ChildPOCOViewModel : ISupportParameter {
        [ServiceProperty(SearchMode=ServiceSearchMode.PreferParents)]
        protected virtual IMessageBoxService MessageBoxService { get { return null; } }
        public virtual object Parameter { get; set; }

        protected virtual void OnParameterChanged() {
            if(Parameter is string)
                MessageBoxService.Show("ChildPOCOViewModel: Parameter = " + (string)Parameter);
        }
    }
}
```
{{/tab_content}}

{{tab_content Title="MainWindow.xaml"}}
```xml
<Window x:Class="Example.MainWindow"
        xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        xmlns:View="clr-namespace:Example.View"
        Title="MainWindow" Height="600" Width="800">
    <Grid>
        <View:MainView/>
    </Grid>
</Window>
```
{{/tab_content}}

{{tab_content Title="ChildView1.xaml"}}
```xml
<UserControl x:Class="Example.View.ChildView1"
             xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
             xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
             xmlns:ViewModel="clr-namespace:Example.ViewModel"
             xmlns:dxmvvm="http://schemas.devexpress.com/winfx/2008/xaml/mvvm"
             xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" 
             xmlns:d="http://schemas.microsoft.com/expression/blend/2008" 
             mc:Ignorable="d" 
             d:DesignHeight="300" d:DesignWidth="300">
    <UserControl.DataContext>
        <ViewModel:ChildViewModelBase/>
    </UserControl.DataContext>
    <Grid>
        <TextBlock Text="This View has a ViewModel derived from the ViewModelBase class" TextWrapping="Wrap"/>
    </Grid>
</UserControl>
```
{{/tab_content}}

{{tab_content Title="MainView.xaml"}}
```xml
<UserControl x:Class="Example.View.MainView"
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    xmlns:ViewModel="clr-namespace:Example.ViewModel"
    xmlns:View="clr-namespace:Example.View"
    xmlns:dxmvvm="http://schemas.devexpress.com/winfx/2008/xaml/mvvm"
    xmlns:dx="http://schemas.devexpress.com/winfx/2008/xaml/core"
    xmlns:d="http://schemas.microsoft.com/expression/blend/2008"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:Ignorable="d" d:DesignHeight="500" d:DesignWidth="600"
    DataContext="{dxmvvm:ViewModelSource ViewModel:MainViewModel}">

    <dxmvvm:Interaction.Behaviors>
        <dx:DXMessageBoxService/>
    </dxmvvm:Interaction.Behaviors>

    <Grid x:Name="LayoutRoot" Background="White">
        <StackPanel Orientation="Vertical">
            <StackPanel Orientation="Horizontal">
                <Button Content="Change Parameter1" Command="{Binding ChangeParameter1Command}"/>
                <Button Content="Change Parameter2" Command="{Binding ChangeParameter2Command}"/>
            </StackPanel>
            <StackPanel Orientation="Horizontal">
                <Border BorderThickness="1" BorderBrush="Black" Margin="10" Width="300" Height="100">
                    <View:ChildView1 dxmvvm:ViewModelExtensions.ParentViewModel="{Binding DataContext, ElementName=LayoutRoot}"
                                     dxmvvm:ViewModelExtensions.Parameter="{Binding DataContext.Parameter1, ElementName=LayoutRoot}"/>
                </Border>
            </StackPanel>
            <StackPanel Orientation="Horizontal">
                <Border BorderThickness="1" BorderBrush="Black" Margin="10" Width="300" Height="100">
                    <View:ChildView2 dxmvvm:ViewModelExtensions.ParentViewModel="{Binding DataContext, ElementName=LayoutRoot}"
                                     dxmvvm:ViewModelExtensions.Parameter="{Binding DataContext.Parameter2, ElementName=LayoutRoot}"/>
                </Border>
            </StackPanel>
        </StackPanel>
    </Grid>
</UserControl>
```
{{/tab_content}}

{{tab_content Title="ChildView2.xaml"}}
```xml
<UserControl x:Class="Example.View.ChildView2"
             xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
             xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
             xmlns:ViewModel="clr-namespace:Example.ViewModel"
             xmlns:dxmvvm="http://schemas.devexpress.com/winfx/2008/xaml/mvvm"
             xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" 
             xmlns:d="http://schemas.microsoft.com/expression/blend/2008" 
             mc:Ignorable="d" 
             d:DesignHeight="300" d:DesignWidth="300"
             DataContext="{dxmvvm:ViewModelSource ViewModel:ChildPOCOViewModel}">
    <Grid>
        <TextBlock Text="This View has a POCO ViewModel" TextWrapping="Wrap"/>
    </Grid>
</UserControl>
```
{{/tab_content}}

{{/tab-wrap}}


