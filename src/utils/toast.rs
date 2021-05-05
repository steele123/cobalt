use eyre::Result;
use winrt_notification::{Duration, Toast};

pub fn send(text: &str) -> Result<()> {
    Ok(Toast::new(Toast::POWERSHELL_APP_ID)
        .title("kataRUST")
        .text1(text)
        .duration(Duration::Short)
        .show()?)
}
