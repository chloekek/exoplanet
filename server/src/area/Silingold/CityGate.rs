use area::AreaStatic;

pub static STATIC: AreaStatic = AreaStatic{
    description: concat!(
        "To the east you see the gate to a seemingly large city. ",

        "You see countless guards verifying the identities of all those ",
        "passing through the gate, and inspecting every item they carry with ",
        "them. ",

        "Sneaking through the gate unnoticed seems impossible, and attacking ",
        "the guards does not seem like a very good idea."

        // TODO: Describe in greater detail what the gate looks like.
        // TODO: Describe in greater detail what the guards look like.
    ),
};
