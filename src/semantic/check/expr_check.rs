use crate::{ast::{expr::{LOrExpr, PrimaryExpr}, func::FuncType, val::BType}, error::Error};

use super::Analyzer;


/// 類型檢查,檢查當前類型是否滿足(能夠賦值給)目標類型
pub fn type_check(ret_type: BType, expect_type: BType) -> Result<(), Error> {
  if ret_type != expect_type && 
    !(ret_type == BType::Bool && expect_type == BType::Int) &&
    !(ret_type == BType::Int && expect_type == BType::Bool) &&
    !(ret_type == BType::Bool && expect_type == BType::Note) &&
    !(ret_type == BType::Int && expect_type == BType::Note)
  {
    let re = Error::SemanticError(
      format!("expects {expect_type}, but found {ret_type}")
    );
    return Err(re);
  }
  Ok(())
}


impl Analyzer {
  /// 检查表达式是否合法，返回类型能否兼容上目標類型。
  /// int 和 bool 类型运算按照 C 语言标准相互兼容。
  /// 其他目標類型必須滿足表達式只有一個 unary_expr 且沒有任何運算
  /// int/bool 可以向 note 轉化, 剩餘類型必須嚴格匹配
  pub fn expr_check(&mut self, lor_expr: &LOrExpr, btype: BType) -> Result<(), Error> {

    let flag1 =  // 表示需要嚴格匹配
      btype != BType::Int && btype != BType::Bool && btype != BType::Note;
    
    let mut flag2 =  // 表示表達式滿足嚴格匹配(只有一個 unary_expr 且沒有任何運算)
      lor_expr.land_exps.len() == 1;

    for land_expr in &lor_expr.land_exps {
      flag2 &= land_expr.eq_exps.len() == 1;

      for eq_expr in &land_expr.eq_exps {
        flag2 &= eq_expr.rel_exps.len() == 1;

        for rel_expr in &eq_expr.rel_exps {
          flag2 &= rel_expr.add_exps.len() == 1;

          for add_expr in &rel_expr.add_exps {
            flag2 &= add_expr.mul_exps.len() == 1;

            for mul_expr in &add_expr.mul_exps {
              flag2 &= mul_expr.unary_exps.len() == 1;

              if flag1 && !flag2 {
                return Err(Error::SemanticError(format!(
                  "expression must have only one unary_expr and no operation"
                )));
              }

              for unary_expr in &mul_expr.unary_exps {
                let expect_type = btype.clone();
                
                match &unary_expr.primary_exp {
                  PrimaryExpr::LVal( lval ) => {
                    let res = self.lval_check(lval);
                    if res.is_err() {
                      return res;
                    }

                    let ret_type = lval.rval.borrow().clone().unwrap().get_btype();
                    let res = type_check(ret_type, expect_type);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::FuncCall( func_call ) => {
                    let res = self.func_call_check(func_call);
                    match res {
                      Err(e) => return Err(e),
                      Ok(func_type) => {
                        match func_type {
                          FuncType::Void =>  return Err(Error::SemanticError(
                            "function return void in expression".to_string()
                          )),
                          FuncType::BType( ret_type) => {
                            let res = type_check(ret_type, expect_type);
                            if res.is_err() {
                              return res;
                            }
                          }
                        }
                      },
                    }
                  },
                  PrimaryExpr::Expr( expr ) => {
                    let res = self.expr_check(expr, expect_type.clone());
                    if res.is_err() {
                      return res;
                    }

                    let ret_type = BType::Int; // 相當於 int/bool
                    let res = type_check(ret_type, expect_type);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::Number( _ ) => {}
                }
              }
            }
          }
        }
      }
    }
    Ok(())
  }

  /// 检查表达式是否合法，但不做类型检查
  pub fn expr_check_untyped(&mut self, lor_expr: &LOrExpr) -> Result<(), Error> {
    for land_expr in &lor_expr.land_exps {
      for eq_expr in &land_expr.eq_exps {
        for rel_expr in &eq_expr.rel_exps {
          for add_expr in &rel_expr.add_exps {
            for mul_expr in &add_expr.mul_exps {
              for unary_expr in &mul_expr.unary_exps {
                match &unary_expr.primary_exp {
                  PrimaryExpr::LVal( lval ) => {
                    let res = self.lval_check(lval);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::FuncCall( func_call ) => {
                    let res = self.func_call_check(func_call);
                    if res.is_err() {
                      return Err(res.unwrap_err());
                    }
                  },
                  PrimaryExpr::Expr( expr ) => {
                    let res = self.expr_check_untyped(expr);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::Number( _ ) => {}
                }
              }
            }
          }
        }
      }
    }
    Ok(())
  }
}