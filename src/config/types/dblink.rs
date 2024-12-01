/*
    Appellation: dblink <module>
    Contrib: @FL03
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum DbUrl {
    DatabaseUrl(DatabaseUrl),
    String(String),
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct DatabaseUrl {
    #[serde(alias = "prefix", alias = "provider")]
    pub scheme: String,
    pub host: String,
    pub port: u16,
    #[serde(alias = "user")]
    pub username: String,
    pub password: String,
    pub database: String,
}

mod impl_link {
    use super::*;

    impl DbUrl {
        pub fn as_db_url(&self) -> DatabaseUrl {
            match self {
                Self::DatabaseUrl(url) => url.clone(),
                Self::String(url) => url.parse().expect("Failed to parse database url"),
            }
        }

        pub fn as_str(&self) -> String {
            match self {
                Self::DatabaseUrl(url) => url.to_string(),
                Self::String(url) => url.clone(),
            }
        }

        pub fn as_url(&self) -> url::Url {
            self.as_str().parse().expect("Failed to parse database url")
        }
    }

    impl Default for DbUrl {
        fn default() -> Self {
            Self::DatabaseUrl(DatabaseUrl::default())
        }
    }

    impl From<url::Url> for DbUrl {
        fn from(url: url::Url) -> Self {
            DbUrl::String(url.to_string())
        }
    }

    impl From<DbUrl> for config::Value {
        fn from(link: DbUrl) -> Self {
            link.as_db_url().into()
        }
    }
    impl From<DbUrl> for DatabaseUrl {
        fn from(link: DbUrl) -> Self {
            link.as_db_url()
        }
    }

    impl From<DatabaseUrl> for DbUrl {
        fn from(url: DatabaseUrl) -> Self {
            DbUrl::DatabaseUrl(url)
        }
    }
}

mod impl_url {
    use super::*;

    impl DatabaseUrl {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn from_url_str(url: &str) -> Self {
            url.parse().expect("Failed to parse database url")
        }

        pub fn from_env() -> Self {
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| crate::DEFAULT_DB_URL.to_string())
                .parse()
                .expect("Failed to parse database url")
        }

        gsw! {
            scheme: String,
            host: String,
            port: u16,
            username: String,
            password: String,
            database: String
        }
    }

    impl Default for DatabaseUrl {
        fn default() -> Self {
            Self {
                scheme: "postgresql".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "password".to_string(),
                database: "postgres".to_string(),
            }
        }
    }

    impl core::fmt::Display for DatabaseUrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(
                f,
                "{}://{}:{}@{}:{}/{}",
                self.scheme, self.username, self.password, self.host, self.port, self.database
            )
        }
    }

    impl core::str::FromStr for DatabaseUrl {
        type Err = url::ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // parse the string as a url
            let url = s.parse::<url::Url>()?;
            // extract the values from the url
            let provider = url.scheme().to_string();
            let host = url.host_str().unwrap().to_string();
            let port = url.port().unwrap();
            let user = url.username().to_string();
            let password = url.password().unwrap().to_string();
            let database = url.path().to_string();
            // create a new instance from the parsed values
            let instance = Self {
                scheme: provider,
                host,
                port,
                username: user,
                password,
                database,
            };
            // return the instance
            Ok(instance)
        }
    }

    impl<'a> core::cmp::PartialEq<&'a str> for DatabaseUrl {
        fn eq(&self, other: &&'a str) -> bool {
            self.to_string() == *other
        }
    }

    impl core::cmp::PartialEq<String> for DatabaseUrl {
        fn eq(&self, other: &String) -> bool {
            self.to_string() == *other
        }
    }

    impl From<String> for DatabaseUrl {
        fn from(url: String) -> Self {
            url.parse().expect("Failed to parse database url")
        }
    }

    impl From<DatabaseUrl> for String {
        fn from(url: DatabaseUrl) -> Self {
            url.to_string()
        }
    }

    impl From<DatabaseUrl> for config::Value {
        fn from(url: DatabaseUrl) -> Self {
            url.to_string().into()
        }
    }

    impl From<config::Value> for DatabaseUrl {
        fn from(value: config::Value) -> Self {
            value
                .to_string()
                .parse()
                .expect("Failed to parse database url")
        }
    }

    impl From<url::Url> for DatabaseUrl {
        fn from(url: url::Url) -> Self {
            url.to_string()
                .parse()
                .expect("Failed to parse database url")
        }
    }

    impl From<DatabaseUrl> for url::Url {
        fn from(url: DatabaseUrl) -> Self {
            url.to_string()
                .parse()
                .expect("Failed to parse database url")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url() {
        let str = "postgres://postgres:postgres@localhost:5432/postgres";
        let url = str.parse::<DatabaseUrl>().unwrap();
        assert_eq!(url.scheme, "postgres");
        assert_eq!(url.host, "localhost");
        assert_eq!(url.port, 5432);
        assert_eq!(url.username, "postgres");
        assert_eq!(url.password, "postgres");
        assert_eq!(url.database, "postgres");
        assert_eq!(url, str);
    }
}
