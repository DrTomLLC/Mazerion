```markdown
# Calculator Catalog

This branch currently wires **46 calculators** into the registry (see `crates/calculators/src/lib.rs`).

## Basic (7)

| ID | Name | Description |
|---|---|---|
| `abv` | ABV Calculator | Calculate alcohol by volume from original and final specific gravity |
| `brix_to_sg` | Brix to SG | Convert degrees Brix to specific gravity (Brew Your Own formula) |
| `gravity_from_ingredients` | Gravity from Ingredients | Calculate expected gravity from honey/sugar and water volumes |
| `hydrometer_correction` | Hydrometer Temperature Correction | Correct hydrometer readings for temperature (general polynomial formula) |
| `plato_to_sg` | Plato to SG | Convert degrees Plato to specific gravity |
| `sg_correction` | SG Temperature Correction | Correct specific gravity reading for temperature (calibrated at 20°C) |
| `sg_to_brix` | SG to Brix | Convert specific gravity to degrees Brix (cubic polynomial) |

## Advanced (7)

| ID | Name | Description |
|---|---|---|
| `alcohol_tolerance` | Alcohol Tolerance | Calculate maximum ABV and estimated FG for yeast strain |
| `attenuation` | Attenuation Calculator | Calculate apparent and real attenuation (ASBC formulas) |
| `bench_trials` | Bench Trials | Calculate bench trial additions and scaling to full batch |
| `blending` | Blending Calculator | Calculate final properties when blending two batches |
| `dilution` | Dilution Calculator | Calculate water needed to dilute to target ABV |
| `refractometer` | Refractometer Correction | Correct refractometer readings for alcohol presence (Terrill cubic) |
| `volume_adjustment` | Volume Adjustment | Calculate volume adjustments for target gravity (dilution or concentration) |

## Brewing (4)

| ID | Name | Description |
|---|---|---|
| `carbonation` | Carbonation Calculator | Calculate priming sugar or keg PSI for target carbonation |
| `nutrition` | TOSNA Nutrition | Calculate TOSNA yeast nutrition schedule (1.0, 2.0, or 3.0 protocol) |
| `yeast_pitch` | Yeast Pitch Rate | Calculate yeast pitch rate for optimal fermentation |
| `yeast_starter` | Yeast Starter | Calculate yeast starter size and DME requirements |

## Beer (4)

| ID | Name | Description |
|---|---|---|
| `efficiency` | Brewhouse Efficiency | Calculate brewhouse efficiency from grain and gravity |
| `ibu` | IBU Calculator (Tinseth) | Calculate International Bitterness Units using Tinseth formula |
| `mash` | Mash Water Calculator | Calculate strike water temperature and volume for mash |
| `srm` | SRM Color Calculator | Calculate beer color using Morey equation |

## Finishing (7)

| ID | Name | Description |
|---|---|---|
| `acid_addition` | Acid Addition | Calculate acid additions to adjust pH - accounts for different acid strengths |
| `backsweetening` | Backsweetening | Calculate sweetener additions to reach target gravity |
| `bottling` | Bottling Calculator | Calculate bottles needed and headspace for batch volume |
| `pasteurization` | Pasteurization | Calculate time/temperature for safe pasteurization |
| `stabilization` | Stabilization | Calculate K-meta + sorbate for chemical stabilization |
| `sulfite` | Sulfite Calculator | Calculate K-meta additions with pH-dependent effectiveness |
| `tannin` | Tannin Calculator | Calculate tannin additions for body and mouthfeel |

## Mead Styles (10)

| ID | Name | Description |
|---|---|---|
| `acerglyn` | Acerglyn Calculator | Calculate ingredients for maple mead |
| `bochet` | Bochet Calculator | Calculate ingredients for caramelized honey mead with sugar loss |
| `braggot` | Braggot Calculator | Calculate ingredients for honey-malt hybrid mead |
| `capsicumel` | Capsicumel Calculator | Calculate ingredients for pepper mead |
| `cyser` | Cyser Calculator | Calculate ingredients for apple mead with juice sugar contribution |
| `great_mead` | Great Mead Calculator | Calculate ingredients for traditional mead (honey, water, yeast) |
| `hydromel` | Hydromel Calculator | Calculate ingredients for session mead (low ABV 3.5-7.5%) |
| `melomel` | Melomel Calculator | Calculate ingredients for fruit mead with sugar contribution |
| `metheglin` | Metheglin Calculator | Calculate ingredients for spiced mead (metheglin) with spice dosage |
| `sack` | Sack Mead Calculator | Calculate ingredients for high-gravity dessert mead (14-18% ABV) |

## Utilities (7)

| ID | Name | Description |
|---|---|---|
| `batch_cost` | Batch Cost Calculator | Calculate total cost per batch and per bottle |
| `gallons_to_bottles` | Gallons to Bottles | Calculate bottle count from volume |
| `gallons_to_bottles_with_losses` | Gallons to Bottles (with Losses) | Calculate bottle count accounting for brewing losses |
| `priming_alternatives` | Priming Sugar Alternatives | Calculate equivalent amounts for different priming sugars |
| `upscaling` | Recipe Upscaling | Scale recipes up or down - maintains perfect proportions |
| `waste` | Waste/Loss Calculator | Calculate expected losses through brewing process from start to bottle |
| `water_chemistry` | Water Chemistry | Calculate water profile and mineral additions |

---
## In progress
`fermentation_timeline` exists in the repo but is not currently exported/registered in `crates/calculators/src/lib.rs`.
