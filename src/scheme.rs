//! Pure URL-scheme detection — which database driver a connection
//! string implies. No network access here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbKind {
    Postgres,
    MySql,
    Redis,
}

pub fn detect_kind(url: &str) -> Option<DbKind> {
    let scheme = url.split("://").next()?;
    match scheme {
        "postgres" | "postgresql" => Some(DbKind::Postgres),
        "mysql" => Some(DbKind::MySql),
        "redis" | "rediss" => Some(DbKind::Redis),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_postgres_from_either_scheme_spelling() {
        assert_eq!(
            detect_kind("postgres://user@host/db"),
            Some(DbKind::Postgres)
        );
        assert_eq!(
            detect_kind("postgresql://user@host/db"),
            Some(DbKind::Postgres)
        );
    }

    #[test]
    fn detects_mysql() {
        assert_eq!(detect_kind("mysql://user@host/db"), Some(DbKind::MySql));
    }

    #[test]
    fn detects_redis_including_tls_scheme() {
        assert_eq!(detect_kind("redis://host:6379"), Some(DbKind::Redis));
        assert_eq!(detect_kind("rediss://host:6379"), Some(DbKind::Redis));
    }

    #[test]
    fn unknown_scheme_returns_none() {
        assert_eq!(detect_kind("mongodb://host/db"), None);
    }

    #[test]
    fn malformed_url_with_no_scheme_separator_returns_none() {
        assert_eq!(detect_kind("not-a-url-at-all"), None);
    }
}
