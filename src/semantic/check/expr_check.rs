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
  pub fn expr_check(&mut self, lor_expr: &LOrExpr, btype_: Option<BType>) -> Result<(), Error> {

    let flag1 =  // 表示需要嚴格匹配
      btype_.is_some_and(|t|  t != BType::Int && t != BType::Bool && t != BType::Note);
    
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
                let btype = btype_.unwrap();
                return Err(Error::SemanticError(format!(
                  "calculating {btype} is not supported yet"
                )));
              }

              for unary_expr in &mul_expr.unary_exps {
                let expect_type_ = btype_.clone();
                
                match &unary_expr.primary_exp {
                  PrimaryExpr::LVal( lval ) => {
                    let mut res = self.lval_check(lval);
                    if res.is_err() {
                      return res;
                    }

                    let ret_type = lval.rval.borrow().clone().unwrap().get_btype();
                    res = match flag2 && expect_type_.is_some() {
                      true => Ok(()),  // 表达式只有这一个 unary_expr 且没有任何运算,不需要检查类型
                      false => type_check(
                        ret_type,
                        expect_type_.unwrap()
                      ),
                    };
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
                          FuncType::Void => match expect_type_.is_some() {
                            true => {
                              let expect_type = expect_type_.unwrap();
                              return Err(Error::SemanticError(format!(
                                "expect {expect_type}, but found void"
                              )))
                            },
                            false => {}  // 非赋值的地方调用void函数
                          }
                          FuncType::BType( ret_type) => {
                            let res = match expect_type_.is_some() {
                              false => match 
                                ret_type != BType::Int && ret_type != BType::Bool && !flag2  // 不允许int/bool之外的类型运算
                              {
                                true => Err(Error::SemanticError(format!(
                                  "calculating {ret_type} is not supported yet"
                                ))),
                                false => Ok(()),
                              },
                              true => type_check(
                                ret_type,
                                expect_type_.unwrap()
                              ),
                            };
                            if res.is_err() {
                              return res;
                            }
                          }
                        }
                      },
                    }
                  },
                  PrimaryExpr::Expr( expr ) => {
                    let res = match expect_type_.is_some() {
                      false => self.expr_check(expr, None),
                      true => match flag2 {
                        true => self.expr_check(expr, expect_type_),  // 没有任何运算,相当于冗余括号
                        false => self.expr_check(expr, Some(BType::Int)),  // 有运算,必须为int/bool
                      },
                    };
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