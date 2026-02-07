use crate::components::content::projects::project_page::ProjectPage;
use yew::{Html, function_component, html};

#[function_component(FloatingPointUnit)]
pub fn floating_point_unit() -> Html {
    html! {
        <ProjectPage
            title="Floating Point Unit"
            tagline="IEEE 754 compliant floating-point arithmetic unit in SystemVerilog"
            description="A hardware implementation of a floating-point unit (FPU) supporting IEEE 754 standard operations. Built in SystemVerilog for FPGA/ASIC deployment with support for addition, subtraction, multiplication, and division operations."
            tools={vec![
                "SystemVerilog".to_string(),
                "Modelsim".to_string(),
                "Rust".to_string(),
                "Digital Design".to_string(),
            ]}
            features={vec![
                "IEEE 754 compliant operations".to_string(),
                "Single and Double precision floating-point support".to_string(),
                "Addition, subtraction, multiplication, division".to_string(),
                "Synthesizable for FPGA and ASIC".to_string(),
            ]}
            github_url="https://github.com/Ka10kenHQ/FloatingPointUnit"
            images={vec![
                "https://raw.githubusercontent.com/Ka10kenHQ/hosting/main/assets/images/floatingpointunit.png".to_string(),
            ]}
        />
    }
}
