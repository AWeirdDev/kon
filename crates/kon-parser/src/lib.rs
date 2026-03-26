//! Parser for Kon language.

#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt;

// AST Node Types
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Struct(StructDef),
    Enum(EnumDef),
    Function(FunctionDef),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroAttr {
    pub name: String,
    pub args: Vec<MacroArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub attrs: Vec<MacroAttr>,
    pub name: String,
    pub body: StructBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructBody {
    Named(Vec<StructField>),
    Tuple(Vec<Type>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    pub attrs: Vec<MacroAttr>,
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub data: VariantData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantData {
    Unit,
    Tuple(Vec<Type>),
    Named(Vec<StructField>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub attrs: Vec<MacroAttr>,
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeriveAttr {
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expr },
    Res { name: String, value: Expr },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<BlockItem>,
    pub expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Item(Item),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Field {
        object: Box<Expr>,
        field: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Try(Box<Expr>),
    If {
        cond: Box<IfCondition>,
        then: Block,
        els: Option<Box<Expr>>,
    },
    While {
        cond: Box<WhileCondition>,
        body: Block,
    },
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Closure {
        move_kw: bool,
        params: Vec<Param>,
        body: Box<ClosureBody>,
    },
    Block(Block),
    Macro {
        name: String,
        args: Vec<MacroArg>,
    },
    Path(String),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfCondition {
    Expr(Expr),
    Let { pattern: Pattern, value: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhileCondition {
    Expr(Expr),
    Let { pattern: Pattern, value: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureBody {
    Block(Block),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: MatchArmBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchArmBody {
    Block(Block),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Binding(String),
    Path(String),
    TupleStruct {
        path: String,
        fields: Vec<Pattern>,
    },
    Struct {
        path: String,
        fields: Vec<FieldPattern>,
    },
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldPattern {
    pub name: String,
    pub pattern: Pattern,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArg {
    Expr(Expr),
    DictPair { key: String, value: Expr },
    Spread(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Rem => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::Assign => write!(f, "="),
            BinOp::AddAssign => write!(f, "+="),
            BinOp::SubAssign => write!(f, "-="),
            BinOp::MulAssign => write!(f, "*="),
            BinOp::DivAssign => write!(f, "/="),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Not,
    Neg,
    Ref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Path(String),
    Reference(Box<Type>),
    Pointer(Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

// PEG Parser
peg::parser! {
    grammar kon() for str {
        // Entry point
        pub rule program() -> Vec<Item>
            = _ items:(item() ** _) _ { items }

        // Top-level items
        rule item() -> Item
            = s:struct_def() { Item::Struct(s) }
            / e:enum_def() { Item::Enum(e) }
            / f:function_def() { Item::Function(f) }

        // Struct definitions
        rule struct_def() -> StructDef
            = attrs:macro_attr()* _ "struct" _ name:ident() _ body:struct_body() {
                StructDef { attrs, name, body }
            }

        rule struct_body() -> StructBody
            = "{" _ fields:struct_field_list() _ "}" { StructBody::Named(fields) }
            / "(" _ fields:tuple_field_list() _ ")" _ ";" { StructBody::Tuple(fields) }
            / ";" { StructBody::Unit }

        rule struct_field_list() -> Vec<StructField>
            = fields:(struct_field() ** (_ "," _)) _ ","? { fields }

        rule struct_field() -> StructField
            = name:ident() _ ":" _ ty:type_expr() {
                StructField { name, ty }
            }

        rule tuple_field_list() -> Vec<Type>
            = types:(type_expr() ** (_ "," _)) _ ","? { types }

        // Enum definitions
        rule enum_def() -> EnumDef
            = attrs:macro_attr()* _ "enum" _ name:ident() _ "{" _ variants:enum_variant_list() _ "}" {
                EnumDef { attrs, name, variants }
            }

        rule enum_variant_list() -> Vec<EnumVariant>
            = variants:(enum_variant() ** (_ "," _)) _ ","? { variants }

        rule enum_variant() -> EnumVariant
            = name:ident() _ data:variant_data() {
                EnumVariant { name, data }
            }

        rule variant_data() -> VariantData
            = "(" _ fields:tuple_field_list() _ ")" { VariantData::Tuple(fields) }
            / "{" _ fields:struct_field_list() _ "}" { VariantData::Named(fields) }
            / { VariantData::Unit }

        // Function definitions
        rule function_def() -> FunctionDef
            = attrs:macro_attr()* _ "fn" _ name:ident() _ "(" _ params:param_list()? _ ")" _ ret:return_type()? _ body:block() {
                FunctionDef {
                    attrs,
                    name,
                    params: params.unwrap_or_default(),
                    ret,
                    body,
                }
            }

        rule param_list() -> Vec<Param>
            = params:(param() ** (_ "," _)) _ ","? { params }

        rule param() -> Param
            = name:ident() _ ":" _ ty:type_expr() {
                Param { name, ty }
            }

        rule return_type() -> Type
            = "->" _ ty:type_expr() { ty }

        // Statements
        rule statement() -> Statement
            = let_stmt()
            / res_stmt()
            / expr_stmt()

        rule let_stmt() -> Statement
            = "let" _ name:ident() _ "=" _ value:expr() _ ";" {
                Statement::Let { name, value }
            }

        rule res_stmt() -> Statement
            = "res" _ name:ident() _ "=" _ value:expr() _ ";" {
                Statement::Res { name, value }
            }

        rule expr_stmt() -> Statement
            = e:expr() _ ";" { Statement::Expr(e) }

        // Expressions (with precedence climbing)
        pub rule expr() -> Expr = _ e:assign_expr() _ { e }

        rule assign_expr() -> Expr
            = left:or_expr() _ op:assign_op() _ right:assign_expr() {
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / or_expr()

        rule assign_op() -> BinOp
            = "+=" { BinOp::AddAssign }
            / "-=" { BinOp::SubAssign }
            / "*=" { BinOp::MulAssign }
            / "/=" { BinOp::DivAssign }
            / "=" { BinOp::Assign }

        rule or_expr() -> Expr
            = left:and_expr() _ "||" _ right:or_expr() {
                Expr::Binary {
                    op: BinOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / and_expr()

        rule and_expr() -> Expr
            = left:equality_expr() _ "&&" _ right:and_expr() {
                Expr::Binary {
                    op: BinOp::And,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / equality_expr()

        rule equality_expr() -> Expr
            = left:comparison_expr() _ op:equality_op() _ right:comparison_expr() {
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / comparison_expr()

        rule equality_op() -> BinOp
            = "==" { BinOp::Eq }
            / "!=" { BinOp::Ne }

        rule comparison_expr() -> Expr
            = left:additive_expr() _ op:comparison_op() _ right:additive_expr() {
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / additive_expr()

        rule comparison_op() -> BinOp
            = "<=" { BinOp::Le }
            / ">=" { BinOp::Ge }
            / "<" { BinOp::Lt }
            / ">" { BinOp::Gt }

        rule additive_expr() -> Expr
            = left:multiplicative_expr() _ op:additive_op() _ right:additive_expr() {
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / multiplicative_expr()

        rule additive_op() -> BinOp
            = "+" { BinOp::Add }
            / "-" { BinOp::Sub }

        rule multiplicative_expr() -> Expr
            = left:unary_expr() _ op:multiplicative_op() _ right:multiplicative_expr() {
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            / unary_expr()

        rule multiplicative_op() -> BinOp
            = "*" { BinOp::Mul }
            / "/" { BinOp::Div }
            / "%" { BinOp::Rem }

        rule unary_expr() -> Expr
            = op:unary_op() _ e:unary_expr() {
                Expr::Unary {
                    op,
                    expr: Box::new(e),
                }
            }
            / postfix_expr()

        rule unary_op() -> UnOp
            = "!" { UnOp::Not }
            / "-" { UnOp::Neg }
            / "&" { UnOp::Ref }

        rule postfix_expr() -> Expr
            = base:primary_expr() suffixes:postfix_suffix()* {
                suffixes.into_iter().fold(base, |acc, suffix| {
                    match suffix {
                        PostfixSuffix::Call(args) => Expr::Call {
                            callee: Box::new(acc),
                            args,
                        },
                        PostfixSuffix::Field(field) => Expr::Field {
                            object: Box::new(acc),
                            field,
                        },
                        PostfixSuffix::Index(index) => Expr::Index {
                            object: Box::new(acc),
                            index: Box::new(index),
                        },
                        PostfixSuffix::Try => Expr::Try(Box::new(acc)),
                    }
                })
            }

        rule postfix_suffix() -> PostfixSuffix
            = "(" _ args:expr_list()? _ ")" {
                PostfixSuffix::Call(args.unwrap_or_default())
            }
            / "." _ field:ident() { PostfixSuffix::Field(field) }
            / "[" _ index:expr() _ "]" { PostfixSuffix::Index(index) }
            / "?" { PostfixSuffix::Try }

        rule expr_list() -> Vec<Expr>
            = exprs:(expr() ** (_ "," _)) _ ","? { exprs }

        rule primary_expr() -> Expr
            = if_expr()
            / while_expr()
            / match_expr()
            / closure_expr()
            / b:block() { Expr::Block(b) }
            / macro_expr()
            / l:literal() { Expr::Literal(l) }
            / p:path() { Expr::Path(p) }
            / "(" _ e:expr() _ ")" { e }

        // Control flow
        rule if_expr() -> Expr
            = "if" _ cond:if_condition() _ then:block() _ els:else_clause()? {
                Expr::If {
                    cond: Box::new(cond),
                    then,
                    els: els.map(Box::new),
                }
            }

        rule else_clause() -> Expr
            = "else" _ e:else_branch() { e }

        rule else_branch() -> Expr
            = if_expr()
            / b:block() { Expr::Block(b) }

        rule if_condition() -> IfCondition
            = "let" _ pattern:pattern() _ "=" _ value:expr() {
                IfCondition::Let { pattern, value }
            }
            / e:expr() { IfCondition::Expr(e) }

        rule while_expr() -> Expr
            = "while" _ cond:while_condition() _ body:block() {
                Expr::While {
                    cond: Box::new(cond),
                    body,
                }
            }

        rule while_condition() -> WhileCondition
            = "let" _ pattern:pattern() _ "=" _ value:expr() {
                WhileCondition::Let { pattern, value }
            }
            / e:expr() { WhileCondition::Expr(e) }

        rule match_expr() -> Expr
            = "match" _ value:expr() _ "{" _ arms:match_arm_list() _ "}" {
                Expr::Match {
                    value: Box::new(value),
                    arms,
                }
            }

        rule match_arm_list() -> Vec<MatchArm>
            = arms:match_arm()+ { arms }

        rule match_arm() -> MatchArm
            = pattern:pattern() _ "=>" _ body:match_arm_body() _ ","? {
                MatchArm { pattern, body }
            }

        rule match_arm_body() -> MatchArmBody
            = b:block() { MatchArmBody::Block(b) }
            / e:expr() { MatchArmBody::Expr(e) }

        // Patterns
        rule pattern() -> Pattern
            = "_" { Pattern::Wildcard }
            / path:path() _ "(" _ fields:pattern_list()? _ ")" {
                Pattern::TupleStruct {
                    path,
                    fields: fields.unwrap_or_default(),
                }
            }
            / path:path() _ "{" _ fields:named_pattern_list()? _ "}" {
                Pattern::Struct {
                    path,
                    fields: fields.unwrap_or_default(),
                }
            }
            / l:literal() { Pattern::Literal(l) }
            / path:path() {
                if path.contains("::") {
                    Pattern::Path(path)
                } else {
                    Pattern::Binding(path)
                }
            }

        rule pattern_list() -> Vec<Pattern>
            = patterns:(pattern() ** (_ "," _)) _ ","? { patterns }

        rule named_pattern_list() -> Vec<FieldPattern>
            = fields:(named_pattern() ** (_ "," _)) _ ","? { fields }

        rule named_pattern() -> FieldPattern
            = name:ident() _ ":" _ pattern:pattern() {
                FieldPattern { name: name.clone(), pattern }
            }
            / name:ident() {
                FieldPattern {
                    name: name.clone(),
                    pattern: Pattern::Binding(name),
                }
            }

        // Closures
        rule closure_expr() -> Expr
            = move_kw:("move" _)? "||" _ body:closure_body() {
                Expr::Closure {
                    move_kw: move_kw.is_some(),
                    params: vec![],
                    body: Box::new(body),
                }
            }
            / move_kw:("move" _)? "|" _ params:param_list()? _ "|" _ body:closure_body() {
                Expr::Closure {
                    move_kw: move_kw.is_some(),
                    params: params.unwrap_or_default(),
                    body: Box::new(body),
                }
            }

        rule closure_body() -> ClosureBody
            = b:block() { ClosureBody::Block(b) }
            / e:expr() { ClosureBody::Expr(e) }


        // Block
        rule block() -> Block
            = "{" _ items:block_stmt()* _ trail:trail_expr()? _ "}" {
                Block {
                    stmts: items,
                    expr: trail,
                }
            }

        rule block_stmt() -> BlockItem
            = s:statement() _ { BlockItem::Statement(s) }
            / i:item() _ { BlockItem::Item(i) }


        rule trail_expr() -> Box<Expr>
            = quiet! {!("}" / ";") e:assign_expr() { Box::new(e) } }
            / expected!("trail expression")

        // Macro expressions (@ syntax)
        rule macro_expr() -> Expr
            = "@(" _ items:expr_list()? _ ")" {
                Expr::Macro {
                    name: "tuple".to_string(),
                    args: items.unwrap_or_default().into_iter().map(MacroArg::Expr).collect(),
                }
            }
            / "@{" _ pairs:dict_pair_list()? _ "}" {
                Expr::Macro {
                    name: "dict".to_string(),
                    args: pairs.unwrap_or_default(),
                }
            }
            / "@[" _ items:list_item_list()? _ "]" {
                Expr::Macro {
                    name: "list".to_string(),
                    args: items.unwrap_or_default(),
                }
            }
            / "@" name:ident() _ "(" _ args:macro_arg_list()? _ ")" {
                Expr::Macro {
                    name,
                    args: args.unwrap_or_default(),
                }
            }
            / "@" name:ident() _ body:block() {
                Expr::Macro {
                    name,
                    args: vec![MacroArg::Expr(Expr::Block(body))],
                }
            }

        rule dict_pair_list() -> Vec<MacroArg>
            = pairs:(dict_pair() ** (_ "," _)) _ ","? { pairs }

        rule dict_pair() -> MacroArg
            = ".." e:expr() { MacroArg::Spread(e) }
            / key:string_literal() _ ":" _ value:expr() {
                MacroArg::DictPair { key, value }
            }
            / key:ident() _ ":" _ value:expr() {
                MacroArg::DictPair { key, value }
            }
            / key:string_literal() {
                MacroArg::DictPair {
                    key: key.clone(),
                    value: Expr::Literal(Literal::String(key))
                }
            }
            / key:ident() {
                MacroArg::DictPair {
                    key: key.clone(),
                    value: Expr::Path(key)
                }
            }

        rule list_item_list() -> Vec<MacroArg>
            = items:(list_item() ** (_ "," _)) _ ","? { items }

        rule list_item() -> MacroArg
            = ".." e:expr() { MacroArg::Spread(e) }
            / e:expr() { MacroArg::Expr(e) }

        // General macro attribute (not just derive)
        rule macro_attr() -> MacroAttr
            = "@" name:ident() _ "(" _ args:macro_arg_list()? _ ")" _ {
                MacroAttr {
                    name,
                    args: args.unwrap_or_default()
                }
            }
            / "@" name:ident() _ body:block() {
                MacroAttr {
                    name,
                    args: vec![MacroArg::Expr(Expr::Block(body))],
                }
            }

        rule macro_arg_list() -> Vec<MacroArg>
            = args:(macro_arg() ** (_ "," _)) _ ","? { args }

        rule macro_arg() -> MacroArg
            = e:expr() { MacroArg::Expr(e) }

        // Types
        rule type_expr() -> Type
            = "#" ty:type_expr() { Type::Pointer(Box::new(ty)) }
            / "&" ty:type_expr() { Type::Reference(Box::new(ty)) }
            / p:path() { Type::Path(p) }

        rule path() -> String
            = head:ident() tail:(path_segment())* {
                let mut p = head;
                for segment in tail {
                    p.push_str("::");
                    p.push_str(&segment);
                }
                p
            }

        rule path_segment() -> String
            = "::" seg:ident() { seg }

        // Literals
        rule literal() -> Literal
            = bool_literal()
            / float_literal()
            / int_literal()
            / s:string_literal() { Literal::String(s) }

        rule bool_literal() -> Literal
            = "true" !ident_rest() { Literal::Bool(true) }
            / "false" !ident_rest() { Literal::Bool(false) }

        rule int_literal() -> Literal
            = n:$(['0'..='9']+) !ident_rest() {
                Literal::Int(n.parse().unwrap())
            }

        rule float_literal() -> Literal
            = n:$(['0'..='9']+ "." ['0'..='9']+) !ident_rest() {
                Literal::Float(n.parse().unwrap())
            }

        rule string_literal() -> String
            = "\"" chars:string_char()* "\"" {
                chars.into_iter().collect()
            }

        rule string_char() -> char
            = "\\" c:$([_]) {
                match c {
                    "n" => '\n',
                    "t" => '\t',
                    "r" => '\r',
                    "\\" => '\\',
                    "\"" => '"',
                    _ => c.chars().next().unwrap(),
                }
            }
            / c:$([^'"' | '\\']) { c.chars().next().unwrap() }

        // Identifiers
        rule ident() -> String
            = !keyword() s:$(ident_start() ident_rest()*) { s.to_string() }

        rule ident_start() = ['a'..='z' | 'A'..='Z' | '_']

        rule ident_rest() = ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

        rule keyword()
            = ("let" / "res" / "fn" / "struct" / "enum" / "match" /
               "if" / "else" / "while" / "true" / "false" / "move") !ident_rest()
            / expected!("keyword")

        // Whitespace and comments
        rule _() = (whitespace() / comment())*

        rule whitespace() = quiet!{[' ' | '\t' | '\n' | '\r']+}

        rule comment()
            = "//" [^'\n']* "\n"?
            / "/*" (!"*/" [_])* "*/"
    }
}

pub use self::kon::{expr, program};

// Helper enum for postfix operations
#[derive(Debug)]
enum PostfixSuffix {
    Call(Vec<Expr>),
    Field(String),
    Index(Expr),
    Try,
}
