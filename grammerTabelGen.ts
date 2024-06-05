import grammer from "./grammer.json" assert { type: "json" };

const NonTerminal = [
  "Program",
  "Declarations",
  "Declaration",
  "VarOrFunc",
  "VarOrFuncRest",
  "VarDeclRest",
  "FunctionRest",
  "Type",
  "MoreIdentifiers",
  "Initialization",
  "Functions",
  "Function",
  "Parameters",
  "ParameterList",
  "Parameter",
  "MoreParameters",
  "Block",
  "Statements",
  "Statement",
  "Assignment",
  "IfStatement",
  "ElseIfs",
  "ElseIf",
  "ElseBlock",
  "ForStatement",
  "ForInit",
  "ForCondition",
  "ForUpdate",
  "PrintStatement",
  "PrintArguments",
  "MorePrintArguments",
  "ReturnStatement",
  "BreakStatement",
  "ContinueStatement",
  "Expression",
  "LogicalOr",
  "LogicalOrPRE",
  "LogicalAnd",
  "LogicalAndPRE",
  "Equality",
  "EqualityPRE",
  "Relational",
  "RelationalPRE",
  "Additive",
  "AdditivePRE",
  "Multiplicative",
  "MultiplicativePRE",
  "Unary",
  "Primary",
  "Identifier",
  "IntegerLiteral",
  "BooleanLiteral",
  "CharacterLiteral",
  "StringLiteral",
];

grammer.forEach((element) => {
  Object.keys(element).forEach((val) => {
    if (
      element[val] !== "" &&
      val !== "FOLLOW" &&
      val !== "Nonterminal" &&
      val !== "FIRST"
    ) {
      console.log(
        `parsing_table.insert((NonTerminal::${element.Nonterminal}, TokenType::${val}), vec![${element[
          val
        ]
          .split(" ")
          .filter((va) => va && va !== "''")
          .map(
            (va) =>
              `Symbol::${NonTerminal.includes(va) ? "NonTerminal" : "Token"}(${NonTerminal.includes(va)  ? "NonTerminal" : "TokenType"}::${va})`,
          )
          .join(",")}]);`.replace('$' , 'End'),
      );
    }
  });
});
