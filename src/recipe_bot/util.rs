
use recipe_bot::commands::error::CmdError;

/// translates the CmdError for this Command
pub fn translate_error(e: &CmdError) -> &str{
    match e {
        &CmdError::NotFound => "Das Rezept wurde nicht gefunden",
        &CmdError::NoArguments => "Bitte geben sie die Namen der gesuchten Rezepte ",
        &CmdError::UnknownError => "Ein unbekannter Fehler ist aufgetreten",
        &CmdError::DatabaseAccessError => "Fehler beim Datenbank zugriff",
        &CmdError::InvalidInput => "Ihre eingabe ist ungültig.",
    }
}