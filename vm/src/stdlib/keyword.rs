/*
 * Testing if a string is a keyword.
 */

extern crate rustpython_parser;
use self::rustpython_parser::lexer;
use crate::obj::objstr;
use crate::pyobject::{PyContext, PyFuncArgs, PyObjectRef, PyResult, TypeProtocol};
use crate::VirtualMachine;

fn keyword_iskeyword(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(s, Some(vm.ctx.str_type()))]);
    let s = objstr::get_value(s);
    let keywords = lexer::get_keywords();
    let value = keywords.contains_key(&s);
    let value = vm.ctx.new_bool(value);
    Ok(value)
}

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    let py_mod = ctx.new_module("keyword", ctx.new_scope(None));

    ctx.set_attr(&py_mod, "iskeyword", ctx.new_rustfunc(keyword_iskeyword));

    let keyword_kwlist = ctx.new_list(
        lexer::get_keywords()
            .keys()
            .map(|k| ctx.new_str(k.to_string()))
            .collect(),
    );
    ctx.set_attr(&py_mod, "kwlist", keyword_kwlist);

    py_mod
}
