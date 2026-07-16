/// Internal implementation of [`status!`].
///
/// Doc:
/// - Parses all supported `status!` invocation forms.
/// - Constructs a `StatusEvent` through internal helper functions.
/// - Exists to keep the public macro as a thin forwarding layer.
///
/// Note:
/// - Not part of the public API.
/// - May change at any time without notice.
/// - Public callers should use [`status!`] instead.
/// - Separated from the public macro to simplify maintenance and reduce
///   duplication when the parsing logic evolves.
#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __status {
    (
        $(id: $id:expr,)?
        $(action: $action:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{

        $crate::__status!(@build
            $($id)?
            ;
            $crate::__status!(@opt_str $($action)?),
            $crate::__status!(@opt_usize $($current)?),
            $crate::__status!(@opt_usize $($total)?),
            $crate::__status!(@opt_str $($message)?),
            $crate::__status!(@opt_path $($path)?)
        )
    }};

    ($(id: $id:expr,)? $message:expr) => {{
            $crate::__status!(@build
                $($id)?
                ;
                None::<&'static str>,
                None,
                None,
                $message,
                None
            )
        }};

    ($(id: $id:expr,)? $fmt:expr, $($arg:tt)+) => {{
            $crate::__status!(@build
                $($id)?
                ;
                None::<&'static str>,
                None,
                None,
                format!($fmt, $($arg)+),
                None
            )
        }};

    // --------------------------
    // internal id dispatch
    // --------------------------

    (@build
        $id:expr;
        $action:expr,
        $current:expr,
        $total:expr,
        $message:expr,
        $path:expr
    ) => {
        $crate::__private_helper::build_status_event_id(
            $id,
            $action,
            $current,
            $total,
            $message,
            $path,
        )
    };

    (@build
        ;
        $action:expr,
        $current:expr,
        $total:expr,
        $message:expr,
        $path:expr
    ) => {
        $crate::__private_helper::build_status_event_no_id(
            $action,
            $current,
            $total,
            $message,
            $path,
        )
    };

    // --------------------------
    // option normalization
    // --------------------------
    (@opt_str $value:expr) => { Some($value) };
    (@opt_str) => { None::<&'static str> };
    (@opt_usize $value:expr) => { Some($value) };
    (@opt_usize) => { None };
    (@opt_path $value:expr) => { Some($value) };
    (@opt_path) => { None };

}

/// Internal implementation of [`status_emit!`].
///
/// Doc:
/// - Parses all supported `status_emit!` invocation forms.
/// - Handles synchronous and asynchronous emission.
/// - Handles global and emitter-specific emission.
/// - Delegates status construction to [`status!`].
///
/// Note:
/// - Not part of the public API.
/// - May change at any time without notice.
/// - Public callers should use [`status_emit!`] instead.
/// - Exists so the public macro can remain a simple forwarding wrapper.
#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __status_emit {
    // ==================================
    // ASYNC INDEPENDENT
    // ==================================

    (async, $emitter:expr, $($status:tt)+) => {{
        $crate::__private_helper::ind_status_emit_async(
            $emitter,
            $crate::status!($($status)+)
        ).await;
    }};

    // ==================================
    // ASYNC GLOBAL
    // ==================================

    (async, $($status:tt)+) => {{
        $crate::__private_helper::global_emit_async(
            $crate::status!($($status)+)
        ).await;
    }};

    // ==================================
    // SYNC INDEPENDENT
    // ==================================

    ($emitter:expr, $($status:tt)+) => {{
        $crate::__private_helper::ind_status_emit_sync(
            $emitter,
            $crate::status!($($status)+)
        );
    }};

    // ==================================
    // SYNC GLOBAL
    // ==================================

    ($($status:tt)+) => {{
        $crate::__private_helper::global_emit_sync(
            $crate::status!($($status)+)
        );
    }};
}

// ========================
// Public Macro
// ========================
/// Constructs a [`StatusEvent`] from the provided fields.
///
/// Doc:
/// `status!` provides a concise way to create a `StatusEvent` without
/// manually using the builder APIs.
///
/// Supported fields:
/// - `action`
/// - `current`
/// - `total`
/// - `message`
/// - `path`
///
/// All fields are optional.
///
/// # Examples
///
/// Construct a status from named fields:
///
/// ```rust
/// use simple_status::status;
///
/// let status = status!(
///     action: "Build",
///     current: 2,
///     total: 10,
///     message: "Compiling",
/// );
/// ```
///
/// Construct a status containing only a message:
///
/// ```rust
/// use simple_status::status;
///
/// let status = status!("Finished");
/// ```
///
/// Construct a status using formatting:
///
/// ```rust
/// use simple_status::status;
///
/// let file = "main.rs";
///
/// let status = status!("Compiling {}", file);
/// ```
///
/// Note:
/// - Named fields may be provided in any combination.
/// - Fields that are not specified remain unset.
/// - Formatting arguments are stored as the status message.
#[macro_export]
#[clippy::format_args]
macro_rules! status {
    ($($tt:tt)*) => {{
        $crate::__status!($($tt)*)
    }};
}

/// Constructs and emits a [`StatusEvent`].
///
/// Doc:
/// `status_emit!` combines [`status!`] with the appropriate emission
/// function, reducing the boilerplate required to report status updates.
///
/// Supported modes:
/// - Global synchronous emission
/// - Global asynchronous emission
/// - Emitter-specific synchronous emission
/// - Emitter-specific asynchronous emission
///
/// # Examples
///
/// Global synchronous emission:
///
/// ```rust
/// use simple_status::status_emit;
///
/// status_emit!(
///     action: "Build",
///     current: 2,
///     total: 10,
/// );
/// ```
///
/// Global synchronous message:
///
/// ```rust
/// use simple_status::status_emit;
///
/// status_emit!("Build completed");
/// ```
///
/// Global asynchronous emission:
///
/// ```rust
/// # async {
/// use simple_status::status_emit;
///
/// status_emit!(
///     async,
///     action: "Download",
///     current: 5,
///     total: 10,
/// );
/// # };
/// ```
///
/// Emitter-specific synchronous emission:
///
/// ```rust,no_run
/// # let emitter = todo!();
/// use simple_status::status_emit;
///
/// status_emit!(
///     emitter,
///     action: "Build",
///     current: 3,
///     total: 10,
/// );
/// ```
///
/// Emitter-specific asynchronous emission:
///
/// ```rust,no_run
/// # async {
/// # let emitter = todo!();
/// use simple_status::status_emit;
///
/// status_emit!(
///     async,
///     emitter,
///     action: "Build",
/// );
/// # };
/// ```
///
/// Note:
/// - This macro is purely ergonomic.
/// - It does not add behavior beyond constructing a status and forwarding
///   it to the corresponding emit function.
/// - All syntax accepted by [`status!`] may be used for the status portion
///   of the invocation.
#[macro_export]
#[clippy::format_args]
macro_rules! status_emit {
    ($($tt:tt)*) => {{
        $crate::__status_emit!($($tt)*)
    }};
}
