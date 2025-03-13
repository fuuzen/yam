mod ast;
mod syntactic;
mod semantic;
mod interpret;
mod error;

pub use syntactic::Analyzer as SyntacticAnalyzer;
pub use semantic::Analyzer as SemanticAnalyzer;
pub use interpret::Interpreter as Interpreter;