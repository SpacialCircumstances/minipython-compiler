use std::error::Error;
use crate::ir::*;
use crate::name::NameStore;
use inkwell::context::Context;
use inkwell::types::FunctionType;

pub fn generate_llvm_code(program: &IRProgram, name_store: &NameStore) -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);
    let main_block_entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(main_block_entry);
    let return_value = i32_type.const_int(0, false);
    builder.build_return(Some(&return_value));

    module.verify().unwrap();

    Ok(())
}