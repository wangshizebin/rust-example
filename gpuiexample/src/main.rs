use gpui::*;
use gpui_platform::application;

// =====================================================
// 菜单动作
// =====================================================

actions!(calculator, [Quit, CopyResult, PasteResult, AboutCalculator]);

// =====================================================
// 计算器全局状态
// =====================================================

struct CalculatorGlobal {
    state: Entity<CalcState>,

    // 关于窗口句柄：最多只允许存在一个
    about_window: Option<WindowHandle<AboutView>>,
}

impl Global for CalculatorGlobal {}

// =====================================================
// 菜单动作处理
// =====================================================

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn copy_result(_: &CopyResult, cx: &mut App) {
    let state = cx.global::<CalculatorGlobal>().state.clone();
    let display = state.read(cx).display.clone();

    cx.write_to_clipboard(ClipboardItem::new_string(display.into()));
}

fn paste_result(_: &PasteResult, cx: &mut App) {
    let Some(item) = cx.read_from_clipboard() else {
        return;
    };

    let Some(text) = item.text() else {
        return;
    };

    let text = text.trim();

    let Ok(number) = text.parse::<f64>() else {
        return;
    };

    if !number.is_finite() {
        return;
    }

    let state = cx.global::<CalculatorGlobal>().state.clone();

    state.update(cx, |state, cx| {
        state.display = CalcState::format_number(number);
        state.expression.clear();
        state.reset_next = false;
        state.result_shown = false;

        cx.notify();
    });
}

// =====================================================
// 关于窗口：单例模式
// =====================================================

fn about_calculator(_: &AboutCalculator, cx: &mut App) {
    // 1. 获取已经保存的窗口句柄
    let existing_window = cx.global::<CalculatorGlobal>().about_window;

    // 2. 如果窗口存在，尝试激活
    if let Some(handle) = existing_window {
        let result = handle.update(cx, |_view, window, _cx| {
            // 激活已经存在的窗口
            window.activate_window();
        });

        if result.is_ok() {
            // 窗口仍然存在，不再创建
            return;
        }

        // 窗口已经关闭，清理失效句柄
        cx.global_mut::<CalculatorGlobal>().about_window = None;
    }

    // 3. 窗口不存在，创建新窗口
    let bounds = Bounds::centered(None, size(px(300.0), px(180.0)), cx);

    let result = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: WindowKind::Normal,
            is_resizable: false,
            titlebar: Some(TitlebarOptions {
                title: Some("关于计算器".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        },
        |_window, cx| cx.new(|_| AboutView),
    );

    // 4. 保存新窗口句柄
    match result {
        Ok(handle) => {
            cx.global_mut::<CalculatorGlobal>().about_window = Some(handle);
        }

        Err(error) => {
            eprintln!("打开关于窗口失败: {error}");
        }
    }
}

// =====================================================
// 关于窗口
// =====================================================

struct AboutView;

impl Render for AboutView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x171717))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(25.0))
                    .text_color(rgb(0xffffff))
                    .child("计算器"),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xA3A3A3))
                    .child("版本 1.0.0"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x737373))
                    .child("简单、便捷的计算工具"),
            )
    }
}

// =====================================================
// 计算器操作
// =====================================================

#[derive(Clone, Copy)]
enum CalcAction {
    Digit(char),
    Operator(char),
    Decimal,
    ToggleSign,
    Clear,
    Backspace,
    Equals,
}

#[derive(Clone, Copy)]
enum Token {
    Number(f64),
    Operator(char),
}

// =====================================================
// 表达式解析器：实现先乘除、后加减
// =====================================================

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(expression: &str) -> Option<Self> {
        let mut tokens = Vec::new();
        let bytes = expression.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let ch = bytes[i] as char;

            if ch.is_ascii_whitespace() {
                i += 1;
                continue;
            }

            if ch.is_ascii_digit() || ch == '.' {
                let start = i;
                let mut dots = 0;

                while i < bytes.len() {
                    let c = bytes[i] as char;

                    if c.is_ascii_digit() {
                        i += 1;
                    } else if c == '.' {
                        dots += 1;

                        if dots > 1 {
                            return None;
                        }

                        i += 1;
                    } else {
                        break;
                    }
                }

                let number = expression[start..i].parse::<f64>().ok()?;

                if !number.is_finite() {
                    return None;
                }

                tokens.push(Token::Number(number));
            } else if "+-*/".contains(ch) {
                tokens.push(Token::Operator(ch));
                i += 1;
            } else {
                return None;
            }
        }

        Some(Self {
            tokens,
            position: 0,
        })
    }

    // expression = term (+ term | - term)*
    fn parse_expression(&mut self) -> Option<f64> {
        let mut result = self.parse_term()?;

        loop {
            match self.tokens.get(self.position) {
                Some(Token::Operator('+')) => {
                    self.position += 1;
                    result += self.parse_term()?;
                }

                Some(Token::Operator('-')) => {
                    self.position += 1;
                    result -= self.parse_term()?;
                }

                _ => break,
            }
        }

        Some(result)
    }

    // term = factor (* factor | / factor)*
    fn parse_term(&mut self) -> Option<f64> {
        let mut result = self.parse_factor()?;

        loop {
            match self.tokens.get(self.position) {
                Some(Token::Operator('*')) => {
                    self.position += 1;
                    result *= self.parse_factor()?;
                }

                Some(Token::Operator('/')) => {
                    self.position += 1;

                    let divisor = self.parse_factor()?;

                    if divisor == 0.0 {
                        return None;
                    }

                    result /= divisor;
                }

                _ => break,
            }
        }

        Some(result)
    }

    // factor = number | +factor | -factor
    fn parse_factor(&mut self) -> Option<f64> {
        match self.tokens.get(self.position).copied()? {
            Token::Number(value) => {
                self.position += 1;
                Some(value)
            }

            Token::Operator('-') => {
                self.position += 1;
                Some(-self.parse_factor()?)
            }

            Token::Operator('+') => {
                self.position += 1;
                self.parse_factor()
            }

            _ => None,
        }
    }

    fn evaluate(expression: &str) -> Option<f64> {
        let mut parser = Self::new(expression)?;

        let result = parser.parse_expression()?;

        if parser.position != parser.tokens.len() {
            return None;
        }

        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
}

// =====================================================
// 计算器状态
// =====================================================

#[derive(Default)]
struct CalcState {
    // 当前显示的数字
    display: String,

    // 已输入的表达式
    expression: String,

    // 是否开始输入下一个数字
    reset_next: bool,

    // 是否刚刚完成计算
    result_shown: bool,
}

impl CalcState {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            ..Default::default()
        }
    }

    fn press(&mut self, action: CalcAction) {
        match action {
            // 输入数字
            CalcAction::Digit(digit) => {
                if self.display == "错误" || self.reset_next {
                    if self.result_shown {
                        self.expression.clear();
                        self.result_shown = false;
                    }

                    self.display = digit.to_string();
                    self.reset_next = false;
                } else if self.display == "0" {
                    self.display = digit.to_string();
                } else {
                    self.display.push(digit);
                }
            }

            // 小数点
            CalcAction::Decimal => {
                if self.display == "错误" || self.reset_next {
                    if self.result_shown {
                        self.expression.clear();
                        self.result_shown = false;
                    }

                    self.display = "0.".to_string();
                    self.reset_next = false;
                } else if !self.display.contains('.') {
                    self.display.push('.');
                }
            }

            // 正负号
            CalcAction::ToggleSign => {
                if self.display == "错误" {
                    return;
                }

                if self.result_shown {
                    self.expression.clear();
                    self.result_shown = false;
                    self.reset_next = false;
                }

                if self.display == "0" || self.display == "0." {
                    return;
                }

                if self.display.starts_with('-') {
                    self.display.remove(0);
                } else {
                    self.display.insert(0, '-');
                }
            }

            // 四则运算符
            CalcAction::Operator(op) => {
                if self.display == "错误" {
                    return;
                }

                if self.result_shown {
                    self.expression = format!("{}{}", self.display, op);
                    self.result_shown = false;
                } else if self.reset_next {
                    self.expression.pop();
                    self.expression.push(op);
                } else {
                    self.expression.push_str(&self.display);
                    self.expression.push(op);
                }

                self.reset_next = true;
            }

            // 等于
            CalcAction::Equals => {
                if self.reset_next || self.display == "错误" {
                    return;
                }

                if self.expression.is_empty() {
                    return;
                }

                let full_expression = format!("{}{}", self.expression, self.display);

                if let Some(result) = Parser::evaluate(&full_expression) {
                    self.display = Self::format_number(result);
                    self.expression = full_expression;
                    self.result_shown = true;
                } else {
                    self.display = "错误".to_string();
                    self.expression = full_expression;
                    self.result_shown = true;
                }

                self.reset_next = true;
            }

            // 清空
            CalcAction::Clear => {
                self.display = "0".to_string();
                self.expression.clear();
                self.reset_next = false;
                self.result_shown = false;
            }

            // 删除
            CalcAction::Backspace => {
                if !self.reset_next && !self.result_shown {
                    self.display.pop();

                    if self.display.is_empty() || self.display == "-" {
                        self.display = "0".to_string();
                    }
                }
            }
        }
    }

    // 格式化数字
    fn format_number(value: f64) -> String {
        if value.abs() < 0.000000001 {
            return "0".to_string();
        }

        if value.fract() == 0.0 {
            format!("{:.0}", value)
        } else {
            format!("{:.8}", value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        }
    }

    // 获取显示的完整表达式
    fn expression_display(&self) -> String {
        let mut expression = self.expression.clone();

        if !self.reset_next && !self.result_shown {
            expression.push_str(&self.display);
        }

        expression
            .replace('*', " × ")
            .replace('/', " ÷ ")
            .replace('+', " + ")
            .replace('-', " - ")
    }
}

// =====================================================
// GPUI 视图
// =====================================================

struct CalculatorView {
    state: Entity<CalcState>,
}

// =====================================================
// 调整按钮颜色
// =====================================================

fn adjust_color(color: u32, amount: i16) -> u32 {
    let r = ((color >> 16) & 0xff) as i16;
    let g = ((color >> 8) & 0xff) as i16;
    let b = (color & 0xff) as i16;

    let r = (r + amount).clamp(0, 255) as u32;
    let g = (g + amount).clamp(0, 255) as u32;
    let b = (b + amount).clamp(0, 255) as u32;

    (r << 16) | (g << 8) | b
}

// =====================================================
// 创建圆形或胶囊按钮
// =====================================================

fn calc_button(
    label: &'static str,
    color: u32,
    action: CalcAction,
    width: f32,
    cx: &mut Context<CalculatorView>,
) -> impl IntoElement {
    let hover_color = adjust_color(color, 18);
    let active_color = adjust_color(color, -25);

    div()
        .id(label)
        .w(px(width))
        .h(px(58.0))
        .bg(rgb(color))
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .text_color(rgb(0xffffff))
        .text_size(px(22.0))
        .hover(move |style| style.bg(rgb(hover_color)))
        .active(move |style| style.bg(rgb(active_color)))
        .child(label)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |state, _cx| {
                    state.press(action);
                });

                cx.notify();
            }),
        )
}

// =====================================================
// 普通按钮行
// =====================================================

fn button_row(
    buttons: &[(&'static str, u32, CalcAction)],
    cx: &mut Context<CalculatorView>,
) -> impl IntoElement {
    let mut row = div().w(px(262.0)).flex().flex_row().gap(px(10.0));

    for (label, color, action) in buttons {
        row = row.child(calc_button(*label, *color, *action, 58.0, cx));
    }

    row
}

// =====================================================
// 底部按钮
// =====================================================

fn bottom_button_row(cx: &mut Context<CalculatorView>) -> impl IntoElement {
    div()
        .w(px(262.0))
        .flex()
        .flex_row()
        .gap(px(5.0))
        .child(calc_button(
            "0",
            0x404040,
            CalcAction::Digit('0'),
            126.0,
            cx,
        ))
        .child(calc_button(".", 0x404040, CalcAction::Decimal, 63.0, cx))
        .child(calc_button("=", 0x16a34a, CalcAction::Equals, 63.0, cx))
}

// =====================================================
// Render
// =====================================================

impl Render for CalculatorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (display, expression) = {
            let state = self.state.read(cx);

            (
                SharedString::from(state.display.clone()),
                SharedString::from(state.expression_display()),
            )
        };

        div()
            .size_full()
            .bg(rgb(0x171717))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .pt(px(28.0))
            .gap(px(12.0))
            // 显示区域
            .child(
                div()
                    .w(px(262.0))
                    .h(px(100.0))
                    .bg(rgb(0x262626))
                    .rounded_lg()
                    .px(px(16.0))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .gap(px(8.0))
                    // 表达式
                    .child(
                        div()
                            .w_full()
                            .text_color(rgb(0xA3A3A3))
                            .text_size(px(15.0))
                            .text_right()
                            .child(expression),
                    )
                    // 计算结果
                    .child(
                        div()
                            .w_full()
                            .text_color(rgb(0xffffff))
                            .text_size(px(32.0))
                            .text_right()
                            .child(display),
                    ),
            )
            // 第一行：清空、删除、正负号、加
            .child(button_row(
                &[
                    ("C", 0xdc2626, CalcAction::Clear),
                    ("⌫", 0x525252, CalcAction::Backspace),
                    ("+/-", 0x525252, CalcAction::ToggleSign),
                    ("+", 0x2563eb, CalcAction::Operator('+')),
                ],
                cx,
            ))
            // 第二行：1、2、3、减
            .child(button_row(
                &[
                    ("1", 0x404040, CalcAction::Digit('1')),
                    ("2", 0x404040, CalcAction::Digit('2')),
                    ("3", 0x404040, CalcAction::Digit('3')),
                    ("-", 0x2563eb, CalcAction::Operator('-')),
                ],
                cx,
            ))
            // 第三行：4、5、6、乘
            .child(button_row(
                &[
                    ("4", 0x404040, CalcAction::Digit('4')),
                    ("5", 0x404040, CalcAction::Digit('5')),
                    ("6", 0x404040, CalcAction::Digit('6')),
                    ("×", 0x2563eb, CalcAction::Operator('*')),
                ],
                cx,
            ))
            // 第四行：7、8、9、除
            .child(button_row(
                &[
                    ("7", 0x404040, CalcAction::Digit('7')),
                    ("8", 0x404040, CalcAction::Digit('8')),
                    ("9", 0x404040, CalcAction::Digit('9')),
                    ("/", 0x2563eb, CalcAction::Operator('/')),
                ],
                cx,
            ))
            // 第五行：0、小数点、等于
            .child(bottom_button_row(cx))
    }
}

// =====================================================
// Main
// =====================================================

fn main() {
    application().run(|cx: &mut App| {
        // 创建计算器状态
        let state = cx.new(|_| CalcState::new());

        // 注册全局状态
        cx.set_global(CalculatorGlobal {
            state: state.clone(),
            about_window: None,
        });

        // 注册菜单动作
        cx.on_action(quit);
        cx.on_action(copy_result);
        cx.on_action(paste_result);
        cx.on_action(about_calculator);

        // 键盘快捷键
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-c", CopyResult, None),
            KeyBinding::new("cmd-v", PasteResult, None),
        ]);

        // macOS 原生菜单
        cx.set_menus(vec![
            Menu {
                name: "计算器".into(),
                disabled: false,
                items: vec![
                    MenuItem::action("关于计算器", AboutCalculator),
                    MenuItem::separator(),
                    MenuItem::action("退出计算器", Quit),
                ],
            },
            Menu {
                name: "编辑".into(),
                disabled: false,
                items: vec![
                    MenuItem::action("复制", CopyResult),
                    MenuItem::action("粘贴", PasteResult),
                ],
            },
        ]);

        // 创建计算器窗口
        let bounds = Bounds::centered(None, size(px(320.0), px(520.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_resizable: false,

                // macOS 透明标题栏
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
                }),

                ..Default::default()
            },
            |_window, cx| cx.new(|_| CalculatorView { state }),
        )
        .unwrap();

        cx.activate(true);
    });
}
