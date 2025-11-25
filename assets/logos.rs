// Logo definitions for Pravyom ecosystem components
// Each component displays its appropriate logo when invoked

pub const PRAVYOM_LOGO: &str = r#"
               ✶  PRAVYOM  ✶
        ┌────────────────────────┐
        │   COSMIC  LEDGER  CORE │
        │  axis • balance • flow │
        │  layers • rings •  6D  │
        └────────────────────────┘
"#;

pub const DOCKLOCK_LOGO: &str = r#"
    ██████╗  ██████╗  ██████╗██╗  ██╗██╗      ██████╗  ██████╗██╗  ██╗
    ██╔══██╗██╔═══██╗██╔════╝██║ ██╔╝██║     ██╔═══██╗██╔════╝██║ ██╔╝
    ██║  ██║██║   ██║██║     █████╔╝ ██║     ██║   ██║██║     █████╔╝ 
    ██║  ██║██║   ██║██║     ██╔═██╗ ██║     ██║   ██║██║     ██╔═██╗ 
    ██████╔╝╚██████╔╝╚██████╗██║  ██╗███████╗╚██████╔╝╚██████╗██║  ██╗
    ╚═════╝  ╚═════╝  ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
"#;

pub const ENC_CLUSTER_LOGO: &str = r#"
    ███████╗███╗   ██╗ ██████╗     ██████╗██╗     ██╗   ██╗███████╗████████╗███████╗██████╗ 
    ██╔════╝████╗  ██║██╔════╝    ██╔════╝██║     ██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗
    █████╗  ██╔██╗ ██║██║         ██║     ██║     ██║   ██║███████╗   ██║   █████╗  ██████╔╝
    ██╔══╝  ██║╚██╗██║██║         ██║     ██║     ██║   ██║╚════██║   ██║   ██╔══╝  ██╔══██╗
    ███████╗██║ ╚████║╚██████╗    ╚██████╗███████╗╚██████╔╝███████║   ██║   ███████╗██║  ██║
    ╚══════╝╚═╝  ╚═══╝ ╚═════╝     ╚═════╝╚══════╝ ╚═════╝ ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝
"#;

pub const BPI_LOGO: &str = r#"
    ██████╗ ██████╗ ██╗
    ██╔══██╗██╔══██╗██║
    ██████╔╝██████╔╝██║
    ██╔══██╗██╔═══╝ ██║
    ██████╔╝██║     ██║
    ╚═════╝ ╚═╝     ╚═╝
"#;

pub const BPCI_LOGO: &str = r#"
    ██████╗ ██████╗  ██████╗██╗
    ██╔══██╗██╔══██╗██╔════╝██║
    ██████╔╝██████╔╝██║     ██║
    ██╔══██╗██╔═══╝ ██║     ██║
    ██████╔╝██║     ╚██████╗██║
    ╚═════╝ ╚═╝      ╚═════╝╚═╝
"#;

pub const SHRUTI_LOGO: &str = r#"
    ███████╗██╗  ██╗██████╗ ██╗   ██╗████████╗██╗
    ██╔════╝██║  ██║██╔══██╗██║   ██║╚══██╔══╝██║
    ███████╗███████║██████╔╝██║   ██║   ██║   ██║
    ╚════██║██╔══██║██╔══██╗██║   ██║   ██║   ██║
    ███████║██║  ██║██║  ██║╚██████╔╝   ██║   ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝    ╚═╝   ╚═╝
"#;

/// Display the appropriate logo for the current component
pub fn display_logo(component: &str) {
    match component {
        "pravyom" | "main" | "cli" => {
            println!("{}", PRAVYOM_LOGO);
            println!("    🚀 Pravyom • Vedic digital infrastructure for Bharat & beyond");
        },
        "docklock" => {
            println!("{}", DOCKLOCK_LOGO);
            println!("    🔒 Docklock - Secure container orchestration");
        },
        "enc-cluster" => {
            println!("{}", ENC_CLUSTER_LOGO);
            println!("    ⚡ ENC Cluster - Revolutionary blockchain orchestration");
        },
        "bpi" => {
            println!("{}", BPI_LOGO);
            println!("    💰 BPI - Banking Protocol Interface");
        },
        "bpci" => {
            println!("{}", BPCI_LOGO);
            println!("    🌐 BPCI - Blockchain Platform Command Interface");
        },
        "shruti" => {
            println!("{}", SHRUTI_LOGO);
            println!("    📋 Shruti - CUE/YAML pipeline automation");
        },
        _ => {
            println!("{}", PRAVYOM_LOGO);
            println!("    🚀 Pravyom • Vedic digital infrastructure for Bharat & beyond");
        }
    }
    println!();
}
