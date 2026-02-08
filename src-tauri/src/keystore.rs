use keyring::Entry;

const SERVICE_NAME: &str = "multivm-workspace-tool";

pub fn store_password(host: &str, user: &str, password: &str) -> Result<(), String> {
    let key = format!("{}@{}", user, host);
    let entry = Entry::new(SERVICE_NAME, &key).map_err(|e| format!("keyring entry: {e}"))?;
    entry
        .set_password(password)
        .map_err(|e| format!("keyring store: {e}"))?;
    Ok(())
}

pub fn retrieve_password(host: &str, user: &str) -> Result<Option<String>, String> {
    let key = format!("{}@{}", user, host);
    let entry = Entry::new(SERVICE_NAME, &key).map_err(|e| format!("keyring entry: {e}"))?;
    match entry.get_password() {
        Ok(pw) => Ok(Some(pw)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("keyring retrieve: {e}")),
    }
}

pub fn delete_password(host: &str, user: &str) -> Result<(), String> {
    let key = format!("{}@{}", user, host);
    let entry = Entry::new(SERVICE_NAME, &key).map_err(|e| format!("keyring entry: {e}"))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("keyring delete: {e}")),
    }
}
