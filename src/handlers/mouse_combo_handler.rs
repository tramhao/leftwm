use crate::display_action::DisplayAction;
use crate::models::Manager;
use crate::models::Mode;
use crate::models::WindowHandle;
use crate::utils::xkeysym_lookup::Button;
use crate::utils::xkeysym_lookup::ModMask;
use x11_dl::xlib;

pub fn process(
    manager: &mut Manager,
    modmask: ModMask,
    button: Button,
    handle: WindowHandle,
) -> bool {
    //look through the config and build a command if its defined in the config
    let act = build_action(manager, modmask, button, handle.clone());
    if let Some(act) = act {
        //new move/resize. while the old starting points
        for w in &mut manager.windows {
            if w.handle == handle {
                w.set_floating(true);
            }
        }
        manager.actions.push_back(act);
        manager.actions.push_back(DisplayAction::MoveToTop(handle));
    }
    false
}

fn build_action(
    manager: &mut Manager,
    _mod_mask: ModMask,
    button: Button,
    window: WindowHandle,
) -> Option<DisplayAction> {
    match button {
        xlib::Button1 => {
            for w in &manager.windows {
                if w.handle == window && w.can_move() {
                    manager.mode = Mode::MovingWindow(window.clone());
                    return Some(DisplayAction::StartMovingWindow(window));
                }
            }
            None
        }
        xlib::Button3 => {
            for w in &manager.windows {
                if w.handle == window && w.can_resize() {
                    manager.mode = Mode::ResizingWindow(window.clone());
                    return Some(DisplayAction::StartResizingWindow(window));
                }
            }
            None
        }
        _ => None,
    }
}
