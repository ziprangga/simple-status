//! Generic rendering framework.
//!
//! This module provides traits for rendering values into arbitrary output
//! representations.
//!
//! The rendering system is generic over the input type and is not tied to any
//! specific data model.
//!
//! Applications may use renderers to:
//!
//! - Format values as text.
//! - Produce structured output for serialization.
//! - Build UI-specific view models.
//! - Convert values into application-defined representations.
//!
//! Most users will use [`Renderable::render_with`] together with either a
//! renderer type or a closure.
//!
//! ```rust
//! use simple_status::renderer::Renderable;
//!
//! let value = 42;
//!
//! let text = value.render_with(|v| format!("value={v}"));
//!
//! assert_eq!(text, "value=42");
//! ```
//!
//! Module summary.
//!
//! Doc:
//! - Defines the generic rendering framework.
//! - Documents renderer and renderable abstractions.
//! - Supports rendering values into arbitrary output types.
//!
//! Note:
//! - The framework is independent of any specific data model.
//! - Closures and functions automatically act as renderers.
//! - Rendering behavior is defined by user-provided renderer implementations.
//!....

mod display;

/// A type that can render a value of type `T` into another representation.
///
/// A renderer may produce any output type through the associated
/// [`Renderer::Output`] type.
///
/// Examples include:
///
/// - `String` for text formatting.
/// - Structured data for serialization.
/// - UI-specific view models.
/// - Application-defined output types.
///
/// Doc:
/// - Defines how a value of type `T` is transformed into another value.
/// - Supports arbitrary output types.
/// - Separates rendering logic from the underlying data model.
///
/// Note:
/// - Renderers do not modify the input value.
/// - Rendering behavior is entirely implementation-defined.
pub trait Renderer<T> {
    type Output;

    fn render(&self, value: &T) -> Self::Output;
}

/// Convenience trait that adds [`Renderable::render_with`] to a type.
///
/// Types typically do not implement this trait manually because a blanket
/// implementation is provided for all types.
///
/// Doc:
/// - Provides a convenient rendering entry point.
/// - Delegates rendering to a user-provided renderer.
/// - Works with any renderer compatible with the target type.
///
/// Note:
/// - Implemented automatically for all types.
/// - Does not impose any storage or formatting requirements.
pub trait Renderable: Sized {
    /// Renders this value using the provided renderer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_status::renderer::Renderable;
    ///
    /// let value = 42;
    ///
    /// let text = value.render_with(|v| format!("value={v}"));
    /// ```
    ///
    /// Doc:
    /// - Delegates rendering to the supplied renderer.
    /// - Supports arbitrary output types.
    ///
    /// Note:
    /// - The output type is determined by the renderer.
    /// - No allocation is performed unless required by the renderer.
    fn render_with<R>(&self, renderer: R) -> R::Output
    where
        R: Renderer<Self>,
    {
        renderer.render(self)
    }
}

/// Automatically implements [`Renderer`] for compatible closures and
/// functions.
///
/// Doc:
/// - Enables closures and functions to be used directly as renderers.
/// - Reduces boilerplate for simple rendering operations.
///
/// Note:
/// - This implementation is primarily an ergonomics feature.
/// - Most users will not need to define a dedicated renderer type.
impl<T, F, O> Renderer<T> for F
where
    F: Fn(&T) -> O,
{
    type Output = O;

    fn render(&self, value: &T) -> Self::Output {
        (self)(value)
    }
}

/// Every type automatically becomes `Renderable`.
impl<T> Renderable for T {}
