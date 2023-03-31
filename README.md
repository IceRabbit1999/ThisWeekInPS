# ThisWeekInPS

Automatically sends an email to everyone in the section every Friday at 9am to collect weekly reports. (PS is for platform section), Automate your work, especially the ones you don't like but have to, so you can spend more time playing. :)

Actually a toy for practicing Rust

# Getting Started
This program will read the configuration file from `config/app.toml`, so create your own and fill it in before you running it. The example `app.toml` looks like this:
```toml
[dev]
# Choose where to store log file
log.path = "XXX"
log.prefix = "XXX.log"

# Your email username
mail.username = "XXX"
# Your email password
mail.password = "XXX"
mail.host = "smtp.xxx.com"
mail.from = "XXX"
mail.to = ["AddressA", "AddressB"]
```