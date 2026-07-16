use std::fs;

fn main() {
    let mut code = fs::read_to_string("tb_ui/src/tabs/alpha_foundry_temp2.rs").unwrap();
    let mut new_code = code.replace("        });\n    }\n\n}\n\npub fn render_leaderboard", "        }\n    }\n\n}\n\npub fn render_leaderboard");
    new_code = new_code.replace("    });\n\n}\n", "    }\n\n}\n");
    fs::write("tb_ui/src/tabs/alpha_foundry.rs", new_code).unwrap();
}
