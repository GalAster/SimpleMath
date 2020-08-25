
函数是指一个符号后面加若干个括号构成的表达式

```sm
f(arg1, arg2, kws*)(arg1, arg2)
```

这种形式被称为**高秩函数(Rank-k Type)**, 括号的数量就是这个函数的秩

sm 在尝试简化函数时会从高秩规则开始匹配, 然后逐步匹配低秩规则, 穷尽规则就会停止化简

如果秩降为零, 那么就会退化为符号.


```sm
f(x_, y_)(z_) := rule_1 % Rank-2
f(x_, y_) := rule_2     % Rank-1

% 高秩规则会优先匹配因此低秩规则不会生效
f(3, 5)(7) <=> rule_1
```


```sm
f(x_)(y_) := rule_1 % Rank-2
f(x_, y_) := rule_2 % Rank-1

% 高秩规则全部匹配失败才会尝试低秩规则
f(3, 5)(7) <=> rule_2(7)
```

参数项

每个参数包含若干个
- 必选项 (Arguments)
- 可选项 (Options)

如果这个函数不处理这个可选项那么会被忽略

sin(pi, base: 10)

```sm
#(a+b)&

#*(a+b)&
```