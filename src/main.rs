use std::cmp::max;
use std::collections::btree_map::BTreeMap;
use std::env::args;
use std::io::{BufRead, Write};
use std::process::exit;

use Stoichio_calc::chemistry::{balance, PeriodicTable};
use Stoichio_calc::data_loading::load_periodic_table;
use Stoichio_calc::parsing::{parse_molecule, parse_raw_equation, PositionedError, tokenize};

type ArgsCommand = fn(&str, &Context) -> Result<(), PositionedError>;
type NoArgsCommand = fn(&Context) -> Result<(), PositionedError>;

struct Context<'a> {
    periodic_table: PeriodicTable,
    args_cmds: BTreeMap<&'a str, (&'a ArgsCommand, &'a str)>,
    no_args_cmds: BTreeMap<&'a str, (&'a NoArgsCommand, &'a str)>,
}

fn main() {

    // include resource file in .exe
    let periodic_table_file_content = include_str!("../res/periodic_table.csv");

    let ctx = Context {
        periodic_table: load_periodic_table(periodic_table_file_content),
        args_cmds: BTreeMap::from([
            ("mass", (&(compute_mass_cmd as ArgsCommand),
                      "mass <molecule> - display the atomic mass of the molecule in uma")),
            ("balance", (&(balance_equation_cmd as ArgsCommand),
                         "balance <equation> - balances the equation, e.g. 'balance H2 + O2 => H2O'"))
        ]),
        no_args_cmds: BTreeMap::from([
            ("exit", (&(exit_cmd as NoArgsCommand), "exit - exit the program")),
            ("help", (&(help_cmd as NoArgsCommand), "help - display the current explanations"))
        ]),
    };

    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() { run_cli(&ctx); }
    else { execute_cmd(&ctx, &args.join("")); }

}

fn run_cli(ctx: &Context){
    println!("\n -------------------- Stoichiometry calculator CLI -------------------- \n");
    display_help(&ctx);
    display_input_line_header();
    for line_res in std::io::stdin().lock().lines() {
        if let Ok(raw_line) = line_res {
            execute_cmd(ctx, &raw_line)
        } else { println!("input line error") }
        display_input_line_header();
    }
}

fn execute_cmd(ctx: &Context, raw_line: &String) {
    let trimmed_line = raw_line.trim();
    let sp: Vec<&str> = trimmed_line
        .splitn(2, ' ')
        .collect();
    let result: Result<(), PositionedError> = match sp[..] {
        [] => Ok(()), // empty line, do nothing
        [cmd] => call_no_arg_command(cmd, &ctx),
        [cmd, args] => call_arg_command(&cmd.to_lowercase(), args, &ctx),
        _ => panic!("should not happen")
    };
    if let Err(PositionedError(msg, pos_opt)) = result {
        println!("an error occured: {}", msg);
        if let Some(pos) = pos_opt {
            let padding_len = sp[0].len() + max(1 + pos, 0) as usize;
            let padding = str::repeat(" ", padding_len);
            println!("{}", trimmed_line);
            println!("{}^", padding)
        }
    }
}

fn call_no_arg_command(cmd: &str, ctx: &Context) -> Result<(), PositionedError> {
    if let Some((cmd_fn, _)) = ctx.no_args_cmds.get(cmd) {
        cmd_fn(&ctx)
    } else if ctx.args_cmds.contains_key(cmd) {
        Err(PositionedError(
            format!("{} needs argument(s)", cmd),
            None,
        ))
    } else {
        Err(PositionedError(
            format!("unknown command: {}", cmd),
            None,
        ))
    }
}

fn call_arg_command(cmd: &str, args: &str, ctx: &Context) -> Result<(), PositionedError> {
    if let Some((cmd_fn, _)) = ctx.args_cmds.get(cmd) {
        cmd_fn(args, &ctx)
    } else if ctx.no_args_cmds.contains_key(cmd) {
        Err(PositionedError(
            format!("{} does not take arguments", cmd),
            None,
        ))
    } else {
        Err(PositionedError(
            format!("unknown command: {}", cmd),
            None,
        ))
    }
}

fn display_input_line_header() {
    print!("> ");
    let _ = std::io::stdout().flush();
}

fn display_help(ctx: &Context) {
    for cmd in &ctx.args_cmds {
        println!("{}", cmd.1.1)
    }
    for cmd in &ctx.no_args_cmds {
        println!("{}", cmd.1.1)
    }
    println!();
}

fn compute_mass_cmd(args: &str, ctx: &Context) -> Result<(), PositionedError> {
    match parse_molecule(&ctx.periodic_table, &tokenize(&args.to_string())) {
        Ok(molecule) => {
            Ok(println!("molecular mass: {} uma", molecule.mass_uma()))
        }
        Err(msg) => Err(msg)
    }
}

fn balance_equation_cmd(args: &str, ctx: &Context) -> Result<(), PositionedError> {
    match parse_raw_equation(&ctx.periodic_table, &tokenize(&args.to_string())){
        Ok(raw_equation) => {
            match balance(&raw_equation){
                Ok(balanced_equation) => {
                    println!("{}", balanced_equation);
                    Ok(())
                }
                Err(pos_err) => Err(pos_err)
            }
        }
        Err(pos_err) => Err(pos_err)
    }
}

fn exit_cmd(_ctx: &Context) -> Result<(), PositionedError> {
    exit(0)
}

fn help_cmd(ctx: &Context) -> Result<(), PositionedError> {
    display_help(ctx);
    Ok(())
}
