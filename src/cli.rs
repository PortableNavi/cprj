
use clap::{arg, Command};


fn new() -> Command
{
    Command::new("new")
        .about("Create a new project.")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(false)
        .arg(arg!(--template <TEMPLATE>)
            .require_equals(true)
            .num_args(1)
            .required(true)
        )
        .arg(arg!(<NAME> "Your Projects Name"))
}


fn list_templates() -> Command
{
    Command::new("list_templates")
        .about("See all available project templates.")
        .subcommand_required(false)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(false)
        .arg(arg!(-l --line "Do not place each template name on a new line. Display them all in one line instead."))
}


fn build() -> Command
{
    Command::new("build")
        .about("Build your project.")
        .subcommand_required(false)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(false)
}


fn run() -> Command
{
    Command::new("run")
        .about("Build and run your project.")
        .subcommand_required(false)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(false)
}


pub fn build_cli() -> Command
{
    Command::new("cprj")
        .about("Simple C project creation tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(new())
        .subcommand(build())
        .subcommand(run())
        .subcommand(list_templates())
}