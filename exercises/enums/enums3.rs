// enums3.rs
//
// 设计模式：
// 这是一个状态模式的实现，通过消息传递来修改状态
// 所有状态修改都集中在一个方法中管理
// 提供了良好的封装性和可维护性
// 使用场景：
// 游戏状态管理
// UI 事件处理
// 有限状态机实现
// 消息处理系统
// 这种设计模式在 Rust 中很常见，特别是在需要处理多种不同类型的消息或事件，并根据这些消息更新状态的场景中。它提供了一种清晰和类型安全的方式来处理状态转换。

enum Message {
    Move { x: u8, y: u8 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))
        //         这个方法通过 match 表达式处理四种不同类型的消息：
        // Move：更新位置状态
        // Echo：更新消息状态
        // ChangeColor：更新颜色状态
        // Quit：更新退出状态
        match message {
            Message::Move { x, y } => self.move_position(Point { x, y }),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move { x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
