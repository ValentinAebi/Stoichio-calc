use std::cmp::max;
use std::io::{BufRead, Write};
use std::collections::BTreeMap;
use std::process::exit;
use Stoichio_calc::chemistry::Atom;
use Stoichio_calc::data_loading::load_periodic_table_as_map;
use Stoichio_calc::parsing::{parse_molecule, PositionedError, tokenize};

type PeriodicTable = BTreeMap<String, Atom>;
type ArgsCommand = fn(&str, &PeriodicTable) -> Result<(), PositionedError>;
type NoArgsCommand = fn(&PeriodicTable) -> Result<(), PositionedError>;

const HELP_TEXT_LINES: [&str; 3] = [
    "mass <molecule> - display the atomic mass of the molecule in uma",
    "help - display the current explanations",
    "exit - exit the program"
];

struct Context<'a> {
    periodic_table: BTreeMap<String, Atom>,
    args_cmds: BTreeMap<&'a str, &'a ArgsCommand>,
    no_args_cmds: BTreeMap<&'a str, &'a NoArgsCommand>,
}

fn main() {

    let compute_mass: ArgsCommand = compute_mass_cmd;
    let exit: NoArgsCommand = exit_cmd;
    let help: NoArgsCommand = help_cmd;

    let ctx = Context {
        periodic_table: load_periodic_table_as_map(),
        args_cmds: BTreeMap::from([
            ("mass", &compute_mass)
        ]),
        no_args_cmds: BTreeMap::from([
            ("exit", &exit),
            ("help", &help)
        ]),
    };

    println!("\n -------------------- Stoechiometry calculator CLI -------------------- \n");
    display_help();

    display_input_line_header();
    for line_res in std::io::stdin().lock().lines() {
        if let Ok(raw_line) = line_res {
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
        } else { println!("input line error") }
        display_input_line_header();
    }
}

fn call_no_arg_command(cmd: &str, ctx: &Context) -> Result<(), PositionedError> {
    if let Some(cmd_fn) = ctx.no_args_cmds.get(cmd) {
        cmd_fn(&ctx.periodic_table)
    } else if ctx.args_cmds.contains_key(cmd) {
        Err(PositionedError(
            format!("{} needs argument(s)", cmd),
            None
        ))
    } else {
        Err(PositionedError(
            format!("unknown command: {}", cmd),
            None
        ))
    }
}

fn call_arg_command(cmd: &str, args: &str, ctx: &Context) -> Result<(), PositionedError> {
    if let Some(cmd_fn) = ctx.args_cmds.get(cmd) {
        cmd_fn(args, &ctx.periodic_table)
    } else if ctx.no_args_cmds.contains_key(cmd) {
        Err(PositionedError(
            format!("{} does not take arguments", cmd),
            None
        ))
    } else {
        Err(PositionedError(
            format!("unknown command: {}", cmd),
            None
        ))
    }
}

fn display_input_line_header(){
    print!("> ");
    let _ = std::io::stdout().flush();
}

fn display_help(){
    for line in HELP_TEXT_LINES {
        println!("{}", line)
    }
    println!();
}

fn compute_mass_cmd(args: &str, periodic_table: &BTreeMap<String, Atom>) -> Result<(), PositionedError> {
    match parse_molecule(periodic_table, &tokenize(&args.to_string())) {
        Ok(molecule) => {
            Ok(println!("molecular mass: {} uma", molecule.mass_uma()))
        }
        Err(msg) => Err(msg)
    }
}

fn exit_cmd(_periodic_table: &PeriodicTable) -> Result<(), PositionedError> {
    exit(0)
}

fn help_cmd(_periodic_table: &PeriodicTable) -> Result<(), PositionedError> {
    display_help();
    Ok(())
}
