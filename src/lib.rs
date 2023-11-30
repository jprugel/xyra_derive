extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let struct_name = extract_struct_name(&input_str);

    let gen = format!(
        r#"
        impl xyra::ecs::Component for {} {{
            fn as_any(&self) -> &dyn std::any::Any {{
                self
            }}

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {{
                self
            }}

        }}
    "#,
        struct_name
    );

    gen.parse().expect("Failed to parse generated code")
}

fn extract_struct_name(input: &str) -> String {
    let start = input.find("struct ").expect("Failed to find 'struct '") + 7;
    let end = input.find('{').expect("Failed to find '{'");

    input[start..end].trim().to_string()
}
