# Fantasy Express Discord Bot 🎲

A Discord dice roller bot for **Fantasy Express RPG** built in Rust using the Serenity framework.

---

## 🚀 For Users: Using the Bot

### Add the Bot to Your Server

You can invite the bot to your Discord server using the following link:
👉 **[Invite Fantasy Express Bot](https://discord.com/oauth2/authorize?client_id=1466046361043468433&permissions=19456&integration_type=0&scope=bot+applications.commands)**

### Features

- **2d10 Open-Ended Rolls**: Dice explode on 19-20 (chain explosions supported)
- **Fumble Detection**: Automatic critical failure on unmodified 2
- **FEAT Table Resolution**: Full success level calculation (0-5 SL)
- **Slash Commands**: Modern Discord interaction support

### Commands Reference

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

---

## 🛠️ For Developers: Technical & Self-Hosting

If you want to host the bot yourself or contribute to the development, follow these instructions.

### Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- A Discord Bot Token

### Installation & Setup

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

### Discord Bot Configuration

If you are hosting your own instance:
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application and add a **Bot**
3. Copy the **Token** to your `.env` file
4. Generate an invite link in **OAuth2 > URL Generator**:
   - **Scopes**: `bot`, `applications.commands`
   - **Bot Permissions**: `Send Messages`, `View Channels`

> **Note**: This bot uses Slash Commands, so the "Message Content" intent is NOT required.

### Development Commands

```bash
# Run tests
cargo test

# Build release
cargo build --release
```

### Deployment (Raspberry Pi / ARM64)

We use `cross` to compile for the Raspberry Pi architecture.

1. **Cross-compile**:
   ```bash
   cargo install cross
   cross build --target aarch64-unknown-linux-gnu --release
   ```

2. **Transfer files**:
   ```bash
   scp target/aarch64-unknown-linux-gnu/release/fantasy-express-bot .env pi@<PI_IP>:~/code/
   ```

3. **Setup Systemd Service**:
   Copy and enable the provided service file to keep the bot running:
   ```bash
   cp fantasy-express-bot.service.example fantasy-express-bot.service
   # Edit paths in the service file as needed...
   sudo cp fantasy-express-bot.service /etc/systemd/system/
   sudo systemctl enable --now fantasy-express-bot
   ```

## Rules Reference

See [RULES_REFERENCE.md](RULES_REFERENCE.md) for the complete Fantasy Express RPG dice mechanics.

## License

MIT
