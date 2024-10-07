mod tui;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut _terminal = tui::init_terminal()?;

    tui::restore_terminal()?;

    Ok(())
}
