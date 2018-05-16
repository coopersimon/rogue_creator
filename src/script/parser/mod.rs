mod resolver;

use script::ast::*;
use script::runtime::Value;

use self::resolver::Resolver;

use nom::IResult;
use nom::{multispace, alpha, alphanumeric, double, digit};

use std::str;
use std::cell::RefCell;
use std::collections::BTreeMap;


// For resolving context-specific package refrences
thread_local!(static RESOLVER: RefCell<Resolver> = RefCell::new(Resolver::new()));

fn get_package_ref(package_ref: Option<&str>) -> String {
    RESOLVER.with(|r| r.borrow().get_package_ref(package_ref).expect("Couldn't find package."))
}

fn add_package_ref(package_ref: &str, package_name: &str) {
    RESOLVER.with(|r| r.borrow_mut().add_package_ref(package_ref, package_name));
}


// TOP LEVEL
pub fn parse_package(name: &str, text: &str) -> ScriptPackage {
    RESOLVER.with(|r| r.borrow_mut().set_package(name));

    let mut output = match p_func_list(text.as_bytes()) {
        IResult::Done(_,o) => o,
        IResult::Incomplete(i) => panic!("Incomplete: {:?}", i),
        IResult::Error(e) => panic!("Error: {:?}", e),
    };

    // convert vec into map
    let mut package = BTreeMap::new();

    while let Some((n, f)) = output.pop() {
        package.insert(n, f);
    }

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    ScriptPackage::new(package)
}


/*pub fn parse_func(text: &str) -> (String, FuncRoot) {
    let (s, f) = match p_func(text.as_bytes()) {
        IResult::Done(_,(s_o,f_o)) => (s_o,f_o),
        IResult::Incomplete(i) => panic!("Incomplete: {:?}", i),
        IResult::Error(e) => panic!("Error: {:?}", e),
    };

    (s, f)
}*/




// TOKENS
const FUNC: &'static str = "func";
const VAR: &'static str = "var";
const RETURN: &'static str = "return";
const FOR: &'static str = "for";
const WHILE: &'static str = "while";
const IF: &'static str = "if";
const ELSE: &'static str = "else";
// continue
// break
// in
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";


// PARSER

named!(p_func_list<&[u8], Vec<(String, FuncRoot)> >,
    many1!(
        do_parse!(
            f: p_func                       >>
            opt!(alt_complete!(multispace)) >>
            (f)
        )
    )
);

named!(p_func<&[u8], (String, FuncRoot)>,
    do_parse!(
        tag!(FUNC)          >>
        multispace          >>
        name: p_id          >>
        opt!(multispace)    >>
        tag!("(")           >>
        opt!(multispace)    >>
        args: p_id_list     >>
        opt!(multispace)    >>
        tag!(")")           >>
        opt!(multispace)    >>
        body: p_func_body   >>
        (name, FuncRoot::new(args, body))
    )
);

named!(p_id<&[u8], String>,
    map_res!(alphanumeric, make_id)
);

named!(p_id_list<&[u8], Vec<String> >,
    do_parse!(
        args: many0!(
            do_parse!(
                arg: p_id           >>
                opt!(multispace)    >>
                tag!(",")           >>
                opt!(multispace)    >>
                (arg)
            )
        )                       >>
        final_arg: opt!(p_id)   >>
        (combine_list(args, final_arg))
    )
);

named!(p_func_body<&[u8], Vec<Box<Statement> > >,
    do_parse!(
        tag!("{")           >>
        opt!(multispace)    >>
        stats: p_stat_list  >>
        tag!("}")           >>
        (stats)
    )
);

named!(p_stat_list<&[u8], Vec<Box<Statement> > >,
    many0!(
        do_parse!(
            s: p_stat           >>
            opt!(multispace)    >>
            (s)
        )
    )
);

named!(p_stat<&[u8], Box<Statement> >,
    alt!(
        p_scope         |
        p_return_stat   |
        p_if_stat       |
        //p_for_stat       |
        p_while_stat    |
        p_decl_stat     |
        p_assign_stat
    )
);

named!(p_scope<&[u8], Box<Statement> >,
    do_parse!(
        tag!("{")           >>
        opt!(multispace)    >>
        stats: p_stat_list  >>
        tag!("}")           >>
        (Box::new(ScopeStat::new(stats)))
    )
);

named!(p_return_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(RETURN)        >>
        e: opt!(do_parse!(
            multispace  >>
            e: p_expr   >>
            (e)
        ))                  >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(ReturnStat::new(e)))
    )
);

named!(p_if_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(IF)            >>
        cond: p_wrap_expr   >>
        opt!(multispace)    >>
        then: p_stat        >>
        e: opt!(
            do_parse!(
                opt!(multispace)    >>
                tag!(ELSE)          >>
                s: p_stat           >>
                (s)
            )
        )                   >>
        (Box::new(IfStat::new(cond, then, e)))
    )
);

named!(p_while_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(WHILE)         >>
        c: p_wrap_expr      >>
        opt!(multispace)    >>
        b: p_stat           >>
        (Box::new(WhileStat::new(c, b)))
    )
);

named!(p_wrap_expr<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            tag!("(")           >>
            opt!(multispace)    >>
            e: p_expr           >>
            opt!(multispace)    >>
            tag!(")")           >>
            (e)
        )   |
        do_parse!(
            multispace  >>
            e: p_expr   >>
            (e)
        )
    )
);

named!(p_decl_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(VAR)           >>
        multispace          >>
        v: p_id             >>
        opt!(multispace)    >>
        a: opt!(p_assign)   >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(VarDecl::new(&v,a)))
    )
);

named!(p_assign<&[u8], Box<Expr> >,
    do_parse!(
        tag!("=")           >>
        opt!(multispace)    >>
        e: p_expr           >>
        (e)
    )
);

named!(p_assign_stat<&[u8], Box<Statement> >,
    do_parse!(
        v: p_id             >>
        opt!(multispace)    >>
        e: p_assign         >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(AssignStat::new(&v,e)))
    )
);

named!(p_expr<&[u8], Box<Expr> >,
    alt!(
        p_add_sub
    )
);

named!(p_add_sub<&[u8], Box<Expr> >,
    alt!(
        p_add   |
        p_sub   |
        p_mul_div
    )
);

named!(p_add<&[u8], Box<Expr> >,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("+")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Box::new(AddExpr::new(a, b)))
    )
);

named!(p_sub<&[u8], Box<Expr> >,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("-")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Box::new(SubExpr::new(a, b)))
    )
);

named!(p_mul_div<&[u8], Box<Expr> >,
    alt!(
        p_mul   |
        p_div   |
        p_func_expr
    )
);

named!(p_mul<&[u8], Box<Expr> >,
    do_parse!(
        a: p_func_expr      >>
        opt!(multispace)    >>
        tag!("*")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Box::new(MulExpr::new(a, b)))
    )
);

named!(p_div<&[u8], Box<Expr> >,
    do_parse!(
        a: p_func_expr      >>
        opt!(multispace)    >>
        tag!("/")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Box::new(DivExpr::new(a, b)))
    )
);

named!(p_func_expr<&[u8], Box<Expr> >,
    alt!(
        p_func_call |
        p_prim_expr
    )
);

named!(p_func_call<&[u8], Box<Expr> >,
    do_parse!(
        f: p_id                 >>
        tag!("(")               >>
        opt!(multispace)        >>
        args: opt!(p_expr_list) >>
        opt!(multispace)        >>
        tag!(")")               >>
        (Box::new(FuncCall::new(&get_package_ref(None), &f, args)))
    )
);

named!(p_prim_expr<&[u8], Box<Expr> >,
    alt!(
        p_float_expr    |
        p_int_expr      |
        p_text_expr     |
        p_bool_expr     |
        p_id_expr       |
        p_par_expr
    )
);

named!(p_float_expr<&[u8], Box<Expr> >,
    do_parse!(
        f: double   >>
        (Box::new(ValExpr::Float(f)))
    )
);

named!(p_int_expr<&[u8], Box<Expr> >,
    do_parse!(
        i: digit    >>
        (Box::new(ValExpr::Int(str_to_int(i).unwrap())))
    )
);

named!(p_text_expr<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            tag!("\"")              >>
            s: take_until!("\"")    >>
            tag!("\"")              >>
            (Box::new(ValExpr::Text(String::from_utf8(s.to_vec()).unwrap())) as Box<Expr>)
        )   |
        do_parse!(
            tag!("\'")              >>
            s: take_until!("\'")    >>
            tag!("\'")              >>
            (Box::new(ValExpr::Text(String::from_utf8(s.to_vec()).unwrap())) as Box<Expr>)
        )
    )
);

named!(p_bool_expr<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            tag!(TRUE)  >>
            (Box::new(ValExpr::Bool(true)) as Box<Expr>)
        )   |
        do_parse!(
            tag!(FALSE)  >>
            (Box::new(ValExpr::Bool(false)) as Box<Expr>)
        )
    )
);

named!(p_id_expr<&[u8], Box<Expr> >,
    do_parse!(
        id: p_id    >>
        (Box::new(ValExpr::Var(id)))
    )
);

named!(p_par_expr<&[u8], Box<Expr> >,
    do_parse!(
        tag!("(")           >>
        opt!(multispace)    >>
        e: p_expr           >>
        opt!(multispace)    >>
        tag!(")")           >>
        (e)
    )
);

named!(p_expr_list<&[u8], Vec<Box<Expr> > >,
    do_parse!(
        exprs: many0!(
            do_parse!(
                e: p_expr           >>
                opt!(multispace)    >>
                tag!(",")           >>
                opt!(multispace)    >>
                (e)
            )
        )                           >>
        final_expr: opt!(p_expr)    >>
        (combine_list(exprs, final_expr))
    )
);


fn make_id(id: &[u8]) -> Result<String, String> {
    let s = str::from_utf8(id).unwrap();

    /*let s = match end {
        Some(s) => out.to_string() + str::from_utf8(s).unwrap(),
        None => out.to_string()
    };*/

    match s.chars().next() {
        Some(c) => if !c.is_alphabetic() {
            return Err(format!("Id begins with non-letter."));
        },
        None => return Err(format!("Id has no characters.")),
    }
    
    // check id isn't a keyword.
    match s {
        FUNC    => Err(format!("{} matches keyword", s)),
        VAR     => Err(format!("{} matches keyword", s)),
        RETURN  => Err(format!("{} matches keyword", s)),
        FOR     => Err(format!("{} matches keyword", s)),
        WHILE   => Err(format!("{} matches keyword", s)),
        IF      => Err(format!("{} matches keyword", s)),
        ELSE    => Err(format!("{} matches keyword", s)),
        TRUE    => Err(format!("{} matches keyword", s)),
        FALSE   => Err(format!("{} matches keyword", s)),
        _ => Ok(s.to_string()),
    }
}

fn combine_list<T>(mut args: Vec<T>, final_arg: Option<T>) -> Vec<T> {
    match final_arg {
        Some(a) => args.push(a),
        None => {},
    }
    args
}

fn str_to_int(s: &[u8]) -> Result<i64, String> {
    match str::from_utf8(s) {
        Ok(i_str) => match i_str.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(format!("Not an integer: {}", i_str)),
        },
        Err(_) => Err(format!("Incorrectly parsed input string.")),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use script::runtime::{GlobState, FuncMap, ExprSig};
    use std::collections::BTreeMap;

    #[test]
    fn parse_single_func() {
        let func = "func f(){}";

        let p = parse_package("p", func);

        assert!(p.funcs.contains_key("f"));

        let f = p.funcs.get("f").unwrap();

        assert_eq!(f.get_arg_names(), Vec::<String>::new().as_slice());
    }

    #[test]
    fn parse_func_arg() {
        let func = "func f(x){}";

        let p = parse_package("p", func);

        let f = p.funcs.get("f").unwrap();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), (vec!["x".to_string()]).as_slice());
    }

    #[test]
    fn parse_func_args() {
        let func = "func f(a, b){}";

        //let (s,f) = parse_func(func);
        let p = parse_package("p", func);

        let f = p.funcs.get("f").unwrap();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), (vec!["a".to_string(), "b".to_string()]).as_slice());
    }

    #[test]
    fn parse_func_body() {
        let func = "func f() {return;}";

        let p = parse_package("p", func);

        let f = p.funcs.get("f").unwrap();

        let mut g = GlobState::new();
        let fm = FuncMap::new();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), Vec::<String>::new().as_slice());

        // run func, check it returns null
        assert_eq!(f.call(&Vec::new(), &mut g, &fm), ExprSig::Value(Value::Null));
    }

    #[test]
    fn parse_func_body_and_attach() {
        let func = "func f() {return;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        // run func, check it returns null
        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new(),
                              &mut g),
                   ExprSig::Value(Value::Null));
    }

    // expr tests
    #[test]
    fn parse_func_with_return_expr() {
        let func = "func f() {return 3+2;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new(),
                              &mut g),
                   ExprSig::Value(Value::Int(5)));
    }

    #[test]
    fn parse_func_with_return_from_args() {
        let func = "func add(a,b) {return a+b;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "add",
                              &vec![Value::Int(3),Value::Int(2)],
                              &mut g),
                   ExprSig::Value(Value::Int(5)));
    }

    #[test]
    fn parse_func_with_string() {
        let func = "func hello(n) {return 'Hello ' + n;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "hello",
                              &vec![Value::Int(3)],
                              &mut g),
                   ExprSig::Value(Value::Str("Hello 3".to_string())));
    }

    // This test is a bit gratuitous
    #[test]
    fn parse_func_with_return_args_expr_cast() {
        let func = "func add(a,b) {return a+b+1.1;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "add",
                              &vec![Value::Float(1.1),Value::Float(1.0)],
                              &mut g),
                   ExprSig::Value(Value::Float(3.2)));
    }

    // func calls
    #[test]
    fn parse_funcs_with_func_call() {
        let func = "func parent(x){return child(x) + x;} func child(x){return x*2;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "parent",
                              &vec![Value::Int(3)],
                              &mut g),
                   ExprSig::Value(Value::Int(9)));
    }

    // if test
    #[test]
    fn parse_func_with_if() {
        let func = "func zero(x) {if x return false; return true;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "zero",
                              &vec![Value::Int(0)],
                              &mut g),
                   ExprSig::Value(Value::Bool(true)));
    }

    // recurisve
    #[test]
    fn parse_recursive_func() {
        let func = "func fact(x) { if x return fact(x-1)*x; return 1; }";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "fact",
                              &vec![Value::Int(5)],
                              &mut g),
                   ExprSig::Value(Value::Int(120)));
    }

    // declaring and using variables
    #[test]
    fn parse_func_with_local_variables() {
        let func = "func f(x) {var a = 3+x; return a*2;}";
        let package_name = "root";

        let package = parse_package(package_name, func);

        let mut g = GlobState::new();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &vec![Value::Str("dupe".to_string())],
                              &mut g),
                   ExprSig::Value(Value::Str("3dupe3dupe".to_string())));
    }
}
