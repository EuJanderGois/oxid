//! The engine's scripting abstraction.
//!
//! [`ScriptRuntime`] is the boundary between Oxid and whatever JavaScript
//! engine currently powers it. The rest of the engine (see
//! [`crate::runtime::game`](../../runtime/game/index.html), the main game
//! loop) only ever talks to a `ScriptRuntime`; today the only implementation
//! is [`crate::scripting::quickjs::QuickJsRuntime`], built on top of
//! `rquickjs`/QuickJS, but nothing outside `scripting::quickjs` needs to know
//! that. This mirrors [`crate::renderer::Renderer`] /
//! [`crate::renderer::MqRenderer`], which draws the same kind of line
//! between the engine and Macroquad.
//!
//! The trait only exposes the three things the engine's main loop actually
//! calls: the `onInit` / `onUpdate` / `onDraw` lifecycle hooks. Loading a
//! project's scripts, wiring up `oxid/*` native modules, and resolving ES
//! Module imports are all part of *constructing* a runtime, which is
//! necessarily backend-specific (it depends on how, or whether, the backend
//! represents modules and contexts internally), so it is deliberately left
//! out of this trait. Each backend exposes its own constructor instead (see
//! [`crate::scripting::quickjs::QuickJsRuntime::new`]).

use crate::renderer::queue::RenderQueue;

/// A running instance of the Oxid scripting layer for one project.
///
/// Implementors own whatever JavaScript engine state they need (a QuickJS
/// `Runtime`/`Context`, or, in the future, something else entirely) and are
/// responsible for keeping it private: nothing here requires exposing that
/// state to callers, and implementations should not do so unless a concrete
/// need arises.
pub trait ScriptRuntime {
    /// Invokes the script app's `onInit()`, if it defines one.
    fn on_init(&self);

    /// Invokes the script app's `onUpdate(deltaTime)`, if it defines one.
    fn on_update(&self, delta_time: f32);

    /// Invokes the script app's `onDraw()`, if it defines one, recording any
    /// draw calls it makes into `queue`.
    fn on_draw(&self, queue: &mut RenderQueue);
}
