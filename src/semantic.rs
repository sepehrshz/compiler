use std::collections::HashMap;

use slab_tree::{NodeId, NodeRef, Tree};

use crate::syntax::{parser::Parser, Symbol, SymbolTree};

pub struct Sem {
    ast: Tree<SymbolTree>,
    ids_table: HashMap<String, String>,
}

impl Sem {
    pub fn new(code: &str) -> Result<Self, String> {
        let ast = Parser::new(code.to_string()).parse()?;
        let mut s = String::new();
        ast.write_formatted(&mut s).unwrap();
        println!("{}", s);
        Ok(Self {
            ast,
            ids_table: HashMap::new(),
        })
    }
    pub fn parser(&mut self) {
        let root_id = self.ast.root_id().unwrap();
        self.post_order_traversal(root_id)
    }

    fn post_order_traversal(&mut self, id:NodeId) {
        // if node.data() == &Symbol::NonTerminal(crate::syntax::NonTerminal::Declaration) {
        //     let token = match node.children().next().unwrap().data() {
        //         Symbol::Token(_) => todo!(),
        //         Symbol::NonTerminal(_) => todo!(),
        //         Symbol::Def => todo!(),
        //     }
        //     self.ids_table.insert(
        //         ,
        //         node.children().next().unwrap().data(),
        //     );
        // }
        let node = self.ast.get(id).unwrap();
        println!("{:?}", node.data());
        for child in node.children() {
            self.post_order_traversal(child.node_id());
        }
    }
}
