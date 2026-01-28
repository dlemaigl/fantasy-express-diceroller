# Fantasy Express - Rules Reference for Discord Bot

This document contains all dice rolling rules, combat mechanics, magic casting rules, and ability checks extracted from Fantasy Express RPG. Designed for implementing a Discord bot for playing Fantasy Express.

---

## Basic Resolution System

### Dice Mechanics

Fantasy Express uses **2d10 (two ten-sided dice)** as the core dice mechanic.

#### Open-Ended Rolls (Exploding Dice)
- **Standard Roll**: Roll 2d10, add modifiers
- **Explosion**: On an **unmodified 19 or 20**, roll again and add to total
- **Chain Explosion**: If the second roll is also 19-20, roll again and keep adding
- **All rolls are open-ended** unless specifically stated otherwise

#### Critical Failure (Fumble)
- **Trigger**: Unmodified roll of **2** (both dice show 1)
- **Effect**: Automatic failure with additional negative consequences
- **May require**: Roll on appropriate Fumble table (Weapon or Spell)

#### Alternative Dice Notations
| Dice | Method |
|------|--------|
| 1d5 | Roll 1d10, divide by 2 (round up) |
| 1d4 | Roll 1d5, subtract 1 (treat 0 as 1) |
| 1d3 | Roll 1d10, divide by 3 (round up), treat 0 or 10 as 3 |
| 1d2 | Roll 1d10, divide by 5 (round up) |

---

## The FEAT Table (Fantasy Express Action/Task)

All actions use this single resolution table:

| Roll Result | Outcome | Success Levels |
|-------------|---------|----------------|
| **UM 2** | Critical Failure | Fumble Roll |
| **3-15** | Failure | 0 |
| **16-19** | Failure/Partial Success | 0 (or half effect) |
| **20-22** | Success | 1 SL |
| **23-26** | Success | 2 SL |
| **27-30** | Success | 3 SL |
| **31-34** | Success | 4 SL |
| **35+** | Critical Success | 5 SL |

### Result Descriptions
- **Critical Failure (2)**: Fumble - danger, broken equipment, double time, or +4 to enemies' next roll
- **Failure (3-15)**: Task not accomplished
- **Partial Success (16-19)**: Bare minimum success with cost/complication, OR failure
- **Success (20-34)**: Task accomplished + Success Levels for Boons
- **Critical Success (35+)**: Best possible outcome + 5 Success Levels

---

## Skill Rolls

### Formula
```
Skill Roll = 2d10 (open-ended) + Skill Bonus + Modifiers - Difficulty
```

### Skill Bonus Composition
- Base Stat (determined by skill)
- Skill Ranks
- Kin Bonuses
- Special Bonuses
- Item Modifiers

### Difficulty Levels
| Difficulty | Modifier |
|------------|----------|
| Easy | +4 |
| Standard/Normal | +0 |
| Challenging | -3 |
| Hard | -6 |
| Very Hard | -9 |
| Heroic | -12 |
| Legendary | -15 |
| Mythic | -20 |

### Taking the Time
- **Conditions**: No hurry or stress
- **Bonus**: +4 to Skill Roll
- **Cost**: At least double the normal time

### Helping
- Helper makes a Skill Roll
- **Success**: +1 bonus per Success Level to lead character
- **Critical Success**: +5 bonus
- **Partial Success**: +1 bonus but helper faces complication
- **Failure/Critical Failure**: No bonus, potential trouble

---

## Success Level Boons (Skills)

| SL Cost | Boon Effect |
|---------|-------------|
| 1 | Learn extra relevant information |
| 1* | +2 bonus on related next task |
| 1+ | Increase result by 10% (max 50%) |
| 1 | Targets get -2 to saves against this |
| 1* | Task performed quietly (+2 to stealth) |
| 1+ | Reduce time by 10% per SL (max 50%) |
| 2 | Gain useful adventure information |
| 2* | +4 on next die roll (any purpose) |
| 2 | Stunning Success: onlookers stunned 1 round |
| 3* | +2 to future uses of this skill until Critical Failure |
| 3 | Gain important adventure information |
| 3* | +2 to all rolls for 24 hours |
| 4 | Character and allies get +3 on next roll |
| 5 | Inspiring: Character and allies within 50' get +4 on next roll |

*\* = Only one of these bonuses may apply at a time*
*+ = May spend multiple SL for cumulative effect*

---

## Conflicting Actions (Opposed Rolls)

Used when actions are mutually exclusive (contests, chases, hiding vs seeking).

### Resolution
1. All participants roll appropriate Skill
2. **Critical Failure**: Fails and suffers Fumble
3. **Compare results**: Higher wins
4. **Tie**: Neither wins, find another solution or re-roll

---

## Save Rolls

### Three Types

| Save Type | Stat | Purpose |
|-----------|------|---------|
| **Toughness (TSR)** | Stamina | Physical threats, poisons, diseases, fatigue |
| **Reflex (RSR)** | Dexterity | Avoid sudden threats, dodge traps |
| **Will (WSR)** | Resolve | Mental/spiritual effects, influence spells |

### Formula
```
Save Roll = 7 + Stat + Kin Modifier + Character Level + Special Modifiers
Save Roll Total = 2d10 (open-ended) + Save Roll Bonus - Attack Level/Tier
```

### Resolution
- **Success (20+)**: Resist the effect
- **Points of Failure**: Subtract total from 20 to determine margin of failure

---

## Combat System

### The Tactical Round
- **Duration**: 5 seconds (12 rounds per minute)
- **Actions are simultaneous** within the round

### Initiative

```
Initiative = 2d10 (NOT open-ended) + Dexterity + Modifiers
```

#### Initiative Modifiers
| Modifier | Condition |
|----------|-----------|
| -10 | Weapon not ready (first round) |
| +varies | Shorter weapon (in close quarters, +2 per step) |
| +varies | Longer weapon (at standard melee, +2 per step) |
| +5 | Longer weapon when closing to combat |
| -5 | Two weapon fighting |
| -5 | Weapon with Heavy quality |
| -varies | Shield (-1 per size; Target=-1, Tower=-5) |
| -20 | Surprised (first round) |
| 0 | Lightly encumbered |
| -5 | Medium encumbered |
| -10 | Heavily encumbered |
| -10 | Wounded more than 50% |
| -varies | Cast spell (-1 per 2 Tiers, rounded up) |

### Combat Round Sequence

**Phase 1: Initiative & Upkeep**
- Roll/check Initiative
- Apply bleeding damage
- Remove 1 round of Stun
- Remove ended spell effects

**Phase 2: Declare Actions**
- GM declares opponent actions first
- Players declare their actions
- Determine Parrying allocation

**Phase 3: Resolve Actions**
- Resolve in Initiative order (highest first)
- Handle Canceling, Opportunity, and Multi-Round actions

### Action Types

**Full Actions**
- Single melee attack (no modifiers)
- Aim ranged weapon (+4 next round)
- Cast Scaled Up spell
- Overcast a spell
- Set vs. Charge
- Walk/Jog base move distance
- Sprint double base move (-4 to DM and actions)

**Half Actions**
- Ranged attack
- Melee attack at -4 (not Heavy weapons)
- Melee with Swift quality weapon
- Load/Reload
- Aim ranged weapon (+2)
- Walk/Jog half base move
- Sprint base move (-4 to all)
- Cast non-instantaneous, non-scaled spell
- Concentrate on spell
- Ready/draw weapon
- Retrieve item from pack

**Free Actions**
- Load/Reload with Quick Load
- Cast Instantaneous spell
- Cast Cantrip
- Aim (+1)
- Quick Step (up to 5')
- Drop held item
- Change/Maintain engagement distance
- Assessment Roll
- Verbalization
- Activate special abilities/magic items
- Consume item from belt/readied location

### Action Combinations per Round
- 1 Full Action + Free Action, OR
- 2 Half Actions + Free Action, OR
- 1 Full + 1 Half + Free (at -4 to all), OR
- 3 Half Actions + Free (at -4 to all)

**Limit**: Only 1 Attack Action OR 1 Spell Casting Action per round

---

## Attack Rolls

### Formula
```
Attack Roll = 2d10 (open-ended) + Attack Bonus - Target's Defensive Modifier
```

### Attack Bonus Components
- Combat Skill bonus (Blades, Blunt, Brawl, Polearms, or Ranged)
- Weapon modifiers
- Situational modifiers

### Attack Results

| Result | Effect |
|--------|--------|
| **Critical Failure** | Fumble - roll on Weapon Fumbles table |
| **Failure** | Miss or glancing blow (no damage) |
| **Partial Success** | Deal Damage Rating (DR), reduced by Armor Rating (AR) |
| **Success (1-4 SL)** | DR + 1 Hit per point over 20 (max 3x DR), spend SL on Boons |
| **Critical Success (5 SL)** | As Success + automatic Death Strike |

### Damage Calculation
1. Base Hit Damage = DR + 1 per point over 20 (capped at 3x DR)
2. Spend Success Levels on Combat Boons
3. Subtract target's Armor Rating (AR) for damage type
4. Apply remaining damage and critical effects

---

## Defensive Modifier (DM)

### Formula
```
DM = Dexterity
   - Armor Dexterity penalty
   + Armor Quality/Magic bonuses
   + Shield bonus
   + Magic item bonuses
   + Condition bonuses (Dim Light, Cover, etc.)
   +/- Parrying
```

### Cover Bonuses
| Cover Type | DM Bonus |
|------------|----------|
| Half Soft Cover (up to 50% hidden) | +2 |
| Half Hard Cover (up to 50% hidden) | +4 |
| Full Soft Cover (50-90% hidden) | +4 |
| Full Hard Cover (50-90% hidden) | +10 |

---

## Parrying

### Mechanics
- Shift points from Attack Bonus (AB) to Defensive Modifier (DM)

### Ratios
| Situation | Ratio (AB to DM) |
|-----------|------------------|
| Single Foe | 1:1 |
| Multiple Foes | 2:1 |
| Improvised/Two-handed non-Swift | 2:1 / 3:1 |
| Stunned | 3:1 |
| With Shield (vs multiple) | 1:1 |

### Restrictions
- Cannot Parry On Rear or Surprise attacks
- Incapacitated/Held/Surprised cannot Parry
- Even with all AB shifted, still get +0 Attack Roll

---

## Combat Boons

### Critical Damage by Type

| SL | Bash | Slash | Pierce | Magic/Energy | Martial Arts | Tooth & Claw |
|----|------|-------|--------|--------------|--------------|--------------|
| 1* | +6 Hits, Dazed(1) | +3 Hits, Pain(1) | +2 Hits, Bleed(1) | +4 Hits, Dazed(1) | +3 Hits, Dazed(1) | +5 Hits, Dazed(1) |
| 2* | +9 Hits, Dazed(2), Stun(1) | +6 Hits, Dazed(1), Pain(2) | +4 Hits, Dazed(1), Pain(1) | +7 Hits, Dazed(2), Pain(1) | +5 Hits, Dazed(2), Pain(1) | +8 Hits, Pain(1), Bleed(1) |
| 3* | +11 Hits, Pain(1), Bleed(1) | +9 Hits, Bleed(1), Pain(2) | +6 Hits, Stun(1), Bleed(2) | +10 Hits, Pain(2), Bleed(1) | +8 Hits, Stun(1), Bleed(1) | +10 Hits, Stun(1), Bleed(2) |
| 4* | +16 Hits, Stun(2), Pain(2) | +12 Hits, Pain(3), Bleed(3) | +8 Hits, Pain(2), Bleed(4) | +14 Hits, Stun(2), Pain(3) | +11 Hits, Stun(3), Pain(2) | +15 Hits, Pain(2), Bleed(3) |
| 5* | +20 Hits, Stun(3), Bleed(3) | +15 Hits, Stun(1), Bleed(4) | +10 Hits, Pain(3), Bleed(5) | +18 Hits, Stun(3), Pain(4) | +15 Hits, Stun(4), Pain(3) | +20 Hits, Stun(2), Bleed(4) |

*\* Base SL cost, may be modified by target's armor SL rating*

### Critical Effect Definitions
- **Bleed(#)**: Lose # Hit Points each Upkeep phase until healed/bandaged
- **Dazed(#)**: -4 to all actions for # rounds
- **Pain(#)**: -# penalty to actions (Move Penalty type)
- **Stun(#)**: Cannot take Full Actions, Attack, or Cast for # rounds
- **Held**: Cannot move, -6 to AB and DM, escape requires roll
- **Incapacitated**: Cannot act or Parry

### Combat Options (Boons)

| SL | Option |
|----|--------|
| 1+ | Combat Shuffle: Move foe/self 5' per SL |
| 1 | Defensive Posture: +2 DM next round |
| 1 | Disarm Foe: Foe makes TSR/RSR or weapon flies 1-10' |
| 1 | Hold Foe: Foe makes RSR/TSR(Brawn) or be Held |
| 1+ | Initiative Boost: +2 initiative next round per SL |
| 1 | Knock Prone: Foe makes RSR or falls Prone |
| 1 | Rapid Reload: Free reload action |
| 2+ | Attack Boost: +1 AB next round per SL |
| 2 | Improved Hold: Foe is Held, no save this round |
| 2+ | Inspiring Attack: Allies get +1 initiative per SL |
| 2 | Lightning Strike: Second attack at -4 |
| 2 | Pierce Armor: Halve enemy AR |
| 3 | Advantageous Attack: Attack from On Rear next round (+6 AB) |
| 3 | Disarm Foe II: Automatic disarm, no save |
| 3 | Double Shot: Hit additional target within 30' |
| 3 | Incapacitated: Foe saves vs Combat Skill ranks or KO |
| 3 | Knocked Prone: No save |
| 3 | Multiple Strikes: Hit adjacent foe at -2 |
| 3 | Pierce Armor II: Quarter enemy AR |
| 3 | Ricochet: Bash hits second target within 10' at -4 |
| 4 | Deadly Strike: TSR vs Level - Success=Incapacitated, Fail=Die in 10-X rounds |
| 4 | Knocked Out: Incapacitated, no save |
| 4 | Pierce Armor True: Ignore AR completely |
| 5 | Death Strike: TSR vs Level - Fail=Die, Success=Incapacitated 24h |
| 6 | Instant Death: No save |

---

## Damage Types

| Code | Type |
|------|------|
| b | Bash |
| s | Slash |
| p | Pierce |
| e | Elemental |
| g | Grapple/Grab |

---

## Weapon Fumbles

### Fumble Trigger
- Roll within weapon's Critical Failure range (usually 2, some weapons 3-4+)

### Fumble Roll Modifiers
| Weapon Type (Melee/Thrown) | Mod | Weapon Type (Missile) | Mod |
|----------------------------|-----|----------------------|-----|
| Brawl, Hand, Short impact | +0 | Hand or Light Crossbow | +0 |
| Short edged, Long impact | +2 | Short bow, sling shot | +2 |
| Long edged | +4 | Heavy crossbow | +4 |
| Two-handed, chain weapons | +6 | Composite bow | +6 |
| Polearms, net, whip | +10 | Long bow | +10 |

**Reductions**: -1 per 2 ranks in weapon skill, -1 per Weapon Focus

### Fumble Results
| Roll | Effect |
|------|--------|
| ≤15 | Make Assessment Roll next Upkeep |
| 16-20 | Assessment + choose 1: Drop weapon, OR Take 1 SL critical to self |
| 21-25 | Assessment + choose 2: Drop, 2 SL critical to self, Hit ally |
| 25-30 | Assessment + choose 2: Drop, 2 SL critical, Hit ally, Weapon breaks |
| 30+ | Assessment + choose 2: Drop, 3 SL critical, Hit ally, Weapon breaks |

---

## Combat Conditions

| Condition | Effect |
|-----------|--------|
| **Bruised** (≥50% HP lost) | -4 to all Actions (not DM/Saves) |
| **Dazed** | -4 to all actions for duration |
| **Dying** | Die in X rounds unless healed |
| **Engaged** | Cannot move without Disengage/Run Away |
| **Flanking** | Attacker gets +3 AB; Shield only protects Shield-side |
| **Frightened** | Cannot attack source, may flee |
| **Held** | Cannot move, -6 AB and DM, attacks get +6, only Hand/Short weapons |
| **Incapacitated** | Cannot act/Parry, attacks auto-resolve as 35 |
| **On Rear** | Attacker gets +6 AB, no Shield bonus to DM |
| **Prone** | Half DM, -4 AB, no 2-handed weapons, +4 vs ranged (side roll) |
| **Stunned** | No Full Actions/Attacks/Spells, Parry at 3:1 |
| **Surprised** | Cannot Attack/Cast, Half/Free Actions only, no Parry/Shield, +4 to attackers |
| **Weary** | Half base move, 1 Action/round, no healing |

---

## Ranged Attack Ranges

| Range | Modifier |
|-------|----------|
| Point Blank (0-0.5x Range) | +4 |
| Short (0.5x-1x Range) | +0 |
| Medium (1x-2x Range) | -4 |
| Long (2x-3x Range) | -10 |
| Extreme (3x-4x Range) | -16 |
| Beyond 4x Range | Impossible |

---

## Martial Moves

### Disarm
- **Action**: Full Action (can still Parry)
- **Resolution**: Conflicting Action - your AB vs foe's AB
- **Success**: Foe disarmed, weapon flies 1-10'
- **Critical Success**: Grab weapon or it flies 6-15'
- **Defender Critical Success**: You fumble your weapon

### Feint
- **Action**: Half Action + Half Action attack
- **Resolution**: Conflicting Action - Deceive vs Perception
- **Win**: Gain difference as bonus to AB for rest of round
- **Lose**: Foe gains difference as bonus to DM

### Grapple
- **Action**: Full Action
- **Resolution**: Conflicting Action - Brawl vs Brawl (or RSR)
- **Win**: Target is Held
- **Critical Success**: Target is Held and Incapacitated
- **Escape**: Another Grapple at -6, or TSR (using Brawn) vs attacker's Brawl ranks

### Knock Down
- **Action**: Full Action
- **Resolution**: Conflicting Action - Brawl vs Brawl (or RSR)
- **Win**: Target is Prone
- **Critical Success**: Target is Prone + Stunned (duration = roll difference)

### Lock
- **Action**: Full Action (can still Parry)
- **Resolution**: Conflicting Action - AB (minus Parry) vs foe's AB
- **Win or Tie**: Both combatants are Locked (cannot move, attack, or Parry)
- **Escape**: Conflicting Action each round, or drop weapon

### Maintain Distance (Hold at Bay)
- **Action**: Full Action
- **Resolution**: Conflicting Action - AB vs Athletics/Acrobatics
- **Win**: Attack at AB with additional -4
- **Critical Success**: Attack at full AB
- **Lose**: Foe closes and attacks, or Disengages

### Riposte
- **Action**: Full Action
- **Resolution**: Parry first, then counterattack
- **Foe does no damage**: +4 to your attack
- **Foe does HP damage only**: +2 to your attack

### Shield Bash
- **Action**: Half Action
- **Skill**: Brawl at -4 or Blunt skill
- **Damage Rating**: 2x Shield's DM bonus
- **Cost**: Lose shield DM bonus for the round

---

## Magic System

### Core Concepts
- **Mana**: Energy used to cast spells
- **Tier**: Spell's power level (= Mana cost)
- **School of Magic**: Collection of 30 spells (Codex)
- **Folio**: Subset of 12 spells for Hybrid classes
- **Universal Spells**: 10 spells any caster can learn

### Casting Requirements
1. Spell must be from caster's Codex/Folio
2. Skill ranks in spell ≥ 2 × Tier
3. Sufficient Mana available
4. Able to speak verbal components
5. At least one hand free or holding Casting Focus

### Casting Roll Formula
```
Casting Roll = 2d10 (open-ended) + Spell Skill Bonus + Casting Modifiers
```

### Casting Results
| Result | Effect |
|--------|--------|
| Critical Failure | Spell Fumble - roll on Spell Fumble table |
| Failure | Spell fails, Mana expended |
| Partial Success | Choose: Fail (no mana), half effect, or double mana cost |
| Success | Spell works, spend SL on Casting Boons |
| Critical Success | All effects doubled + 5 SL for Boons |

### Casting Modifiers (Bonuses)
| Source | Bonus |
|--------|-------|
| Casting Focus (wand, holy symbol) | +4 (only offsets penalties) |
| Flamboyant Gestures | +2 (only offsets penalties) |
| Flamboyant Incantations | +2 (only offsets penalties) |

### Casting Time
| Spell Type | Time |
|------------|------|
| Instantaneous | Free Action |
| Normal (non-scaled) | Half Action |
| Scaled Up | Full Action |
| Overcast | 2 Full Actions (2 rounds) |

### Mana Recovery
- **8 hours sleep**: Full Mana recovery
- **4 hours sleep**: Half Mana recovery
- **2 hours sleep**: Quarter Mana recovery (rounded down)
- **Less than 2 hours**: No recovery

---

## Casting Boons

| SL | Boon Effect |
|----|-------------|
| 1+ | Attack Advantage: +1 to spell attack roll |
| 1+ | Harder Save: -1 to target's Save Roll |
| 1+ | Increased Targets: +1 extra target |
| 2+ | Increased Range: +1 range increment |
| 2+ | Increased Duration: +1 duration increment |
| 2+ | Increased Radius: +1 radius increment |
| 2+ | Increased Distance: +1 effect distance increment |
| 2+ | Increase Numerical Effect: +1 to non-damage number |

*+ = Each additional SL adds another instance*
*Increment = Base spell value (e.g., 10' range → 10' increment)*

---

## Abnormal Spell Failure (ASF)

### ASF Modifiers
| Condition | Modifier |
|-----------|----------|
| Spell within School but outside Codex/Folio | +5 ASF |
| Spell from different School | +10 ASF |
| Overcasting non-instantaneous spell | +1 per missing rank |
| Overcasting instantaneous spell | +2 per missing rank |
| Undercasting | +3 per missing Mana Point |
| No Gestures | +4 ASF |
| Subtle Gestures | +2 ASF |
| No Incantations | +4 ASF |
| Quiet Incantations | +2 ASF |
| Soul Burn | +5 per 1 Soul Damage |
| Armor | Varies |

### ASF Effects
- Critical Failure range = 2 + total ASF modifier
- If Critical Failure occurs, Spell Fumble roll uses 3× ASF as modifier

---

## Spell Fumble

### Fumble Roll Modifiers
| Factor | Modifier |
|--------|----------|
| Spell Tier | +Tier |
| ASF Modifiers | +3 × ASF |
| Skill Ranks in Spell | -Ranks |
| Healing/Divination spells | +0 |
| Utility/Self/Defensive/Nature | +2 |
| Enchantment magic | +4 |
| Alteration magic | +6 |
| Dark/Elemental magic | +6 |

### Fumble Results
| Roll | Effect |
|------|--------|
| ≤18 | Lose Mana Points |
| 19-22 | Lose Mana, Stunned 1 round |
| 23-26 | Stunned 1 round + choose 1: Lose double Mana OR Spell delays 2 rounds (requires concentration) |
| 27-30 | Stunned 2 rounds + choose 1: Lose double Mana OR Spell hits different target |
| 31-34 | Stunned 3 rounds + choose 2: Lose triple Mana, wrong target, Magical Resonance (+4) |
| 35+ | Stunned 4 rounds + choose 2: Lose triple Mana + Weary, wrong target, Magical Resonance (+10), knocked out 6 hours |

---

## Attack Spells

### Bolt Spells (Single Target)
1. Cast spell successfully
2. Make Attack Roll (higher of Ranged or Arcana)
3. Resolve as ranged attack
4. No Medium/Long/Extreme range - dissipates beyond spell range

### Ball Spells (Area)
1. Cast spell successfully
2. Make Attack Roll (higher of Ranged or Arcana)
3. Choose: Target specific enemy OR target empty space
4. All in radius take damage
5. Center target: RSR to avoid +4 bonus damage
6. Edge targets (within 3'): RSR for half damage

### Cone Spells
- Fills cone-shaped area with caster at apex
- Width at far end = 2.5' per 5' of length
- Same damage resolution as Ball spells

### Line Spells
- 1' wide for entire length
- Hits all in path, continues past targets
- Successful RSR = no damage (complete dodge)

---

## Counterspells

### Methods
1. **Opposing Spell**: Cast spell that cancels effect (e.g., darkness vs light)
2. **Counterspell**: Cast the Counterspell spell
3. **Arcana Skill**: Use double the Mana of spell being countered

### Resolution
1. Assessment Roll (Perception or Arcana) to learn Tier
2. Apply modifiers:
   - Different School: -4
   - Mana difference: +1 per point you're ahead, -1 per point behind
3. Conflicting Action roll
4. Tie or counter wins: Spell is countered

---

## Magical Resonance

### Trigger
- Rolling **doubles** on Casting Roll where doubled number ≤ spell Tier
- Example: Casting Tier 3 spell, rolling double 1, 2, or 3 triggers resonance

### Resonance Roll Modifiers
| Factor | Modifier |
|--------|----------|
| Spell Tier | +Tier |
| With ASF modifiers | +ASF |
| In Safe Haven | -4 |
| In Blighted/Darkland | +4 |
| Healing/Beneficial spell | -4 |
| Nature/Illusionary spell | -2 |
| Attack/Harmful spell | +4 |
| Dark/Infernal spell | +6 |

### Resonance Results
| Roll | Effect |
|------|--------|
| ≤14 | Nothing - escaped notice |
| 15-18 | Shiver - brief attention, passed |
| 19-22 | Awareness - Darkmaster notices, next spell auto-triggers Resonance roll |
| 23-26 | Attention - Darkmaster sends agents OR caster Stunned 1-4 rounds |
| 27-30 | Pursuit - Capture team sent OR caster has visions, unable to act until Assessment Roll |
| 31-34 | Assault - Destruction team sent OR caster incapacitated 1d10 rounds, loses all Mana OR answers one question |
| 35+ | Lieutenant - Overwhelming servant of Darkmaster arrives |

---

## Essential Salts (Alternative Mana Source)

### Usage
- Replace all but 1 Mana with Essential Salts (1 dram per Mana)
- Maximum carry: 2 × Magic Stat ounces (8 drams/ounce)
- Exceeding limit: All explode for 1 damage per dram (10' radius)

### Finding Essential Salts
1. Spend 1 hour sifting soil
2. Roll Arcana or Alkemics
3. Find 2 drams per Success Level
4. Spend Drive: +5 drams per Drive Point
5. Maximum 3 searches per day
6. After Wyrstorm: Double yield

---

## Quick Reference: Rolling Formulas

```
SKILL ROLL:     2d10 + Skill Bonus - Difficulty ≥ 20 = Success
SAVE ROLL:      2d10 + (7 + Stat + Kin + Level) - Attack Level ≥ 20 = Success
ATTACK ROLL:    2d10 + Attack Bonus - Target DM ≥ 20 = Hit
CASTING ROLL:   2d10 + Spell Skill Bonus + Casting Mods ≥ 20 = Success
INITIATIVE:     2d10 (not open-ended) + Dexterity + Modifiers

CRITICAL FAILURE: Unmodified 2 (both dice = 1)
EXPLOSION: On 19 or 20, roll again and add
TARGET NUMBER: 20 (always)
```

---

*Extracted from Fantasy Express RPG by Tim Dugger and Firehawk Games, © 2023*
