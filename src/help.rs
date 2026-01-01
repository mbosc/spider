use crate::InputState;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Keybinding {
    pub key: &'static str,
    pub text: &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum KeybindingContext {
    NonCheatMenu,
    SelectSource,
    SelectDestination,
    CheatMenu,
}

static KEYBINDINGS: &[(Keybinding, KeybindingContext)] = &[
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a row to move cards from",
        },
        KeybindingContext::SelectSource,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a row to move cards to",
        },
        KeybindingContext::SelectDestination,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a cheat to apply",
        },
        KeybindingContext::SelectDestination,
    ),
    (
        Keybinding {
            key: "[Enter]",
            text: "Deal row of cards",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[u]",
            text: "Undo",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[c]",
            text: "Quit",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[q]",
            text: "Quit",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[C]",
            text: "Cheats",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[R]",
            text: "Restart",
        },
        KeybindingContext::NonCheatMenu,
    ),
    (
        Keybinding {
            key: "[0-9]",
            text: "Select a cheat",
        },
        KeybindingContext::CheatMenu,
    ),
    (
        Keybinding {
            key: "[q]",
            text: "Exit menu",
        },
        KeybindingContext::CheatMenu,
    ),
    (
        Keybinding {
            key: "[esc]",
            text: "Exit menu",
        },
        KeybindingContext::CheatMenu,
    ),
];

pub fn get_keybindings(state: InputState) -> Vec<Keybinding> {
    KEYBINDINGS
        .iter()
        .filter(|(_, ctx)| match state {
            InputState::SelectSource => {
                matches!(
                    ctx,
                    KeybindingContext::NonCheatMenu | KeybindingContext::SelectSource
                )
            }
            InputState::SelectDestination(_) => {
                matches!(
                    ctx,
                    KeybindingContext::NonCheatMenu | KeybindingContext::SelectDestination
                )
            }
            InputState::CheatMenu => matches!(ctx, KeybindingContext::CheatMenu),
        })
        .map(|(kb, _)| *kb)
        .collect()
}
