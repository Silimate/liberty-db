// use super::BooleanExpression;
// use boolean_expression::{Expr, BDD};
use biodivine_lib_bdd::boolean_expression::BooleanExpression as Expr;
use itertools::Itertools;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while},
  character::complete::{alpha1, alphanumeric0, char, digit1},
  combinator::map,
  multi::many1,
  sequence::{pair, preceded, tuple},
  IResult,
};
use std::collections::{HashSet, VecDeque};

#[inline]
pub(super) fn _fmt(expr: &Expr, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  match expr {
    Expr::Variable(s) => {
      if s.as_bytes()[0].is_ascii_digit() {
        write!(f, "\\\"{s}\\\"")
      } else {
        write!(f, "{s}")
      }
    }
    Expr::Const(b) => {
      if *b {
        write!(f, "1")
      } else {
        write!(f, "0")
      }
    }
    Expr::Not(e) => match **e {
      Expr::Variable(_) | Expr::Const(_) | Expr::Not(_) => {
        write!(f, "!")?;
        _fmt(e, f)
      }
      _ => {
        write!(f, "!(")?;
        _fmt(e, f)?;
        write!(f, ")")
      }
    },
    Expr::Or(e1, e2) => {
      _fmt(e1, f)?;
      write!(f, "|")?;
      _fmt(e2, f)
    }
    Expr::And(e1, e2) => {
      match **e1 {
        Expr::Or(_, _) => {
          write!(f, "(")?;
          _fmt(e1, f)?;
          write!(f, ")")?;
        }
        _ => {
          _fmt(e1, f)?;
        }
      };
      write!(f, "&")?;
      match **e2 {
        Expr::Or(_, _) => {
          write!(f, "(")?;
          _fmt(e2, f)?;
          write!(f, ")")
        }
        _ => _fmt(e2, f),
      }
    }
    Expr::Xor(e1, e2) => {
      match **e1 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e1, f)?;
          write!(f, ")")?;
        }
        _ => {
          _fmt(e1, f)?;
        }
      };
      write!(f, "^")?;
      match **e2 {
        Expr::Or(_, _) | Expr::And(_, _) => {
          write!(f, "(")?;
          _fmt(e2, f)?;
          write!(f, ")")
        }
        _ => _fmt(e2, f),
      }
    }
    Expr::Imp(_, _) => todo!(),
    Expr::Iff(_, _) => todo!(),
  }
}

#[derive(Debug, Clone)]
enum SingleOp {
  Not,
  // !A = A'
  BackNot,
  One,
  Zero,
}
#[derive(Debug, Clone)]
enum BinaryOp {
  And,
  Or,
  Xor,
}
#[derive(Debug, Clone)]
pub(super) enum Token {
  Blank,
  OpenBracket,
  CloseBracket,
  SingleOp(SingleOp),
  BinaryOp(BinaryOp),
  // Node(String),
  Expr(Expr),
}

#[inline]
fn space(i: &str) -> IResult<&str, ()> {
  map(take_while(move |c: char| matches!(c, '\t' | '\r' | ' ')), |_| ())(i)
}
fn open_b(i: &str) -> IResult<&str, Token> {
  map(char('('), |_| Token::OpenBracket)(i)
}
fn close_b(i: &str) -> IResult<&str, Token> {
  map(char(')'), |_| Token::CloseBracket)(i)
}
fn single_op(i: &str) -> IResult<&str, Token> {
  alt((
    map(tag("!!"), |_| Token::Blank),
    map(tag("\'\'"), |_| Token::Blank),
    map(tag("’’"), |_| Token::Blank),
    map(char('!'), |_| Token::SingleOp(SingleOp::Not)),
    map(char('\''), |_| Token::SingleOp(SingleOp::BackNot)),
    map(char('’'), |_| Token::SingleOp(SingleOp::BackNot)),
    map(char('0'), |_| Token::SingleOp(SingleOp::Zero)),
    map(char('1'), |_| Token::SingleOp(SingleOp::One)),
  ))(i)
}
fn binary_op(i: &str) -> IResult<&str, Token> {
  alt((
    map(char('+'), |_| Token::BinaryOp(BinaryOp::Or)),
    map(char('|'), |_| Token::BinaryOp(BinaryOp::Or)),
    map(char('&'), |_| Token::BinaryOp(BinaryOp::And)),
    map(char('*'), |_| Token::BinaryOp(BinaryOp::And)),
    map(char('^'), |_| Token::BinaryOp(BinaryOp::Xor)),
  ))(i)
}
fn node(i: &str) -> IResult<&str, Token> {
  alt((
    map(pair(alpha1, alphanumeric0), |(s1, s2)| {
      Token::Expr(Expr::Variable(format!("{s1}{s2}")))
    }),
    map(tuple((tag(r#"\""#), digit1, alphanumeric0, tag(r#"\""#))), |(_, s1, s2, _)| {
      Token::Expr(Expr::Variable(format!("{s1}{s2}")))
    }),
  ))(i)
}
pub(super) fn token_vec(i: &str) -> IResult<&str, Vec<Token>> {
  many1(preceded(space, alt((open_b, close_b, single_op, binary_op, node))))(i)
}

///
#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq)]
pub enum BoolExprErr {
  ///
  #[error("Lexing parser, Nom error")]
  Nom,
  ///
  #[error("right of single op is not {{signle op / expr}}")]
  SingleOp,
  ///
  #[error("binary_op left / right is not expr")]
  BinaryOp,
  ///
  #[error("left-right Bracket mismatch")]
  Bracket,
  ///
  #[error("something go wrong, {0}")]
  NoIdea(u8),
  ///
  #[error("Can not move back-not")]
  BackNot,
}

fn process_once(
  tokens: &mut VecDeque<Token>,
  left: usize,
  right: usize,
) -> Result<(), BoolExprErr> {
  fn _single_op(
    tokens: &mut VecDeque<Token>,
    pos: usize,
    reduce: &mut usize,
  ) -> Result<Expr, BoolExprErr> {
    match tokens.remove(pos) {
      Some(Token::SingleOp(SingleOp::Not)) => {
        *reduce += 1;
        let expr = _single_op(tokens, pos, reduce)?;
        Ok(Expr::Not(Box::new(expr)))
      }
      Some(Token::SingleOp(SingleOp::Zero)) => {
        *reduce += 1;
        let _ = _single_op(tokens, pos, reduce);
        Ok(Expr::Const(false))
      }
      Some(Token::SingleOp(SingleOp::One)) => {
        *reduce += 1;
        let _ = _single_op(tokens, pos, reduce);
        Ok(Expr::Const(true))
      }
      Some(Token::Expr(expr)) => {
        *reduce += 1;
        Ok(expr)
      }
      Some(other) => {
        tokens.insert(pos, other);
        Err(BoolExprErr::SingleOp)
      }
      _ => Err(BoolExprErr::SingleOp),
    }
  }
  fn _binary_op(
    tokens: &mut VecDeque<Token>,
    pos: usize,
    reduce: &mut usize,
  ) -> Result<(Expr, Expr), BoolExprErr> {
    // 3 -> 1
    *reduce += 2;
    let b = tokens.remove(pos + 1);
    let _ = tokens.remove(pos);
    let a = tokens.remove(pos - 1);
    match (a, b) {
      (Some(Token::Expr(expr_a)), Some(Token::Expr(expr_b))) => Ok((expr_a, expr_b)),
      _ => Err(BoolExprErr::BinaryOp),
    }
  }
  let d = right - left;
  let mut reduce = 0;
  // loop {

  //  println!("{:?}", tokens);
  // 1. find all single op
  let mut i1 = 0;
  loop {
    if i1 + reduce > d {
      break;
    }
    let pos = left + i1;
    match tokens.get(pos) {
      Some(Token::SingleOp(SingleOp::Not)) => {
        // 1 -> 1
        // *reduce += 0;
        let _ = tokens.remove(pos);
        let expr = _single_op(tokens, pos, &mut reduce)?;
        tokens.insert(pos, Token::Expr(Expr::Not(Box::new(expr))));
      }
      Some(Token::SingleOp(SingleOp::Zero)) => {
        let _ = tokens.remove(pos);
        let _ = _single_op(tokens, pos, &mut reduce);
        tokens.insert(pos, Token::Expr(Expr::Const(false)));
      }
      Some(Token::SingleOp(SingleOp::One)) => {
        let _ = tokens.remove(pos);
        let _ = _single_op(tokens, pos, &mut reduce);
        tokens.insert(pos, Token::Expr(Expr::Const(true)));
      }
      _ => (),
    }
    i1 += 1;
  }
  //  println!("{:?}", tokens);
  // 2. find all xor
  let mut i2 = 1;
  loop {
    if i2 + reduce > d {
      break;
    }
    let pos = left + i2;
    if let Some(Token::BinaryOp(BinaryOp::Xor)) = tokens.get(pos) {
      let (expr_a, expr_b) = _binary_op(tokens, pos, &mut reduce)?;
      tokens.insert(pos - 1, Token::Expr(Expr::Xor(Box::new(expr_a), Box::new(expr_b))));
    } else {
      i2 += 1
    }
  }
  //  println!("{:?}", tokens);
  // 3. find all and
  let mut i3 = 1;
  loop {
    if i3 + reduce > d {
      break;
    }
    //  println!("{:?}", tokens);
    let pos = left + i3;
    match tokens.get(pos) {
      Some(Token::BinaryOp(BinaryOp::And)) => {
        let (expr_a, expr_b) = _binary_op(tokens, pos, &mut reduce)?;
        tokens
          .insert(pos - 1, Token::Expr(Expr::And(Box::new(expr_a), Box::new(expr_b))));
      }
      // If we have (A B), recognize it as (A&B)
      Some(Token::Expr(_)) => {
        if let Some(Token::Expr(_)) = tokens.get(pos - 1) {
          // 2 -> 1
          reduce += 1;
          if let Some(Token::Expr(expr_b)) = tokens.remove(pos) {
            if let Some(Token::Expr(expr_a)) = tokens.remove(pos - 1) {
              tokens.insert(
                pos - 1,
                Token::Expr(Expr::And(Box::new(expr_a), Box::new(expr_b))),
              );
            };
          };
        } else {
          i3 += 1;
        }
      }
      _ => i3 += 1,
    }
  }
  //  println!("{:?}", tokens);
  // 4. find all or
  let mut i4 = 1;
  loop {
    if i4 + reduce > d {
      break;
    }
    let pos = left + i4;
    match tokens.get(pos) {
      Some(Token::BinaryOp(BinaryOp::Or)) => {
        let (expr_a, expr_b) = _binary_op(tokens, pos, &mut reduce)?;
        tokens.insert(pos - 1, Token::Expr(Expr::Or(Box::new(expr_a), Box::new(expr_b))));
      }
      _ => i4 += 1,
    }
  }
  //  println!("{:?}", tokens);
  if d - reduce == 0 {
    Ok(())
  } else {
    Err(BoolExprErr::NoIdea(0))
  }
}

// fn new_process_once(tokens: &mut VecDeque<Token>) -> Result<(), BoolExprErr> {
//   // 0. find all Bracket
//   'L0: for i in 0..tokens.len() {
//     match tokens.get(i) {
//       Some(Token::OpenBracket) => {
//         // remove this OpenBracket
//         let _ = tokens.remove(i);
//         new_process_once(tokens)?;
//         // now position i should be a expr,
//         // and position i+1 should be a CloseBracket
//         // remove that CloseBracket
//         let _next = tokens.remove(i + 1);
//         if !matches!(Some(Token::CloseBracket), _next) {
//           return Err(BoolExprErr::Bracket);
//         }
//       }
//       None => break 'L0,
//       _ => (),
//     }
//   }
//   // 1. find all single op
//   fn _single_op(tokens: &mut VecDeque<Token>, i: usize) -> Result<Expr, BoolExprErr> {
//     match tokens.get(i) {
//       Some(Token::SingleOp(SingleOp::Not)) => {
//         let _ = tokens.remove(i);
//         let expr = _single_op(tokens, i)?;
//         Ok(Expr::Not(Box::new(expr)))
//       }
//       Some(Token::SingleOp(SingleOp::Zero)) => {
//         let _ = tokens.remove(i);
//         let _ = _single_op(tokens, i);
//         Ok(Expr::Const(false))
//       }
//       Some(Token::SingleOp(SingleOp::One)) => {
//         let _ = tokens.remove(i);
//         let _ = _single_op(tokens, i);
//         Ok(Expr::Const(true))
//       }
//       Some(Token::Expr(_)) => {
//         if let Some(Token::Expr(e)) = tokens.remove(i) {
//           Ok(e)
//         } else {
//           Err(BoolExprErr::NoIdea(2))
//         }
//       }
//       _ => Err(BoolExprErr::SingleOp),
//     }
//   }
//   'L1: for i in 0..tokens.len() {
//     match tokens.get(i) {
//       Some(Token::SingleOp(op)) => {
//         // remove this OpenBracket
//         let _ = tokens.remove(i);
//         new_process_once(tokens)?;
//         // now position i should be a expr,
//         // and position i+1 should be a CloseBracket
//         // remove that CloseBracket
//         let _next = tokens.remove(i + 1);
//         if !matches!(Some(Token::CloseBracket), _next) {
//           return Err(BoolExprErr::Bracket);
//         }
//       }
//       None => break 'L1,
//       _ => (),
//     }
//   }
//   todo!();
// }
// pub(super) fn new_process_tokens(
//   tokens: &mut VecDeque<Token>,
// ) -> Result<Expr, BoolExprErr> {
//   //  println!("{:?}", tokens);
//   let _ = pre_process_tokens(tokens)?;
//   new_process_once(tokens)?;
//   if tokens.len() == 1 {
//     match tokens.remove(0) {
//       Some(Token::Expr(expr)) => Ok(expr),
//       _ => Err(BoolExprErr::NoIdea(2)),
//     }
//   } else {
//     Err(BoolExprErr::NoIdea(0))
//   }
// }
pub(super) fn process_tokens(tokens: &mut VecDeque<Token>) -> Result<Expr, BoolExprErr> {
  //  println!("{:?}", tokens);
  let (mut left, mut right) = pre_process_tokens(tokens)?;
  //  println!("{:?}", tokens);
  loop {
    //  println!("{:?}", tokens);
    process_once(tokens, left, right)?;
    right = left;
    let len = tokens.len();
    if len == 1 {
      break;
    }
    let mut new_left = None;
    let mut new_right = None;
    //  println!("{:?}", tokens);
    for i_left in (0..left).rev() {
      match tokens.get(i_left) {
        Some(Token::OpenBracket) => {
          new_left = Some(i_left);
          break;
        }
        _ => (),
      }
    }
    for i_right in (if let Some(l) = new_left { l } else { right })..len {
      match tokens.get(i_right) {
        Some(Token::CloseBracket) => {
          new_right = Some(i_right);
          break;
        }
        _ => (),
      }
    }
    match (new_left, new_right) {
      (Some(l), Some(r)) => {
        (left, right) = (l, r);
        let _ = tokens.remove(right);
        let _ = tokens.remove(left);
        right -= 2;
      }
      (None, None) => (left, right) = (0, len - 1),
      _ => return Err(BoolExprErr::Bracket),
    }
  }

  match tokens.remove(0) {
    Some(Token::Expr(expr)) => Ok(expr),
    _ => Err(BoolExprErr::NoIdea(2)),
  }
}

pub(super) fn get_nodes(expr: &Expr, node_set: &mut HashSet<String>) {
  match expr {
    Expr::Const(_) => (),
    Expr::Imp(_, _) => todo!(),
    Expr::Iff(_, _) => todo!(),
    Expr::Variable(node) => {
      let _ = node_set.insert(node.to_string());
    }
    Expr::Not(e) => get_nodes(e, node_set),
    Expr::And(e1, e2) => {
      get_nodes(e1, node_set);
      get_nodes(e2, node_set);
    }
    Expr::Or(e1, e2) => {
      get_nodes(e1, node_set);
      get_nodes(e2, node_set);
    }
    Expr::Xor(e1, e2) => {
      get_nodes(e1, node_set);
      get_nodes(e2, node_set);
    }
  }
}
fn pre_process_tokens(
  tokens: &mut VecDeque<Token>,
) -> Result<(usize, usize), BoolExprErr> {
  // Remove Blank
  for i in (0..tokens.len()).rev() {
    match tokens.get(i) {
      Some(Token::Blank) => {
        let _ = tokens.remove(i);
      }
      _ => (),
    }
  }
  //  println!("{:?}", tokens);
  // Find BackNot and then move it
  // A' -> !A
  // (A)' -> !(A)
  // ((A))' -> !((A))
  'L: for i in (0..tokens.len()).rev() {
    match tokens.get(i) {
      Some(Token::SingleOp(SingleOp::BackNot)) => {
        let mut pos = 0;
        let mut can_move = false;
        for j in (0..i).rev() {
          match tokens.get(j) {
            Some(Token::CloseBracket) => pos += 1,
            Some(Token::OpenBracket) => pos -= 1,
            Some(Token::Expr(_)) => can_move = true,
            _ => (),
          }
          if pos == 0 && can_move {
            let _ = tokens.remove(i);
            tokens.insert(j, Token::SingleOp(SingleOp::Not));
            continue 'L;
          }
        }
        return Err(BoolExprErr::BackNot);
      }
      _ => (),
    }
  }
  let mut left = None;
  let mut right = None;
  for i in (0..tokens.len()).rev() {
    match tokens.get(i) {
      Some(Token::OpenBracket) => {
        if left.is_none() {
          left = Some(i);
        }
      }
      Some(Token::CloseBracket) => {
        if left.is_none() {
          right = Some(i);
        }
      }
      _ => (),
    }
  }
  match (left, right) {
    (Some(_left), Some(_right)) => {
      let _ = tokens.remove(_right);
      let _ = tokens.remove(_left);
      Ok((_left, _right - 2))
    }
    (None, None) => Ok((0, tokens.len() - 1)),
    _ => Err(BoolExprErr::Bracket),
  }
}