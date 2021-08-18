mod redirect_home;
mod redirect_legacy_domain;
mod redirect_path;
mod redirect_path_prefix;
mod redirect_to_https;
mod remove_trailing_slash;
mod serve_static;

pub use self::{
    redirect_home::redirect_home,
    redirect_legacy_domain::redirect_legacy_domain,
    redirect_path::redirect_path, redirect_path_prefix::redirect_path_prefix,
    redirect_to_https::redirect_to_https,
    remove_trailing_slash::remove_trailing_slash, serve_static::serve_static,
};