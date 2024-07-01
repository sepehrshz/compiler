use std::{collections::HashMap, i32, isize, sync::mpsc::channel, u32, usize};

use slab_tree::{NodeRef, Tree};

use crate::{
    syntax::{parser::Parser, NonTerminal, SymbolTree},
    token::{self, Token, TokenType},
};

pub struct Sem {
    ast: Tree<SymbolTree>,
    ids_table: HashMap<(String, u32), (TokenType, Vec<TokenType>)>,
}

impl Sem {
    pub fn new(code: &str) -> Result<Self, String> {
        let ast = Parser::new(code.to_string()).parse()?;
        let mut s = String::new();
        ast.write_formatted(&mut s).unwrap();
        print!("{}", s);
        Ok(Self {
            ast,
            ids_table: HashMap::new(),
        })
    }
    pub fn parser(&mut self) {
        let mut blo = 0;
        post_order_traversal(&mut self.ids_table, self.ast.root().unwrap(), &mut blo);
        println!("{:?}", self.ids_table);
        if !self
            .ids_table
            .iter()
            .find(|f| f.0 .0 == "main")
            .is_some_and(|f| f.1 .0 == TokenType::T_Int && f.1 .1.is_empty())
        {
            println!("there should be main fun with out params");
        }
    }
}

fn post_order_traversal(
    ids_table: &mut HashMap<(String, u32), (TokenType, Vec<TokenType>)>,
    node: NodeRef<SymbolTree>,
    block_num: &mut u32,
) {
    let symbole = node.data();

    if let SymbolTree::Token(t) = symbole {
        if t.token == TokenType::T_LC {
            *block_num = *block_num + 1;
        }
    }

    if let SymbolTree::NonTerminal(data) = symbole {
        if data == &NonTerminal::Declaration {
            let mut choil = node.children();
            // println!("{:?}", &choil.count());
            let n = choil.next().unwrap();
            let n = n.last_child().unwrap();
            let name = if let SymbolTree::Token(t) = n.first_child().unwrap().data() {
                t
            } else {
                unimplemented!();
            };
            let n = choil.next().unwrap();
            let n = n.last_child().unwrap();

            let types = if let SymbolTree::Token(t) = n.data() {
                t
            } else {
                unimplemented!();
            };
            if ids_table.contains_key(&(name.literal.clone(), block_num.clone())) {
                println!("two same var in a block {:?}", &name);
            } else {
                let mut prams = vec![];
                find_prams(&node, &mut prams);
                prams.reverse();

                let mut typer = None;
                find_the_op(&node, &mut typer);

                if typer.is_some()
                    && typer.unwrap() != types.token
                    && n.parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .data()
                        == &SymbolTree::NonTerminal(NonTerminal::Statement)
                {
                    println!("types dont match {:?}", types);
                }

                ids_table.insert(
                    (name.literal.clone(), block_num.clone()),
                    (types.token.clone(), prams),
                );
            }
        } else if data == &NonTerminal::VarDeclRest {
            let mut choil = node.children();
            choil.next();
            choil.next();
            let mut choil = choil.next().unwrap().children();
            let choil_node = choil.next().unwrap();
            if choil_node.data() == &SymbolTree::NonTerminal(NonTerminal::Initialization) {
                choil.next();
                let choil = choil.next();
                if let Some(data) = choil {
                    if data.first_child().is_none() {
                        println!(
                            "array number should be number and bigger and 0 {:?}",
                            data.parent().unwrap().children().nth(1).unwrap().data()
                        )
                    } else {
                        let number =
                            if let SymbolTree::Token(t) = data.first_child().unwrap().data() {
                                t
                            } else {
                                unreachable!()
                            };

                        if !number.literal.parse::<i32>().is_ok_and(|f| f > 0) {
                            println!(
                                "array number should be number and bigger and 0 {:?}",
                                number
                            )
                        }
                    }
                }
            } else {
                let mut prams: usize = 0;
                count_prams(&choil_node, &mut prams);

                if let SymbolTree::Token(token) = choil_node.last_child().unwrap().data() {
                    if ids_table
                        .iter()
                        .find(|f| f.0 .0 == token.literal && f.1 .1.len() == prams - 1)
                        .is_none()
                    {
                        println!("func call params doesnt match {:?}", token);
                    }
                }
            }
        } else if data == &NonTerminal::ReturnStatement {
            let node = node.first_child().unwrap();
            let typr = if node.data() == &SymbolTree::NonTerminal(NonTerminal::Expression) {
                if let SymbolTree::Token(token) = node.last_child().unwrap().data() {
                    ids_table
                        .iter()
                        .find(|f| f.0 .0 == token.literal)
                        .unwrap()
                        .1
                         .0
                        .clone()
                } else {
                    let mut g = None;
                    find_the_op(&node, &mut g);
                    g.unwrap().clone()
                }
            } else {
                unimplemented!();
            };
            fn find_typr(node: NodeRef<SymbolTree>) -> SymbolTree {
                if node.data() == &SymbolTree::NonTerminal(NonTerminal::Declaration) {
                    node.last_child()
                        .unwrap()
                        .first_child()
                        .unwrap()
                        .data()
                        .clone()
                } else {
                    find_typr(node.parent().unwrap())
                }
            }
            if let SymbolTree::Token(t) = find_typr(node) {
                if typr.clone() != t.token {
                    println!("return type doesnt match {:?}", t);
                }
            }
        }
    }

    if let SymbolTree::Token(token) = symbole {
        if token.token == TokenType::T_Id
            && ids_table.keys().find(|f| f.0 == token.literal).is_none()
            && node.parent().unwrap().parent().unwrap().data()
                == &SymbolTree::NonTerminal(NonTerminal::VarOrFunc)
        {
            println!("var or func not declaration {:?}", { token })
        }
    }

    for child in node.children() {
        post_order_traversal(ids_table, child, block_num);
    }
}

fn find_types(
    node: &NodeRef<SymbolTree>,
    typef: &mut Option<TokenType>,
    ids_table: &mut HashMap<(String, u32), (TokenType, Vec<TokenType>)>,
) {
    for child in node.children() {
        find_types(&child, typef, ids_table);
    }
    if let SymbolTree::Token(data) = &node.data() {
        let li = ids_table.iter().find(|f| f.0 .0 == data.literal);
        if li.is_none() {
            return;
        }
        if typef.is_none() {
            *typef = Some(li.unwrap().1.clone().0);
        } else if typef.clone().unwrap() != li.unwrap().1.clone().0 {
            println!("types don't match {:?}", data)
        }
    }
}

fn find_the_op(node: &NodeRef<SymbolTree>, typef: &mut Option<TokenType>) {
    for child in node.children() {
        find_the_op(&child, typef);
    }
    if let SymbolTree::Token(t) = node.data() {
        match t.token {
            TokenType::T_ROp_L
            | TokenType::T_ROp_G
            | TokenType::T_ROp_L
            | TokenType::T_ROp_G
            | TokenType::T_ROp_LE
            | TokenType::T_ROp_GE
            | TokenType::T_ROp_NE
            | TokenType::T_ROp_E
            | TokenType::T_LOp_AND
            | TokenType::T_LOp_OR
            | TokenType::T_LOp_NOT
            | TokenType::T_False
            | TokenType::T_True => {
                if typef.is_none() {
                    *typef = Some(TokenType::T_Bool);
                } else if typef.clone().unwrap() != TokenType::T_Bool {
                    println!("types don't match {:?}" ,t);
                }
            }
            TokenType::T_AOp_PL
            | TokenType::T_AOp_MN
            | TokenType::T_AOp_ML
            | TokenType::T_AOp_DV
            | TokenType::T_AOp_RM
            | TokenType::T_Decimal => {
                if typef.is_none() {
                    *typef = Some(TokenType::T_Int);
                } else if typef.clone().unwrap() != TokenType::T_Int {
                    println!("types don't match {:?}" , t);
                }
            }
            _ => {}
        }
    }
}

fn count_prams(node: &NodeRef<SymbolTree>, prams: &mut usize) {
    if let SymbolTree::Token(data) = &node.data() {
        if data.token == TokenType::T_Id {
            *prams = *prams + 1;
        }
    }
    for child in node.children() {
        count_prams(&child, prams);
    }
}

fn find_prams(node: &NodeRef<SymbolTree>, prams: &mut Vec<TokenType>) {
    if let SymbolTree::Token(data) = &node.data() {
        if (data.token == TokenType::T_Int
            || data.token == TokenType::T_Bool
            || data.token == TokenType::T_Char)
            && node.parent().unwrap().parent().unwrap().data()
                == &SymbolTree::NonTerminal(NonTerminal::Parameter)
        {
            prams.push(data.token.clone());
        }
    }
    for child in node.children() {
        find_prams(&child, prams);
    }
}
