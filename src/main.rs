use std::env;

use anyhow::{Context, Ok, Result};
use args::{Cli, PluginCommand};
use clap::Parser;
use dialoguer::Input;

mod args;

fn main() -> Result<()> {
    let args: PluginCommand = Cli::parse().into();

    match args {
        PluginCommand::Hello => hello(),
        PluginCommand::Printenv => printenv(),
        PluginCommand::Stdinreader => stdinreader(),
    }
}

fn hello() -> Result<()> {
    println!("Hello world!");

    Ok(())
}

fn printenv() -> Result<()> {
    println!("Environment variables: ");

    for (key, value) in env::vars() {
        println!("\t- {key}={value}");
    }

    Ok(())
}

fn stdinreader() -> Result<()> {
    let name = Input::<String>::new()
        .with_prompt("Please enter your name")
        .interact_text()
        .context("prompting name")?;

    println!("Hello, {name}!");

    Ok(())
}
