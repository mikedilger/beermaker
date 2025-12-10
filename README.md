# Beermaker

This is a library to help you make beer.

Usually you would specify a process, specify a recipe, and then print that recipe.
The printout will contain etailed instructions for you with all calculations
done. See the [example](examples/marzen.rs) and try running it:

```rust
cargo run --example marzen
```

## Process

Here is the overall process, with it's various changes in volume and gravity:

| Volume Change       | Step          | Volume                      | Gravity                |
|---------------------|---------------|-----------------------------|------------------------|
| + strike water      |               |                             |                        |
|                     | Mash Step 1   | strike_volume()             |                        |
| + infusion(N)       |               |                             |                        |
|                     | Mash Step N+1 | mash_volume()               |                        |
| - absorb            |               |                             |                        |
|                     |               | pre_sparge_volume()         |                        |
| + sparge            | Sparge        |                             |                        |
|                     |               | pre_boil_volume()           | pre_boil_gravity()     |
| - evaporation       | Boil          |                             |                        |
|                     |               | post_boil_pre_loss_volume() |                        |
| - kettle loss       | Transfer      |                             |                        |
|                     |               | post_boil_volume()          |                        |
| + postboil dilution | Dilute        |                             |                        |
|                     |               | ferment_volume              | original_gravity()     |
|                     | Ferment       |                             |                        |
| - fermenter_losses  | Transfer      |                             |                        |
|                     |               | post_ferment_volume()       | post_ferment_gravity() |
| + postferm dilution | Dilute        |                             |                        |
|                     |               | product_volume()            | final_gravity()        |
|                     | Package       |                             |                        |
