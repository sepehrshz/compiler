use std::collections::HashMap;

pub mod parser;

use crate::token::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    Program,
    Declarations,
    Declaration,
    VarOrFunc,
    VarOrFuncRest,
    VarDeclRest,
    FunctionRest,
    Type,
    MoreIdentifiers,
    Initialization,
    Functions,
    Function,
    Parameters,
    ParameterList,
    Parameter,
    MoreParameters,
    Block,
    Statements,
    Statement,
    Assignment,
    IfStatement,
    ElseIfs,
    ElseIf,
    ElseBlock,
    ForStatement,
    ForInit,
    ForCondition,
    ForUpdate,
    PrintStatement,
    PrintArguments,
    MorePrintArguments,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    Expression,
    LogicalOr,
    LogicalOrPRE,
    LogicalAnd,
    LogicalAndPRE,
    Equality,
    EqualityPRE,
    Relational,
    RelationalPRE,
    Additive,
    AdditivePRE,
    Multiplicative,
    MultiplicativePRE,
    Unary,
    Primary,
    Identifier,
    IntegerLiteral,
    BooleanLiteral,
    CharacterLiteral,
    StringLiteral,
}

pub type ParsingTable = HashMap<(NonTerminal, TokenType), Vec<Symbol>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Token(TokenType),
    NonTerminal(NonTerminal),
    Action(String), // Used to denote actions for AST generation.
}

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        return_type: String,
        name: String,
        params: Vec<(String, String)>,
        body: Vec<ASTNode>,
    },
    VariableDeclaration {
        var_type: String,
        name: String,
        value: Option<String>,
    },
    PrintStatement {
        value: String,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    Loop {
        initialization: Option<Box<ASTNode>>,
        condition: Option<Box<ASTNode>>,
        increment: Option<Box<ASTNode>>,
        body: Vec<ASTNode>,
    },
    BinaryOperation {
        operator: TokenType,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    UnaryOperation {
        operator: TokenType,
        operand: Box<ASTNode>,
    },
    Literal(String),
    Identifier(String),
    Expression(Vec<String>), // Added for IfStatement condition parsing
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
}

// Implement the LL(1) parser.

pub fn add_rules() -> ParsingTable {
    // Define the parsing table based on the grammar.
    let mut parsing_table = ParsingTable::new();

    // Fill in the parsing table with your grammar productions.
    // For example:
    parsing_table.insert(
        (NonTerminal::Program, TokenType::T_Int),
        vec![Symbol::NonTerminal(NonTerminal::Declarations)],
    );
    parsing_table.insert(
        (NonTerminal::Program, TokenType::T_Bool),
        vec![Symbol::NonTerminal(NonTerminal::Declarations)],
    );
    parsing_table.insert(
        (NonTerminal::Program, TokenType::T_Char),
        vec![Symbol::NonTerminal(NonTerminal::Declarations)],
    );
    parsing_table.insert(
        (NonTerminal::Program, TokenType::End),
        vec![Symbol::NonTerminal(NonTerminal::Declarations)],
    );
    parsing_table.insert(
        (NonTerminal::Declarations, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Declaration),
            Symbol::NonTerminal(NonTerminal::Declarations),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Declarations, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Declaration),
            Symbol::NonTerminal(NonTerminal::Declarations),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Declarations, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Declaration),
            Symbol::NonTerminal(NonTerminal::Declarations),
        ],
    );
    parsing_table.insert((NonTerminal::Declarations, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Declaration, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::VarOrFunc),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Declaration, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::VarOrFunc),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Declaration, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::VarOrFunc),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFunc, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::NonTerminal(NonTerminal::VarOrFuncRest),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Semicolon),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_LP),
        vec![Symbol::NonTerminal(NonTerminal::FunctionRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Int),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Bool),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Char),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Comma),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Assign),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_LB),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_RC),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_If),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_For),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Print),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Return),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Break),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Continue),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarOrFuncRest, TokenType::End),
        vec![Symbol::NonTerminal(NonTerminal::VarDeclRest)],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Semicolon),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Comma),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Assign),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_LB),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_RC),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_If),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_For),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Print),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Return),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Break),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Continue),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::VarDeclRest, TokenType::End),
        vec![
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::FunctionRest, TokenType::T_LP),
        vec![
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::Parameters),
            Symbol::Token(TokenType::T_RP),
            Symbol::NonTerminal(NonTerminal::Block),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Type, TokenType::T_Int),
        vec![Symbol::Token(TokenType::T_Int)],
    );
    parsing_table.insert(
        (NonTerminal::Type, TokenType::T_Bool),
        vec![Symbol::Token(TokenType::T_Bool)],
    );
    parsing_table.insert(
        (NonTerminal::Type, TokenType::T_Char),
        vec![Symbol::Token(TokenType::T_Char)],
    );
    parsing_table.insert(
        (NonTerminal::MoreIdentifiers, TokenType::T_Semicolon),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MoreIdentifiers, TokenType::T_Comma),
        vec![
            Symbol::Token(TokenType::T_Comma),
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::NonTerminal(NonTerminal::Initialization),
            Symbol::NonTerminal(NonTerminal::MoreIdentifiers),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Initialization, TokenType::T_Semicolon),
        vec![],
    );
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Comma), vec![]);
    parsing_table.insert(
        (NonTerminal::Initialization, TokenType::T_Assign),
        vec![
            Symbol::Token(TokenType::T_Assign),
            Symbol::NonTerminal(NonTerminal::Expression),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Initialization, TokenType::T_LB),
        vec![
            Symbol::Token(TokenType::T_LB),
            Symbol::NonTerminal(NonTerminal::IntegerLiteral),
            Symbol::Token(TokenType::T_RB),
            Symbol::NonTerminal(NonTerminal::Initialization),
        ],
    );
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::Initialization, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Functions, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Function),
            Symbol::NonTerminal(NonTerminal::Functions),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Functions, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Function),
            Symbol::NonTerminal(NonTerminal::Functions),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Functions, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Function),
            Symbol::NonTerminal(NonTerminal::Functions),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Function, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::NonTerminal(NonTerminal::FunctionRest),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Function, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::NonTerminal(NonTerminal::FunctionRest),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Function, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::NonTerminal(NonTerminal::FunctionRest),
        ],
    );
    parsing_table.insert((NonTerminal::Parameters, TokenType::T_RP), vec![]);
    parsing_table.insert(
        (NonTerminal::Parameters, TokenType::T_Int),
        vec![Symbol::NonTerminal(NonTerminal::ParameterList)],
    );
    parsing_table.insert(
        (NonTerminal::Parameters, TokenType::T_Bool),
        vec![Symbol::NonTerminal(NonTerminal::ParameterList)],
    );
    parsing_table.insert(
        (NonTerminal::Parameters, TokenType::T_Char),
        vec![Symbol::NonTerminal(NonTerminal::ParameterList)],
    );
    parsing_table.insert(
        (NonTerminal::ParameterList, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Parameter),
            Symbol::NonTerminal(NonTerminal::MoreParameters),
        ],
    );
    parsing_table.insert(
        (NonTerminal::ParameterList, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Parameter),
            Symbol::NonTerminal(NonTerminal::MoreParameters),
        ],
    );
    parsing_table.insert(
        (NonTerminal::ParameterList, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Parameter),
            Symbol::NonTerminal(NonTerminal::MoreParameters),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Parameter, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Parameter, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Parameter, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Type),
            Symbol::NonTerminal(NonTerminal::Identifier),
        ],
    );
    parsing_table.insert((NonTerminal::MoreParameters, TokenType::T_RP), vec![]);
    parsing_table.insert(
        (NonTerminal::MoreParameters, TokenType::T_Comma),
        vec![
            Symbol::Token(TokenType::T_Comma),
            Symbol::NonTerminal(NonTerminal::Parameter),
            Symbol::NonTerminal(NonTerminal::MoreParameters),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Block, TokenType::T_LC),
        vec![
            Symbol::Token(TokenType::T_LC),
            Symbol::NonTerminal(NonTerminal::Statements),
            Symbol::Token(TokenType::T_RC),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Int),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Bool),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Char),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert((NonTerminal::Statements, TokenType::T_RC), vec![]);
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_If),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_For),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Print),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Return),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Break),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Continue),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statements, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Statement),
            Symbol::NonTerminal(NonTerminal::Statements),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Int),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Bool),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Char),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_If),
        vec![Symbol::NonTerminal(NonTerminal::IfStatement)],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_For),
        vec![Symbol::NonTerminal(NonTerminal::ForStatement)],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Print),
        vec![
            Symbol::NonTerminal(NonTerminal::PrintStatement),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Return),
        vec![
            Symbol::NonTerminal(NonTerminal::ReturnStatement),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Break),
        vec![
            Symbol::NonTerminal(NonTerminal::BreakStatement),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Continue),
        vec![
            Symbol::NonTerminal(NonTerminal::ContinueStatement),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Statement, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Assignment),
            Symbol::Token(TokenType::T_Semicolon),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Assignment, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Identifier),
            Symbol::Token(TokenType::T_Assign),
            Symbol::NonTerminal(NonTerminal::Expression),
        ],
    );
    parsing_table.insert(
        (NonTerminal::IfStatement, TokenType::T_If),
        vec![
            Symbol::Token(TokenType::T_If),
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::Expression),
            Symbol::Token(TokenType::T_RP),
            Symbol::NonTerminal(NonTerminal::Block),
            Symbol::NonTerminal(NonTerminal::ElseIfs),
            Symbol::NonTerminal(NonTerminal::ElseBlock),
        ],
    );
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Else), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::ElseIfs, TokenType::T_Id), vec![]);
    parsing_table.insert(
        (NonTerminal::ElseIf, TokenType::T_Else),
        vec![
            Symbol::Token(TokenType::T_Else),
            Symbol::Token(TokenType::T_If),
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::Expression),
            Symbol::Token(TokenType::T_RP),
            Symbol::NonTerminal(NonTerminal::Block),
        ],
    );
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_If), vec![]);
    parsing_table.insert(
        (NonTerminal::ElseBlock, TokenType::T_Else),
        vec![
            Symbol::Token(TokenType::T_Else),
            Symbol::NonTerminal(NonTerminal::Block),
        ],
    );
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::ElseBlock, TokenType::T_Id), vec![]);
    parsing_table.insert(
        (NonTerminal::ForStatement, TokenType::T_For),
        vec![
            Symbol::Token(TokenType::T_For),
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::ForInit),
            Symbol::Token(TokenType::T_Semicolon),
            Symbol::NonTerminal(NonTerminal::ForCondition),
            Symbol::Token(TokenType::T_Semicolon),
            Symbol::NonTerminal(NonTerminal::ForUpdate),
            Symbol::Token(TokenType::T_RP),
            Symbol::NonTerminal(NonTerminal::Block),
        ],
    );
    parsing_table.insert((NonTerminal::ForInit, TokenType::T_Semicolon), vec![]);
    parsing_table.insert(
        (NonTerminal::ForInit, TokenType::T_Int),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::ForInit, TokenType::T_Bool),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::ForInit, TokenType::T_Char),
        vec![Symbol::NonTerminal(NonTerminal::Declaration)],
    );
    parsing_table.insert(
        (NonTerminal::ForInit, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::Assignment)],
    );
    parsing_table.insert((NonTerminal::ForCondition, TokenType::T_Semicolon), vec![]);
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_LP),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_LOp_NOT),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_Decimal),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_True),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_False),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_Character),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert(
        (NonTerminal::ForCondition, TokenType::T_String),
        vec![Symbol::NonTerminal(NonTerminal::Expression)],
    );
    parsing_table.insert((NonTerminal::ForUpdate, TokenType::T_RP), vec![]);
    parsing_table.insert(
        (NonTerminal::ForUpdate, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::Assignment)],
    );
    parsing_table.insert(
        (NonTerminal::PrintStatement, TokenType::T_Print),
        vec![
            Symbol::Token(TokenType::T_Print),
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::PrintArguments),
            Symbol::Token(TokenType::T_RP),
        ],
    );
    parsing_table.insert(
        (NonTerminal::PrintArguments, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::StringLiteral),
            Symbol::NonTerminal(NonTerminal::MorePrintArguments),
        ],
    );
    parsing_table.insert((NonTerminal::MorePrintArguments, TokenType::T_RP), vec![]);
    parsing_table.insert(
        (NonTerminal::MorePrintArguments, TokenType::T_Comma),
        vec![
            Symbol::Token(TokenType::T_Comma),
            Symbol::NonTerminal(NonTerminal::Expression),
            Symbol::NonTerminal(NonTerminal::MorePrintArguments),
        ],
    );
    parsing_table.insert(
        (NonTerminal::ReturnStatement, TokenType::T_Return),
        vec![
            Symbol::Token(TokenType::T_Return),
            Symbol::NonTerminal(NonTerminal::Expression),
        ],
    );
    parsing_table.insert(
        (NonTerminal::BreakStatement, TokenType::T_Break),
        vec![Symbol::Token(TokenType::T_Break)],
    );
    parsing_table.insert(
        (NonTerminal::ContinueStatement, TokenType::T_Continue),
        vec![Symbol::Token(TokenType::T_Continue)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_LP),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_LOp_NOT),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_Decimal),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_True),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_False),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_Character),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::Expression, TokenType::T_String),
        vec![Symbol::NonTerminal(NonTerminal::LogicalOr)],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalOr, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Semicolon), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Continue), vec![]);
    parsing_table.insert(
        (NonTerminal::LogicalOrPRE, TokenType::T_LOp_OR),
        vec![
            Symbol::Token(TokenType::T_LOp_OR),
            Symbol::NonTerminal(NonTerminal::LogicalAnd),
            Symbol::NonTerminal(NonTerminal::LogicalOrPRE),
        ],
    );
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::LogicalOrPRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::LogicalAnd, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Semicolon), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_LOp_OR), vec![]);
    parsing_table.insert(
        (NonTerminal::LogicalAndPRE, TokenType::T_LOp_AND),
        vec![
            Symbol::Token(TokenType::T_LOp_AND),
            Symbol::NonTerminal(NonTerminal::Equality),
            Symbol::NonTerminal(NonTerminal::LogicalAndPRE),
        ],
    );
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::LogicalAndPRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Equality, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Semicolon), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_LOp_OR), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_LOp_AND), vec![]);
    parsing_table.insert(
        (NonTerminal::EqualityPRE, TokenType::T_ROp_E),
        vec![
            Symbol::Token(TokenType::T_ROp_E),
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::EqualityPRE, TokenType::T_ROp_NE),
        vec![
            Symbol::Token(TokenType::T_ROp_NE),
            Symbol::NonTerminal(NonTerminal::Relational),
            Symbol::NonTerminal(NonTerminal::EqualityPRE),
        ],
    );
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::EqualityPRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Relational, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Semicolon), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_LOp_OR), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_LOp_AND), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_ROp_E), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_ROp_NE), vec![]);
    parsing_table.insert(
        (NonTerminal::RelationalPRE, TokenType::T_ROp_L),
        vec![
            Symbol::Token(TokenType::T_ROp_L),
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::RelationalPRE, TokenType::T_ROp_LE),
        vec![
            Symbol::Token(TokenType::T_ROp_LE),
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::RelationalPRE, TokenType::T_ROp_G),
        vec![
            Symbol::Token(TokenType::T_ROp_G),
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::RelationalPRE, TokenType::T_ROp_GE),
        vec![
            Symbol::Token(TokenType::T_ROp_GE),
            Symbol::NonTerminal(NonTerminal::Additive),
            Symbol::NonTerminal(NonTerminal::RelationalPRE),
        ],
    );
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::RelationalPRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Additive, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Semicolon), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Print), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Return), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Break), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Continue), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_LOp_OR), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_LOp_AND), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_E), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_NE), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_L), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_LE), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_G), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_ROp_GE), vec![]);
    parsing_table.insert(
        (NonTerminal::AdditivePRE, TokenType::T_AOp_PL),
        vec![
            Symbol::Token(TokenType::T_AOp_PL),
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::AdditivePRE, TokenType::T_AOp_MN),
        vec![
            Symbol::Token(TokenType::T_AOp_MN),
            Symbol::NonTerminal(NonTerminal::Multiplicative),
            Symbol::NonTerminal(NonTerminal::AdditivePRE),
        ],
    );
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::AdditivePRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_LP),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_LOp_NOT),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_Id),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_Decimal),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_True),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_False),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_Character),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Multiplicative, TokenType::T_String),
        vec![
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_Semicolon),
        vec![],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_RP), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Int), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Bool), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Char), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Comma), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_RC), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_If), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_For), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Print), vec![]);
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_Return),
        vec![],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Break), vec![]);
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_Continue),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_LOp_OR),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_LOp_AND),
        vec![],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_ROp_E), vec![]);
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_ROp_NE),
        vec![],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_ROp_L), vec![]);
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_ROp_LE),
        vec![],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_ROp_G), vec![]);
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_ROp_GE),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_AOp_PL),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_AOp_MN),
        vec![],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_AOp_ML),
        vec![
            Symbol::Token(TokenType::T_AOp_ML),
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_AOp_DV),
        vec![
            Symbol::Token(TokenType::T_AOp_DV),
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert(
        (NonTerminal::MultiplicativePRE, TokenType::T_AOp_RM),
        vec![
            Symbol::Token(TokenType::T_AOp_RM),
            Symbol::NonTerminal(NonTerminal::Unary),
            Symbol::NonTerminal(NonTerminal::MultiplicativePRE),
        ],
    );
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::T_Id), vec![]);
    parsing_table.insert((NonTerminal::MultiplicativePRE, TokenType::End), vec![]);
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_LP),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_LOp_NOT),
        vec![
            Symbol::Token(TokenType::T_LOp_NOT),
            Symbol::NonTerminal(NonTerminal::Unary),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_Decimal),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_True),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_False),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_Character),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Unary, TokenType::T_String),
        vec![Symbol::NonTerminal(NonTerminal::Primary)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_LP),
        vec![
            Symbol::Token(TokenType::T_LP),
            Symbol::NonTerminal(NonTerminal::Expression),
            Symbol::Token(TokenType::T_RP),
        ],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_Id),
        vec![Symbol::NonTerminal(NonTerminal::Identifier)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_Decimal),
        vec![Symbol::NonTerminal(NonTerminal::IntegerLiteral)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_True),
        vec![Symbol::NonTerminal(NonTerminal::BooleanLiteral)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_False),
        vec![Symbol::NonTerminal(NonTerminal::BooleanLiteral)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_Character),
        vec![Symbol::NonTerminal(NonTerminal::CharacterLiteral)],
    );
    parsing_table.insert(
        (NonTerminal::Primary, TokenType::T_String),
        vec![Symbol::NonTerminal(NonTerminal::StringLiteral)],
    );
    parsing_table.insert(
        (NonTerminal::Identifier, TokenType::T_Id),
        vec![Symbol::Token(TokenType::T_Id)],
    );
    parsing_table.insert(
        (NonTerminal::IntegerLiteral, TokenType::T_Decimal),
        vec![Symbol::Token(TokenType::T_Decimal)],
    );
    parsing_table.insert(
        (NonTerminal::BooleanLiteral, TokenType::T_True),
        vec![Symbol::Token(TokenType::T_True)],
    );
    parsing_table.insert(
        (NonTerminal::BooleanLiteral, TokenType::T_False),
        vec![Symbol::Token(TokenType::T_False)],
    );
    parsing_table.insert(
        (NonTerminal::CharacterLiteral, TokenType::T_Character),
        vec![Symbol::Token(TokenType::T_Character)],
    );
    parsing_table.insert(
        (NonTerminal::StringLiteral, TokenType::T_String),
        vec![Symbol::Token(TokenType::T_String)],
    );

    parsing_table
    // ... and so on for each grammar rule and corresponding input token.

    // Create a list of tokens to parse.
    // let input_tokens = vec![
    //     Token::Id(String::from("id")),
    //     Token::Plus,
    //     Token::Id(String::from("id")),
    //     Token::Eof,
    // ];

    // Initialize the parser with the parsing table and the input tokens.
    // let mut parser = Parser::new(parsing_table, input_tokens);

    // Parse the input tokens.
    // match parser.parse() {
    //     Ok(_) => println!("Parsing successful!"),
    //     Err(e) => println!("Parsing error: {}", e),
    // }
}
