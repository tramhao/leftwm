use crate::layouts::Layout;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Command {
    Execute(String),
    CloseWindow,
    SwapScreens,
    SoftReload,
    HardReload,
    ToggleScratchPad(String),
    ToggleFullScreen,
    ToggleSticky,
    GotoTag(usize),
    FloatingToTile,
    MoveWindowUp,
    MoveWindowDown,
    MoveWindowTop,
    FocusNextTag,
    FocusPreviousTag,
    FocusWindowUp,
    FocusWindowDown,
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    SendWindowToTag(usize),
    MoveWindowToLastWorkspace,
    MoveWindowToNextWorkspace,
    MoveWindowToPreviousWorkspace,
    MouseMoveWindow,
    NextLayout,
    PreviousLayout,
    SetLayout(Layout),
    RotateTag,
    IncreaseMainWidth(i8),
    DecreaseMainWidth(i8),
    SetMarginMultiplier(f32),
    SendWorkspaceToTag(usize, usize),
    Other(String),
}
