// src/parser.rs

use std::fmt;
use crate::ast::{Expr, BinaryOp, Stmt, Program};
use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEOF,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          ParseError::UnexpectedToken => write!(f, "Unexpected token"),
          ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
      }
  }
}

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
  let mut pos = 0;
  let mut statements = Vec::new();

  while pos < tokens.len() {
      match parse_stmt(tokens, &mut pos) {
          Ok(stmt) => statements.push(stmt),
          Err(e) => return Err(e.to_string()),
      }
  }

  Ok(Program { statements })
}

fn parse_stmt(tokens: &[Token], pos: &mut usize) -> Result<Stmt, ParseError> {
    match tokens.get(*pos) {
        Some(Token::Let) => parse_let_stmt(tokens, pos),
        Some(Token::Func) => parse_func_def(tokens, pos),
        Some(Token::Print) => parse_print_stmt(tokens, pos),
        _ => parse_expr_stmt(tokens, pos),
    }
}

fn parse_let_stmt(tokens: &[Token], pos: &mut usize) -> Result<Stmt, ParseError> {
    *pos += 1;
    if let Some(Token::Ident(name)) = tokens.get(*pos).cloned() {
        *pos += 1;
        if let Some(Token::Equals) = tokens.get(*pos) {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            Ok(Stmt::Let(name, expr))
        } else {
            Err(ParseError::UnexpectedToken)
        }
    } else {
        Err(ParseError::UnexpectedToken)
    }
}

fn parse_print_stmt(tokens: &[Token], pos: &mut usize) -> Result<Stmt, ParseError> {
    *pos += 1;
    let expr = parse_expr(tokens, pos)?;
    Ok(Stmt::Print(expr))
}

fn parse_func_def(tokens: &[Token], pos: &mut usize) -> Result<Stmt, ParseError> {
    *pos += 1;
    let name = if let Some(Token::Ident(n)) = tokens.get(*pos).cloned() {
        *pos += 1;
        n
    } else {
        return Err(ParseError::UnexpectedToken);
    };

    expect_token(tokens, pos, Token::LParen)?;
    let mut params = Vec::new();
    while let Some(Token::Ident(p)) = tokens.get(*pos).cloned() {
        *pos += 1;
        params.push(p);
        if let Some(Token::Comma) = tokens.get(*pos) {
            *pos += 1;
        } else {
            break;
        }
    }
    expect_token(tokens, pos, Token::RParen)?;
    expect_token(tokens, pos, Token::LBrace)?;

    let mut body = Vec::new();
    while !matches!(tokens.get(*pos), Some(Token::RBrace)) {
        body.push(parse_stmt(tokens, pos)?);
    }
    expect_token(tokens, pos, Token::RBrace)?;

    Ok(Stmt::FuncDef { name, params, body })
}

fn expect_token(tokens: &[Token], pos: &mut usize, expected: Token) -> Result<(), ParseError> {
    if let Some(tok) = tokens.get(*pos) {
        if *tok == expected {
            *pos += 1;
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken)
        }
    } else {
        Err(ParseError::UnexpectedEOF)
    }
}

fn parse_expr_stmt(tokens: &[Token], pos: &mut usize) -> Result<Stmt, ParseError> {
    let expr = parse_expr(tokens, pos)?;
    Ok(Stmt::Expr(expr))
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
    parse_binary_expr(tokens, pos, 0)
}

fn parse_binary_expr(tokens: &[Token], pos: &mut usize, min_prec: u8) -> Result<Expr, ParseError> {
    let mut left = parse_primary(tokens, pos)?;

    while let Some(op) = match tokens.get(*pos) {
        Some(Token::Plus) => Some((1, BinaryOp::Add)),
        Some(Token::Minus) => Some((1, BinaryOp::Sub)),
        Some(Token::Star) => Some((2, BinaryOp::Mul)),
        Some(Token::Slash) => Some((2, BinaryOp::Div)),
        _ => None,
    } {
        if op.0 < min_prec {
            break;
        }
        *pos += 1;
        let right = parse_binary_expr(tokens, pos, op.0 + 1)?;
        left = Expr::Binary {
            left: Box::new(left),
            op: op.1,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
    match tokens.get(*pos) {
        Some(Token::Number(n)) => {
            *pos += 1;
            Ok(Expr::Number(*n))
        },
        Some(Token::String(s)) => {
            *pos += 1;
            Ok(Expr::String(s.clone()))
        },
        Some(Token::Ident(name)) => {
            let name = name.clone();
            *pos += 1;
            if let Some(Token::LParen) = tokens.get(*pos) {
                *pos += 1;
                let mut args = Vec::new();
                while !matches!(tokens.get(*pos), Some(Token::RParen)) {
                    let arg = parse_expr(tokens, pos)?;
                    args.push(arg);
                    if let Some(Token::Comma) = tokens.get(*pos) {
                        *pos += 1;
                    } else {
                        break;
                    }
                }
                expect_token(tokens, pos, Token::RParen)?;
                Ok(Expr::Call { name, args })
            } else {
                Ok(Expr::Ident(name))
            }
        }
        Some(Token::LParen) => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            expect_token(tokens, pos, Token::RParen)?;
            Ok(expr)
        }
        _ => Err(ParseError::UnexpectedToken),
    }
}