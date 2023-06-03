# Flame_rs

Flame_rs is a flame simulator CLI-tool built in the rust programming language. Flame score is calculated using stat equivalence ratios specified in **flame_values.json**. This tool helps to calculate the chance of getting a specific flame score (and its cost in meso for red flames). Additionally, it can also display the top scoring flames.

## Usage

Download the latest [release](https://github.com/viminaria/flame_rs/releases) and extract it wherever you like.

To use the tool, open your command line inside the folder you extracted and execute:

```powershell
.\flame_rs
```

This will run the tool with the default settings.

### Command Line Flags

* `-t --trials <TRIALS>`: Amount of flame simulations (default: 100000)
* `-s --stat <STAT>`: Stat to roll for (default: str)
  * options: str, dex, int, luk, kanna, da, xenon, alt_thief
* `-l --level <LEVEL>`: Equip level (default: 140-149)
  * options: 100-109, 110-119, 120-129, 130-139, 140-149, 150-159, 160-169, 170-179, 180-189, 190-199, 200-249, 250+
* `-k --keep <THRESHOLD>`: Minimum flame score target (default: 100)
* `-f --flametype <FLAMETYPE>`: Type of flame used (default: pflame)
  * options: totem, drop, pflame, eflame, regcraft, mastercraft, meistercraft, masterfuse, meisterfuse
* `--top <NUMBER>`: Displays the top scoring flames [OPTIONAL, max 1000]
* `-c --chance <NUMBER>`: Calculates the odds of getting target flame within the specified amount of flames [OPTIONAL]
* `-n --noboss`: Simulate non-boss flames [OPTIONAL]

### Examples

Here are some examples of how to use flame_rs:

```bash
.\flame_rs -t 1000000 -s luk -l 160-169 -k 120 -f masterfuse --top 10
```

This command simulates 1,000,000 masterfuse flames for an equip level of 160-169, rolling for luk with a minimum flamescore target of 120. It also displays the top 10 scoring flames.

```bash
.\flame_rs -t 500000 -s dex -l 200-249 -k 150 -f eflame --chance 50
```

This command simulates 500,000 eternal flames for an equip level of 200-249, rolling for dex with a minimum flamescore target of 150. It also calculates the odds of getting a flamescore of 150 or higher within 50 flames.

### Notes

Calculating very high flame scores requires a large amount of trials for accurate results, make sure the flame score you're aiming for isn't out of range of what's possible for a given flame type.


## Further Reading
* [StrategyWiki - Bonus Stats](https://strategywiki.org/wiki/MapleStory/Bonus_Stats)
