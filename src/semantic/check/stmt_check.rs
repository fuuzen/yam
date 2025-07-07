use crate::ast::func::FuncType;
use crate::ast::score::{ScoreStmt, SetChannelInstrument, SetChannelTrack, SetTimeSignature};
use crate::ast::stmt::Stmt;
use crate::ast::track::TrackRVal;
use crate::ast::val::BType;
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 以 Stmt 为单位进行语义检查
  pub fn stmt_check(&mut self, stmt: &Stmt) -> Result<(), Error> {
    match stmt {
      Stmt::Break => self.break_check(),
      Stmt::Continue => self.continue_check(),
      Stmt::ConstDecl( const_decl ) => self.const_decl_check(const_decl),
      Stmt::VarDecl( var_decl ) => self.var_decl_check(var_decl),
      Stmt::Asgn( asgn ) => self.asgn_check(&asgn),
      Stmt::Return( expr_ ) => self.return_check(&expr_),
      Stmt::Block( block ) => self.block_check(block.clone()),
      Stmt::While( while_ ) => self.while_check(while_),
      Stmt::FuncDef( func_def ) => self.func_def_check(func_def.clone()),
      Stmt::IfElse( ifelse ) => self.ifelse_check(ifelse),
      Stmt::Expr( expr_ ) => match expr_.is_some() {
        true => self.expr_check(expr_.as_ref().unwrap(), None),
        false => Ok(()),
      },
    }
  }

  /// 以 Channel Stmt 为单位进行语义检查
  pub fn channel_stmt_check(&mut self, stmt: &ScoreStmt) -> Result<(), Error> {
    match stmt {
      ScoreStmt::SetChannelInstrument(SetChannelInstrument{channel, instrument}) => {
        match self.expr_check(channel, Some(BType::Int)) {
          Ok(()) => self.expr_check(instrument, Some(BType::Int)),
          Err(e) => Err(e)
        }
      },
      ScoreStmt::SetChannelTrack( SetChannelTrack{channel, track} ) => {
        match self.expr_check(channel, Some(BType::Int)) {
          Ok(()) => match track {
            TrackRVal::Track( track ) => self.track_check(track),
            TrackRVal::LVal( lval ) => {
              self.lval_check(lval)?;

              let ret_type = lval.rval.borrow().as_ref().unwrap().get_btype();
              match ret_type != BType::Track {
                true => Err(Error::SemanticError(format!("expect track, but found {ret_type}"))),
                false => Ok(())
              }
            },
            TrackRVal::FuncCall( func_call ) => {
              let ret_type = self.func_call_check(func_call)?;

              match ret_type {
                FuncType::Void => Err(Error::SemanticError(format!("expect track, but found void"))),
                FuncType::BType( ret_type ) => {
                  match ret_type != BType::Track {
                    true => Err(Error::SemanticError(format!("expect track, but found {ret_type}"))),
                    false => Ok(())
                  }
                }
              }
            }
          },
          Err(e) => Err(e)
        }
      },
      ScoreStmt::SetTimeSignature( SetTimeSignature{top_num, bottom_num} ) => {
        self.expr_check(top_num, Some(BType::Int))?;
        self.expr_check(bottom_num, Some(BType::Int))
      },
      ScoreStmt::SetTempo( expr ) => self.expr_check(expr, Some(BType::Int))
    }
  }
}