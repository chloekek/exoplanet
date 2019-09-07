use character::Gender;
use character::Race;

/// The topmost interaction on the interaction stack is the active one. When the
/// interaction stack is empty, the player can enter arbitrary commands.
pub type InteractionStack =
    Vec<Interaction>;

/// An interaction is some state of a screen that a player is interacting with.
/// Interactions decide what prompts the player sees and what input they can
/// give.
#[derive(Clone, Debug)]
pub enum Interaction
{
    /// The player is creating a character. The prompt is placed at the first
    /// non-empty field. Once filled in, the prompt therefore moves to the next
    /// field. Once all fields are filled in, character creation is complete.
    CreateCharacter{
        name: String,
        race: Option<Race>,
        gender: Option<Gender>,
    },
}
