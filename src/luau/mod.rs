use std::collections::HashMap;
use full_moon::{ast::{Ast, Block, Prefix, Stmt, Suffix}, node::Node};
use crate::Backend;

type Range = (usize, usize);

fn range<N: Node>(node: N) -> (usize, usize) {
    let (start, end) = node.range().unwrap();
    (start.bytes(), end.bytes())
}

fn internal_find_from_visit<'a>(function_to_find: &str, block: &'a Block, usage_map: &mut HashMap<Range, Vec<&'a Suffix>>) {
    for stmt in block.stmts() {
        match stmt {
            Stmt::Do(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            }
            Stmt::FunctionCall(node) => {
                match node.prefix() {
                    Prefix::Name(token) => {
                        if &token.token().to_string() == function_to_find {
                            usage_map.insert(range(token), node.suffixes().collect());
                        }
                    },
                    _ => {}
                };
            },
            Stmt::FunctionDeclaration(node) => {
                internal_find_from_visit(function_to_find, node.body().block(), usage_map);
            }
            Stmt::GenericFor(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            },
            Stmt::If(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            },
            Stmt::LocalFunction(node) => {
                internal_find_from_visit(function_to_find, node.body().block(), usage_map);
            }
            Stmt::NumericFor(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            },
            Stmt::Repeat(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            },
            Stmt::While(node) => {
                internal_find_from_visit(function_to_find, node.block(), usage_map);
            },
            _ => {}
        };
    }
}

impl Backend {
    pub fn luau_ast_from_string(&self, source: &String) -> Result<Ast, Box<dyn std::error::Error>> {
        Ok(full_moon::parse(source)?)
    }

    pub fn luau_find_global_function_usage<'a>(&'a self, ast: &'a Ast, function_to_find: &str) -> HashMap<Range, Vec<&Suffix>> {
        let mut usage_map: HashMap<Range, Vec<&Suffix>> = HashMap::new();
        let block = ast.nodes();
        internal_find_from_visit(function_to_find, block, &mut usage_map);

        usage_map
    }
}