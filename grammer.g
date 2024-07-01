Program -> Declarations
Declarations -> Declaration Declarations
Declarations -> ''
Declaration -> Type VarOrFunc
VarOrFunc -> Identifier VarOrFuncRest
VarOrFuncRest -> FunctionRest
VarOrFuncRest -> VarDeclRest
VarDeclRest -> Initialization MoreIdentifiers T_Semicolon
FunctionRest -> T_LP Parameters T_RP Block
Type -> T_Int
Type -> T_Bool
Type -> T_Char
MoreIdentifiers -> T_Comma Identifier Initialization MoreIdentifiers
MoreIdentifiers -> ''
Initialization -> T_Assign Expression
Initialization -> T_LB IntegerLiteral T_RB Initialization
Initialization -> ''
Functions -> Function Functions 
Functions -> ''
Function -> Type Identifier FunctionRest
Parameters -> ParameterList
Parameters -> ''
ParameterList -> Parameter MoreParameters
Parameter -> Type Identifier
MoreParameters -> T_Comma Parameter MoreParameters
MoreParameters -> ''
Block -> T_LC Statements T_RC
Statements -> Statement Statements
Statements -> ''
Statement -> Declaration
Statement -> T_Id iddd T_Semicolon
iddd -> FuncCall
iddd -> Assignment 
Statement -> IfStatement
Statement -> ForStatement
Statement -> PrintStatement T_Semicolon
Statement -> ReturnStatement T_Semicolon
Statement -> BreakStatement T_Semicolon
Statement -> ContinueStatement T_Semicolon
Assignment -> T_Assign Expression
IfStatement -> T_If T_LP Expression T_RP Block ElseIfs ElseBlock
ElseIfs -> ElseIf ElseIfs
ElseIfs -> ''
ElseIf -> T_Else T_If T_LP Expression T_RP Block
ElseBlock -> T_Else Block
ElseBlock -> ''
ForStatement -> T_For T_LP ForInit ForCondition T_Semicolon ForUpdate T_RP Block
ForInit -> Assignment
ForInit -> Declaration
ForInit -> ''
ForCondition -> Expression
ForCondition -> ''
ForUpdate -> Assignment
ForUpdate -> ''
PrintStatement -> T_Print T_LP PrintArguments T_RP
PrintArguments -> StringLiteral MorePrintArguments
PrintArguments -> Expression
MorePrintArguments -> T_Comma Expression MorePrintArguments
MorePrintArguments -> ''
ReturnStatement -> T_Return Expression
BreakStatement -> T_Break
ContinueStatement -> T_Continue
Expression -> LogicalOr
LogicalOr -> LogicalAnd LogicalOrPRE
LogicalOrPRE  -> T_LOp_OR LogicalAnd LogicalOrPRE
LogicalOrPRE  -> ''
LogicalAnd -> Equality LogicalAndPRE
LogicalAndPRE  -> T_LOp_AND Equality LogicalAndPRE
LogicalAndPRE  -> ''
Equality -> Relational EqualityPRE
EqualityPRE ->  T_ROp_E Relational EqualityPRE
EqualityPRE  -> T_ROp_NE Relational EqualityPRE
EqualityPRE  -> ''
Relational -> Additive RelationalPRE
RelationalPRE ->  T_ROp_L Additive RelationalPRE
RelationalPRE ->  T_ROp_LE Additive RelationalPRE
RelationalPRE ->  T_ROp_G Additive RelationalPRE
RelationalPRE ->  T_ROp_GE Additive RelationalPRE
RelationalPRE ->  ''
Additive -> Multiplicative AdditivePRE
AdditivePRE  -> T_AOp_PL Multiplicative AdditivePRE
AdditivePRE  -> T_AOp_MN Multiplicative AdditivePRE
AdditivePRE  -> ''
Multiplicative -> Unary MultiplicativePRE
MultiplicativePRE ->  T_AOp_ML Unary MultiplicativePRE
MultiplicativePRE ->  T_AOp_DV Unary MultiplicativePRE
MultiplicativePRE ->  T_AOp_RM Unary MultiplicativePRE
MultiplicativePRE ->  ''
Unary -> T_LOp_NOT Unary
Unary -> Primary
Primary -> Identifier
Primary -> IntegerLiteral
Primary -> BooleanLiteral
Primary -> CharacterLiteral
Primary -> StringLiteral
Primary -> T_LP Expression T_RP
Identifier -> T_Id
IntegerLiteral -> T_Decimal
IntegerLiteral -> T_Hexadecimal
BooleanLiteral -> T_True
BooleanLiteral -> T_False
CharacterLiteral -> T_Character
StringLiteral -> T_String
FuncCall -> T_LP ParametersCall T_RP T_Semicolon
ParametersCall -> ParameterListCa
ParametersCall -> ''
ParameterListCa -> ParameterCa MoreParametersCal
ParameterCa -> Identifier
MoreParametersCal -> T_Comma ParameterCa MoreParametersCal
MoreParametersCal -> ''
Expression -> iddd 
