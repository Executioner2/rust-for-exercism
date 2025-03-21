use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type TokenId = usize;
type Tokens = HashMap<String, BTreeMap<TokenId, Token>>;

macro_rules! tokens {
    ( $($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key.to_string(), BTreeMap::from([(1, Token::new(vec![$value]))])); )*
        map
    }};
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operator {
    Plus,                    // +
    Minus,                   // -
    Multiply,                // *
    Divide,                  // /
    Dup,                     // 复制栈顶元素
    Drop,                    // 丢弃栈顶元素
    Swap,                    // 交换栈顶两个元素的位置
    Over,                    // 将栈顶的第二个元素复制到栈顶
    Push(Value),             // 直接压入一个值到栈顶
    Custom(String, TokenId), // 自定义操作符
}

struct Token {
    options: Vec<Operator>, // 操作符列表
    _ref_count: usize,      // 引用计数
}

impl Token {
    fn new(options: Vec<Operator>) -> Token {
        Token {
            options,
            _ref_count: 1,
        }
    }
}

pub struct Forth {
    stack: Vec<Value>,
    tokens: Tokens,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            tokens: Self::default_operators(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut is_define = false;
        let mut define = vec![];

        for s in input.split(" ") {
            let s = s.to_lowercase();
            if s == ":" {
                // 开始定义一个新的词法单元
                is_define = true;
                continue;
            } else if s == ";" {
                // 结束定义一个新的词法单元
                self.parse_define(&mut define)?;
                define.clear();
                is_define = false;
                continue;
            }
            if is_define {
                define.push(s.to_string());
                continue;
            }

            // 进行普通操作
            if let Ok(v) = s.parse::<Value>() {
                self.stack.push(v);
            } else if let Some(tree) = self.tokens.get(&s) {
                let op = tree.last_key_value().unwrap().1;
                Self::apply_operator(&mut self.stack, &self.tokens, &op.options)?;
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }

    fn arith<F: FnOnce(Value, Value) -> ::std::result::Result<Value, Error>>(stack: &mut Vec<Value>, f: F) -> Result {
        if stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(f(a, b)?);
        Ok(())
    }
    
    fn dup(stack: &mut Vec<Value>) -> Result {
        if stack.is_empty() {
            Err(Error::StackUnderflow)
        } else {
            stack.push(*stack.last().unwrap());
            Ok(())
        }
    }
    
    fn drop(stack: &mut Vec<Value>) -> Result {
        if stack.is_empty() {
            Err(Error::StackUnderflow)
        } else {
            stack.pop();
            Ok(())
        }
    }
    
    fn over(stack: &mut Vec<Value>) -> Result {
        if stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        stack.push(*stack.get(stack.len() - 2).unwrap());
        Ok(())
    }
    
    fn swap(stack: &mut Vec<Value>) -> Result {
        if stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(a);
        stack.push(b);
        Ok(())
    }
    
    fn custom(stack: &mut Vec<Value>, tokens: &Tokens, s: &String, id: TokenId) -> Result {
        if let Some(tree) = tokens.get(s) {
            if let Some(token) = tree.get(&id) {
                return Self::apply_operator(stack, tokens, &token.options);
            }
        }
        Err(Error::UnknownWord)
    }

    fn apply_operator(stack: &mut Vec<Value>, tokens: &Tokens, op: &Vec<Operator>) -> Result {
        for op in op {
            match op {
                Operator::Plus => Self::arith(stack, |a, b| Ok(a + b))?,
                Operator::Minus => Self::arith(stack, |a, b| Ok(a - b))?,
                Operator::Multiply => Self::arith(stack, |a, b| Ok(a * b))?,
                Operator::Divide => Self::arith(stack, |a, b| 
                    if b == 0 {
                        Err(Error::DivisionByZero)
                    } else {
                        Ok(a / b)
                    }
                )?,Operator::Dup => Self::dup(stack)?,
                Operator::Drop => Self::drop(stack)?,
                Operator::Over => Self::over(stack)?,
                Operator::Swap => Self::swap(stack)?,
                Operator::Push(v) => stack.push(*v),
                Operator::Custom(s, id) => Self::custom(stack, tokens, s, *id)?,
            }
        }
        Ok(())
    }

    fn parse_define(&mut self, define: &mut [String]) -> Result {
        let name = &define[0];
        if name.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let name = name.to_string();

        let mut options: Vec<Operator> = vec![];
        for s in define[1..].iter() {
            if let Ok(v) = s.parse::<Value>() {
                options.push(Operator::Push(v));
            } else if let Some(tree) = self.tokens.get(s) {
                options.push(Operator::Custom(
                    s.to_string(),
                    *tree.last_key_value().unwrap().0,
                ));
            } else {
                return Err(Error::UnknownWord);
            }
        }

        if let Some(tree) = self.tokens.get_mut(&name) {
            let id = tree.last_key_value().unwrap().0 + 1;
            tree.insert(id, Token::new(options));
        } else {
            let tree = BTreeMap::from([(1, Token::new(options))]);
            self.tokens.insert(name, tree);
        }
        Ok(())
    }

    fn default_operators() -> Tokens {
        tokens!("+".to_string() => Operator::Plus, "-".to_string() => Operator::Minus,
                "*".to_string() => Operator::Multiply, "/".to_string() => Operator::Divide,
                "dup".to_string() => Operator::Dup, "drop".to_string() => Operator::Drop,
                "swap".to_string() => Operator::Swap, "over".to_string() => Operator::Over
        )
    }
}
