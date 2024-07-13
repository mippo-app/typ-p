#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Null,
    //is
    EQUAL,
    NotEqual,
    REGEX,
    GT,
    GTE,
    LT,
    LTE,
    IN,
    NotIn,
}

impl Operator {
    pub fn new(v: String) -> Self {
        if v == String::from("=") {
            return Operator::EQUAL;
        } else if v == String::from("like") {
            return Operator::REGEX;
        } else if v == String::from("<") {
            return Operator::GT;
        } else if v == String::from("<=") {
            return Operator::GTE;
        } else if v == String::from(">") {
            return Operator::LT;
        } else if v == String::from(">=") {
            return Operator::LTE;
        } else if v == String::from("<@") {
            return Operator::IN;
        }

        return Operator::Null;
    }
}

impl From<Operator> for String {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Null => String::from(""),
            Operator::EQUAL => String::from("=="),
            Operator::NotEqual => String::from(""),
            Operator::REGEX => String::from("like"),
            Operator::GT => String::from("<"),
            Operator::GTE => String::from("<="),
            Operator::LT => String::from(">"),
            Operator::LTE => String::from(">="),
            Operator::IN => String::from("<@"),
            Operator::NotIn => String::from(""),
        }
    }
}

impl Into<Operator> for String {
    fn into(self) -> Operator {
        if self == String::from("=") {
            return Operator::EQUAL;
        } else if self == String::from("like") {
            return Operator::REGEX;
        } else if self == String::from("<") {
            return Operator::GT;
        } else if self == String::from("<=") {
            return Operator::GTE;
        } else if self == String::from(">") {
            return Operator::LT;
        } else if self == String::from(">=") {
            return Operator::LTE;
        } else if self == String::from("<@") {
            return Operator::IN;
        }

        return Operator::Null;
    }
}
