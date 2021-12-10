/// Provides the platform-specific path segment separator:
/// - `\` on Windows
/// - `/` on POSIX
#[allow(non_upper_case_globals)]
pub const sep: char = '\\';
/// Provides the platform-specific path delimiter:
/// - `;` for Windows
/// - `:` for POSIX
#[allow(non_upper_case_globals)]
pub const delimiter: char = ';';
