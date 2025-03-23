extern crate core;

pub mod s_advanced;
pub mod s_closure;
pub mod s_control_flow;
pub mod s_error;
pub mod s_genericity;
pub mod s_iter;
pub mod s_lifetime;
pub mod s_macro;
pub mod s_network;
pub mod s_option_result;
pub mod s_pattern_match;
pub mod s_pointer;
pub mod s_smart_pointer;
pub mod s_struct_trait;
pub mod s_thread;
pub mod s_tokio;
pub mod s_type;
pub mod s_unsafe;

pub fn study_basic_concept() {
    s_type::study_primitive_type();
    s_type::study_compound_type();
    s_type::study_collection_type();
    s_type::study_type_convert();

    s_smart_pointer::study_smart_point();

    s_control_flow::study_condition_expression();
    s_control_flow::study_loop();

    s_option_result::study_option();
    s_option_result::study_result();

    s_struct_trait::study_struct();
    s_struct_trait::study_trait();

    s_genericity::study_genericity();

    s_closure::study_closure();

    s_iter::study_iter();

    s_macro::study_macro();
}
