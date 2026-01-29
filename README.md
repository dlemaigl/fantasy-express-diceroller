# Fantasy Express Discord Bot 🎲

A Discord dice roller bot for **Fantasy Express RPG** built in Rust using the Serenity framework.

## Features

- **2d10 Open-Ended Rolls**: Dice explode on 19-20 (chain explosions supported)
- **Fumble Detection**: Automatic critical failure on unmodified 2
- **FEAT Table Resolution**: Full success level calculation (0-5 SL)
- **Slash Commands**: Modern Discord interaction support

## Commands

| Command | Description |
|---------|-------------|
| `/roll` | Basic 2d10 open-ended roll |
| `/roll modifier:<num>` | Roll with bonus/penalty |
| `/skill bonus:<num> difficulty:<num>` | Skill check with FEAT resolution |
| `/initiative dexterity:<num> modifier:<num>` | Roll initiative (2d10, NOT open-ended) |
| `/attack attack_bonus:<num> defense:<num> damage:<num>` | Attack roll with FEAT and damage |
| `/save type:<TSR/RSR/WSR> bonus:<num> attack_level:<num>` | Saving throw roll |
| `/cast tier:<num> bonus:<num>` | Spell casting with Magical Resonance check |
| `/fumble weapon_type:<choice> skill_ranks:<num>` | Weapon fumble table |
| `/spellfumble tier:<num> ranks:<num> spell_type:<choice>` | Spell fumble table |
| `/opposed bonus1:<num> bonus2:<num>` | Opposed roll between two participants |
| `/help` | List all available commands |
| `/difficulty` | Show difficulty modifier reference |

### Example Output

```
🎲 Skill Roll
━━━━━━━━━━━━━━
Dice: [10, 9] = 19 💥 → [8, 6] = 14 = 33
Bonus: +15 | Difficulty: -6
━━━━━━━━━━━━━━
✅ SUCCESS (3 SL) (Total: 42)
Task accomplished! Spend Success Levels on Boons.
```

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- A Discord Bot Token

### Setup

1. **Clone the repository**
   ```bash
   git clone <repo-url>
   cd fantasy-express-bot
   ```

2. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env and add your DISCORD_TOKEN
   ```

3. **Build and run**
   ```bash
   cargo run
   ```

### Discord Bot Setup

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Go to **Bot** section and create a bot
4. Copy the token to your `.env` file
5. Go to **OAuth2 > URL Generator**
6. Select scopes: `bot`, `applications.commands`
7. Use the generated URL to invite the bot to your server

> **Tip**: Set `TEST_GUILD_ID` in `.env` for instant command registration during development.

## Development

```bash
# Run tests
cargo test

# Build release
cargo build --release
```

## Deployment (Raspberry Pi)

To deploy the bot on a Raspberry Pi (or other ARM64 Linux devices):

### 1. Cross-compile for ARM64
We use `cross` to compile for the Raspberry Pi architecture.

```bash
# Install cross if you haven't already
cargo install cross

# Build for ARM64
cross build --target aarch64-unknown-linux-gnu --release
```

### 2. Transfer files
Copy the executable and your environment file to the Raspberry Pi:

```bash
scp target/aarch64-unknown-linux-gnu/release/fantasy-express-bot .env pi@<PI_IP>:~/code/
```

### 3. Setup Systemd Service
To ensure the bot runs in the background and restarts automatically:

1. Copy the example service file:
   ```bash
   cp fantasy-express-bot.service.example fantasy-express-bot.service
   ```
2. Edit `fantasy-express-bot.service` if your paths or user are different.
3. Copy it to the Pi:
   ```bash
   scp fantasy-express-bot.service pi@<PI_IP>:~/code/
   ```
4. On the Pi, install and start the service:
   ```bash
   sudo mv ~/code/fantasy-express-bot.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable fantasy-express-bot
   sudo systemctl start fantasy-express-bot
   ```

## Rules Reference

See [RULES_REFERENCE.md](RULES_REFERENCE.md) for the complete Fantasy Express RPG dice mechanics.

## License

MIT
