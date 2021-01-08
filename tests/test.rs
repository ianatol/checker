#[cfg(test)]
mod tests {
    use hw5::type_::Type;
    use hw5::build::*;
    use hw5::typechecker::*;
    use std::rc::Rc;
    use hw5::expr::Expression;

    #[test]
    fn pstwo_one_a() {
        assert_eq!(
            type_of(
                is_zero_e(num_e(0)),
                &mut Vec::new()
            ),
            Ok(bool_t())
        )
    }

    #[test]
    fn pstwo_one_b() {
        assert_eq!(
            type_of(
                add_e(
                    sub_e(
                        add_e(
                            num_e(0),
                            num_e(1)
                        ),
                        num_e(1)
                    ),
                    num_e(1)
                ),
                &mut Vec::new()
            ),
            Ok(int_t())
        )
    }

    #[test]
    fn pstwo_one_c() {
        assert_eq!(
            type_of(
                if_e(
                    true_e(),
                    add_e(
                        num_e(0),
                        num_e(1)
                    ),
                    true_e()
                ),
                &mut Vec::new()
            ),
            Err("If statement expected then and else types to match.".to_string())
        )
    }

    #[test]
    fn pstwo_one_d() {
        let mut gamma:Vec<(String, Type)> = vec![("x".to_string(),bool_t())];
        assert_eq!(
            type_of(
                if_e(
                    var_e("x".to_string()),
                    add_e(
                        num_e(0),
                        num_e(1)
                    ),
                    sub_e(
                        num_e(0),
                        num_e(1)
                    )
                ),
                &mut gamma
            ),
            Ok(int_t())
        )
    }

    #[test]
    fn pstwo_three_a() {
        assert_eq!(
            type_of(
                app_e(
                    abs_e(
                        var_e("x".to_string()),
                        int_t(),
                        sub_e(
                            var_e("x".to_string()),
                            num_e(1)
                        )
                    ),
                    num_e(0)
                ),
                &mut Vec::new()
            ),
            Ok(int_t())
        )
    }

    #[test]
    fn pstwo_three_b() {
        assert_eq!(
            type_of(
                app_e(
                    abs_e(
                        var_e("x".to_string()),
                        int_t(),
                        sub_e(
                            var_e("x".to_string()),
                            num_e(1)
                        )
                    ),
                    true_e()
                ),
                &mut Vec::new()
            ),
            Err(("t2 did not match input of t1").to_string())
        )
    }
    #[test]
    fn pstwo_three_c() {
      let mut gamma:Vec<(String, Type)> = vec![("times".to_string(), 
         fun_t(
            fun_t(
               int_t(),
               int_t()
            ),
            int_t()
         ))];
        assert_eq!(
            type_of(
                app_e(
                    abs_e(
                        var_e("x".to_string()),
                        int_t(),
                        app_e(
                            app_e(
                            var_e("times".to_string()),
                            var_e("x".to_string())
                            ),
                            num_e(0)
                        )
                    ),
                    add_e(
                        num_e(0),
                        num_e(1)
                    )
                ),
                &mut gamma
            ),
            Err("t2 did not match input of t1".to_string())
        )
    }

    #[test]
    fn pstwo_three_d() {
        let mut gamma:Vec<(String, Type)> = vec![("x".to_string(), int_t())];
        assert_eq!(
            type_of(
                app_e(
                abs_e(
                    var_e("x".to_string()),
                    bool_t(),
                    if_e(
                        var_e("x".to_string()),
                        app_e(
                            abs_e(
                            var_e("x".to_string()),
                            int_t(),
                            add_e(
                                var_e("x".to_string()),
                                num_e(1)
                            )
                            ),
                            num_e(0)
                        ),
                        app_e(
                            abs_e(
                                var_e("x".to_string()),
                                fun_t(
                                    int_t(),
                                    bool_t()
                                ),
                                app_e(
                                    var_e("x".to_string()),
                                    num_e(0)
                                )
                            ),
                            abs_e(
                                var_e("y".to_string()),
                                int_t(),
                                var_e("x".to_string())
                            )
                        )
                    )
                ),
                true_e()
                ),
                &mut gamma
            ),
            Err("t2 did not match input of t1".to_string()))
    }

   #[test]
   fn abs_var() {
       let mut gamma:Vec<(String, Type)> = vec![("x".to_string(),bool_t())];
       assert_eq!(type_of(
                   abs_e(
                       var_e("x".to_string()),
                       int_t(),
                       var_e("x".to_string())
                   ), &mut gamma),
           Ok(fun_t(int_t(),int_t())));
   }


  #[test]
   fn shadow() {
       let mut gamma:Vec<(String, Type)> = vec![("x".to_string(),bool_t())];
       assert_eq!(type_of(
                    app_e(
                       abs_e(
                          var_e("x".to_string()),
                          bool_t(),
                          var_e("x".to_string())
                      ),
                      num_e(7)),
                    &mut gamma),
           Err(("t2 did not match input of t1").to_string()));
   }

   #[test]
   fn record_basic() {
       let test_rec:Vec<(String, Rc<Expression>)> = vec![("x".to_string(), true_e()), ("y".to_string(), num_e(7))];
       let test_rec_types:Vec<(String, Type)> = vec![("x".to_string(), bool_t()), ("y".to_string(), int_t())];
       assert_eq!(
           type_of(val_record_e(test_rec.clone()), &mut Vec::new()),
           Ok(record_t(test_rec_types)))
   }


    #[test]
    fn proj_basic() {
        let test_rec:Vec<(String, Rc<Expression>)> = vec![("x".to_string(), true_e()), ("y".to_string(), num_e(7))];
        assert_eq!(
            type_of(proj_e(val_record_e(test_rec.clone()), "y".to_string()), &mut Vec::new()),
            Ok(int_t()))
   }

   #[test]
   fn case_basic() {
       let test_var:Vec<(String, Type)> = vec![("x".to_string(), bool_t()), ("y".to_string(), int_t())];
       let test_case:Vec<(String, Rc<Expression>)> = vec![("a".to_string(), var_e("a".to_string())), ("b".to_string(), true_e())];
       assert_eq!(
            type_of(case_e(test_var.clone(), test_case.clone()), &mut Vec::new()),
            Ok(bool_t()))
   }

   #[test]
   fn psthree_one_a() {
        let test_rec:Vec<(String, Rc<Expression>)> = vec![("x".to_string(), num_e(2)), ("y".to_string(), num_e(3)), ("color".to_string(), var_e("c".to_string()))];
        let test_rec_types:Vec<(String, Type)> = vec![("x".to_string(), int_t()), ("y".to_string(), int_t()), ("color".to_string(), unit_t())];
        let mut gamma:Vec<(String, Type)> = vec![("c".to_string(),unit_t())];
        assert_eq!(
            type_of(val_record_e(test_rec),&mut gamma),
            Ok(record_t(test_rec_types)))
   }

   #[test]
   fn psthree_one_b() {
        let test_rec:Vec<(String, Rc<Expression>)> = vec![("x".to_string(), num_e(2)), ("y".to_string(), num_e(3)), ("color".to_string(), var_e("c".to_string()))];
        let mut gamma:Vec<(String, Type)> = vec![("c".to_string(),unit_t())];
        assert_eq!(
            type_of(proj_e(val_record_e(test_rec), "color".to_string()), &mut gamma),
                    Ok(unit_t()))
   }

   #[test]
   fn psthree_one_c() {
        let point_rec = record_t(vec![("x".to_string(), int_t()),("y".to_string(), int_t())]);
        let move_fun = fun_t(point_rec.clone(),point_rec.clone());
        let test_rec:Vec<(String, Type)> = vec![
            ("move".to_string(), move_fun),
            ("point".to_string(), point_rec.clone())];
        let final_fun = fun_t(record_t(test_rec.clone()), int_t());
        assert_eq!(
            type_of(
                abs_e(
                    var_e("obj".to_string()),
                    record_t(test_rec.clone()),
                    proj_e(
                        proj_e(
                            var_e("obj".to_string()),"point".to_string()),
                        "x".to_string())
                ),
            &mut Vec::new()),
            Ok(final_fun))
   }

   #[test]
   fn eq_basic() {
        let rec_a = record_t(vec![("x".to_string(), bool_t()),("y".to_string(), int_t())]);
        let rec_b = record_t(vec![("x".to_string(), bool_t()),("y".to_string(), int_t())]);
        assert_eq!(rec_a == rec_b, true)
   }

   #[test]
   fn psetfour_one_a() {
       let rec_a = record_t(vec![("x".to_string(), int_t())]);
       let rec_b = val_record_e(vec![("x".to_string(), num_e(2)), ("y".to_string(), num_e(3))]);
       assert_eq!(type_of(
                  app_e(
                      abs_e(
                          var_e("r".to_string()),
                          rec_a.clone(),
                          var_e("r".to_string())
                      ),
                      rec_b
                  ),
                  &mut Vec::new()
                ),
                Ok(rec_a))
   }

   #[test]
   fn psetfour_one_b() {
        let rec_a = record_t(vec![("x".to_string(), int_t()), ("y".to_string(), int_t()), ("z".to_string(), int_t())]);
        let rec_b = record_t(vec![("x".to_string(), int_t()), ("y".to_string(), int_t())]);
        let mut gamma:Vec<(String, Type)> = vec![("a".to_string(),rec_b)];
        assert_eq!(type_of(
                    app_e(
                        abs_e(
                            var_e("r".to_string()),
                            rec_a.clone(),
                            var_e("r".to_string())
                        ),
                        var_e("a".to_string())
                    ),
                    &mut gamma
                    ),
                    Err(format!("t2 did not match input of t1")))
   }

   #[test]
   fn psetfour_one_c() {
        let f_t = fun_t( 
                      fun_t(
                          record_t(vec![("y".to_string(), int_t()), ("x".to_string(), int_t())]),
                          fun_t(
                              record_t(vec![("a".to_string(), int_t()), ("b".to_string(), int_t())]),
                              record_t(vec![("m".to_string(), int_t())])
                          )
                      ),int_t()
        );
        let g_t = fun_t(
                      record_t(vec![("y".to_string(), int_t())]),
                      fun_t(
                            record_t(vec![("a".to_string(), int_t())]),
                            record_t(vec![("n".to_string(), int_t()), ("m".to_string(), int_t())]),
                      )
        );
        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("g".to_string(), g_t)];
        assert_eq!(type_of(
                    app_e(
                          var_e("f".to_string()),
                          var_e("g".to_string())
                    ),
                    &mut gamma
                    ),
                    Ok(int_t()))
   }

   #[test]
   fn psetfour_two_a() {
        let f_t = fun_t(
            record_t(vec![("x".to_string(), int_t()), ("y".to_string(), int_t())]),
            int_t());
        let r_t = record_t(vec![("y".to_string(), int_t()), ("x".to_string(), int_t())]);
        
        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("r".to_string(), r_t)];
        assert_eq!(type_of(
            app_e(
                var_e("f".to_string()),
                var_e("r".to_string())
            ),
            &mut gamma
            ),
            Ok(int_t()))
   }

   #[test]
   fn psetfour_two_b() {
        let f_t = fun_t(
            record_t(vec![("x".to_string(), int_t())]),
            int_t());
        let r_t = record_t(vec![("y".to_string(), int_t()), ("x".to_string(), int_t())]);

        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("r".to_string(), r_t)];
        assert_eq!(type_of(
            app_e(
                var_e("f".to_string()),
                var_e("r".to_string())
            ),
            &mut gamma
            ),
            Ok(int_t()))
   }

   #[test]
   fn psetfour_two_c() {
        let f_t = fun_t(
            record_t(vec![("x".to_string(), record_t(vec![("n".to_string(), int_t())]))]),
            int_t());
        let r_t = record_t(vec![
            ("x".to_string(),record_t(vec![("n".to_string(), int_t()), ("m".to_string(), int_t())])),
            ("y".to_string(), int_t())]);

        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("r".to_string(), r_t)];
        assert_eq!(type_of(
            app_e(
                var_e("f".to_string()),
                var_e("r".to_string())
            ),
            &mut gamma
            ),
            Ok(int_t()))
   }

   #[test]
   fn psetfour_two_d() {
        let f_t = fun_t(
            record_t(vec![
                ("x".to_string(), fun_t(
                    record_t(vec![("n".to_string(), int_t())]),
                    record_t(vec![("x".to_string(), int_t())]),
                ))]),
            int_t());
        let r_t = record_t(vec![
            ("x".to_string(), fun_t(
                record_t(vec![("n".to_string(), int_t())]),
                record_t(vec![("x".to_string(), int_t())]),
            ))]);

        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("r".to_string(), r_t)];
        assert_eq!(type_of(
            app_e(
                var_e("f".to_string()),
                var_e("r".to_string())
            ),
            &mut gamma
            ),
            Ok(int_t()))
   }

   #[test]
   fn psetfour_two_e() {
        let f_t = fun_t(
            record_t(vec![
                ("x".to_string(), fun_t(
                    record_t(vec![("n".to_string(), int_t())]),
                    record_t(vec![("x".to_string(), int_t())]),
                ))]),
            int_t());
        let r_t = record_t(vec![
            ("x".to_string(), fun_t(
                record_t(vec![("n".to_string(), int_t())]),
                record_t(vec![("x".to_string(), int_t()), ("y".to_string(), int_t())]),
            ))]);

        let mut gamma:Vec<(String, Type)> = vec![("f".to_string(), f_t), ("r".to_string(), r_t)];
        assert_eq!(type_of(
            app_e(
                var_e("f".to_string()),
                var_e("r".to_string())
            ),
            &mut gamma
            ),
            Ok(int_t()))
   }

   #[test]
   fn psetfive_four_a(){
       assert_eq!(type_of(tabs_e("X".to_string(),
            tabs_e("Y".to_string(),
                abs_e(
                    var_e("f".to_string()),
                    fun_t(
                        fun_t(
                            var_t("X".to_string()),
                            var_t("Y".to_string())
                        ),
                        var_t("X".to_string())
                    ),
                    abs_e(
                        var_e("g".to_string()),
                        fun_t(
                            var_t("X".to_string()),
                            var_t("Y".to_string())
                        ),
                        app_e(
                            var_e("g".to_string()),
                            app_e(
                                var_e("f".to_string()),
                                var_e("g".to_string())
                            )
                        )
                    )
                )
            )
       ),&mut Vec::new(),
       ), 
       Ok(univ_t("X".to_string(),
       univ_t("Y".to_string(),
            fun_t(
                fun_t(
                    fun_t(
                        var_t("X".to_string()),
                        var_t("Y".to_string())
                    ),
                    var_t("X".to_string())
                ),
                fun_t(
                    fun_t(
                        var_t("X".to_string()),
                        var_t("Y".to_string())
                    ),
                    var_t("Y".to_string())
                )
            )
        )
    )))}

   #[test]
   fn psetfive_four_b() {
       let mut gamma:Vec<(String, Type)> = vec![("a".to_string(), int_t()), ("b".to_string(), bool_t())];
       assert_eq!(type_of(
           abs_e(
               var_e("f".to_string()),
               univ_t(
                   "X".to_string(),
                   fun_t(
                       var_t("X".to_string()),
                       int_t()
                   )
               ),
               add_e(
                   app_e(
                       tapp_e(
                           var_e("f".to_string()),
                           int_t()
                       ),
                       var_e("a".to_string())
                   ),
                   app_e(
                    tapp_e(
                        var_e("f".to_string()),
                        bool_t()
                    ),
                    var_e("b".to_string())
                    )
               )
           ), &mut gamma),
           Ok(fun_t(
               univ_t(
                   "X".to_string(),
                   fun_t(
                       var_t("X".to_string()),
                       int_t()
                   )
               ),
               int_t()
           ))
       )
   }

   #[test]
   fn psetfive_four_c(){
       let mut gamma:Vec<(String, Type)> = vec![("a".to_string(), int_t()), ("b".to_string(), bool_t())];
       assert_eq!(type_of(
           abs_e(
               var_e("X".to_string()),
               var_t("X".to_string()),
               abs_e(
                   var_e("f".to_string()),
                   fun_t(
                       var_t("X".to_string()),
                       int_t()
                   ),
                   add_e(
                       app_e(
                           var_e("f".to_string()),
                           var_e("a".to_string())
                       ),
                       app_e(
                        var_e("f".to_string()),
                        var_e("b".to_string())
                    )
                   )
               )
           ), &mut gamma),
           Err(format!("t2 did not match input of t1"))
       )
   }

   #[test]
   fn exis_test() {
       let u = Rc::new(int_t());
       let t = val_record_e(vec![
            ("new".to_string(), num_e(0)),
            ("inc".to_string(), abs_e(var_e("v".to_string()), int_t(), add_e(var_e("v".to_string()), num_e(1)))),
            ("get".to_string(), abs_e(var_e("v".to_string()), int_t(), var_e("v".to_string())))
        ]);
       let exis_type = Rc::new(exis_t("Counter".to_string(), record_t(vec![
            ("new".to_string(), var_t("Counter".to_string())),
            ("inc".to_string(), fun_t(var_t("Counter".to_string()), var_t("Counter".to_string()))),
            ("get".to_string(), fun_t(var_t("Counter".to_string()), int_t()))
       ])));
       let pack = pack_e(u, t, exis_type);
       assert_eq!(
           type_of(
               unpack_e(
                   var_t("Counter".to_string()),
                   "counter".to_string(),
                   pack,
                   app_e(
                       proj_e(var_e("counter".to_string()), "get".to_string()),
                       app_e(
                           proj_e(var_e("counter".to_string()), "inc".to_string()),
                           proj_e(var_e("counter".to_string()), "new".to_string()))
                   )
                ),
                &mut Vec::new()),
           Ok(int_t())
       )
   }
}