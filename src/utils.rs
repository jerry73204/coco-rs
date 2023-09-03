pub mod serde_url {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        url.as_str().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let url = Url::parse(&text)
            .map_err(|err| D::Error::custom(format!("failed to parse url: {:?}", err)))?;
        Ok(url)
    }
}

pub mod serde_date_captured {
    use chrono::NaiveDateTime;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2013-11-14 12:36:29
        format!("{}", datetime.format("%Y-%m-%d %H:%M:%S")).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let datetime =
            NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S").map_err(|err| {
                D::Error::custom(format!(
                    "failed to parse the value of date_captured: {:?}",
                    err
                ))
            })?;
        Ok(datetime)
    }
}

pub mod serde_iscrowd {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(&yes: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if yes { 1 } else { 0 }.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let yes = usize::deserialize(deserializer)? != 0;
        Ok(yes)
    }
}

pub mod serde_date_created {
    use chrono::NaiveDate;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 2017/09/01
        format!("{}", date.format("%Y/%m/%d")).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let date = NaiveDate::parse_from_str(&text, "%Y/%m/%d").map_err(|err| {
            D::Error::custom(format!(
                "failed to parse the value of date_created: {:?}",
                err
            ))
        })?;
        Ok(date)
    }
}
