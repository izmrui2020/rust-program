use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod mod_traits {
    pub trait Out {
        fn print_twice(&self);
        fn print_add_doller(&self);
    }
}
use mod_traits::Out;

#[derive(Debug)]
pub struct Human {
    hogehoge: i64,
}
#[derive(Debug)]
pub struct Lion {
    hogehoge: String,
}

#[derive(Debug)]
pub struct StaplFood {
    // food_store: HashMap<Animal, Vec<Animal>>,
}

impl StaplFood {
    fn yen_mark(&self) {
        println!("¥ {:?}", self.food_store);
    }
}

impl Out for StaplFood {
    fn print_twice(&self) {
        println!("{:?}, {:?}", self.food_store, self.food_store);
    }
    fn print_add_doller(&self) {
        println!("$ {:?}", self.food_store);
    }
}
///////////////////////////////////

enum Expression {
    SomeValue,
    PrefixOperator(Box<Expression>),
    BinaryOperator(Box<Expression>, Box<Expression>),
}

// impl Expression {
//     pub fn map<F: Copy + FnMut(Expression) -> Expression>(self, mut f: F) -> Expression {
//         match self {
//             Expression::SomeValue => Expression::SomeValue,
//             Expression::BinaryOperator(e1, e2) => {
//                 Expression::BinaryOperator(Box::new(f(*e1)), Box::new(f(*e2)))
//             }
//             Expression::PrefixOperator(e) => Expression::PrefixOperator(Box::new(f(*e))),
//         }
//     }
// }

// fn func(e: Expression) -> Expression {
//     if let Expression::SomeValue = e {
//         // ここにSomeValueの最適化処理をかく
//         Expression::SomeValue
//     } else {
//         e.map(|e| func(e))
//     }

impl Expression {
    pub fn iter<'a>(&'a self) -> std::vec::IntoIter<&'a Expression> {
        let v = match self {
            Expression::SomeValue => Vec::new(),
            Expression::BinaryOperator(e1, e2) => vec![e1.as_ref(), e2.as_ref()],
            Expression::PrefixOperator(e) => vec![e.as_ref()],
        };
        v.into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::vec::IntoIter<&'a mut Expression> {
        let v = match self {
            Expression::SomeValue => Vec::new(),
            Expression::BinaryOperator(e1, e2) => vec![e1.as_mut(), e2.as_mut()],
            Expression::PrefixOperator(e) => vec![e.as_mut()],
        };
        v.into_iter()
    }
}

impl IntoIterator for Expression {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        let v = match self {
            Expression::SomeValue => Vec::new(),
            Expression::BinaryOperator(e1, e2) => vec![*e1, *e2],
            Expression::PrefixOperator(e) => vec![*e],
        };
        v.into_iter()
    }
}

fn main() {
    let input = Expression::SomeValue.iter();

    // let output = if let Some(val) = input {
    //     Some(func(val))
    // } else {
    //     None
    // };
}
