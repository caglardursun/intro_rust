// mod primitive_types;
// mod ownership_barrowing;
mod execute_commands;
mod lifetime_parameter;


fn main(){
    // primitive_types::string_types();
    // primitive_types::tuple_types();
    // primitive_types::array_types();
    // primitive_types::primitives();
    // ownership_barrowing::ownership();
    // ownership_barrowing::barrowing();
    execute_commands::list_directory("/home/caglar");
    execute_commands::pipe_two_process("code");
    lifetime_parameter::test_it();
}