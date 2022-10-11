# Wicker

It's often helpful to have weighted probabilities.
This crate offers the [`WeightedPicker`] struct that serves as a sort of weighted bag;
you can give it entries with various weights, and then randomly sample them.

This is the way Minecraft loot tables work, if this sounds familiar.

The algorithm used is [Vose's Alias Method](https://www.keithschwarz.com/darts-dice-coins/)
(scroll to the bottom), which to be honest I absolutely do not understand.
But it has O(n) creation and O(1) selection, so sounds good to me.

A [`WeightedPicker`] is static; you can't edit the probabilities after you've created it due
to the algorithm used. However, you can edit each associated value after creation through the
[`WeightedPicker::pick`] method, if you wanted to do that for some reason.

## Sample Usage

```rust
# use wicker::WeightedPicker;
let picker = WeightedPicker::new(vec![
    ("common", 10.0),
    ("uncommon", 5.0),
    ("rare", 2.0),
    ("legendary", 1.0),
    ("mythic", 0.1),
]);

let mut rng = rand::thread_rng();
for _ in 0..10 {
    println!("- {}", picker.get(&mut rng));
}
```

A sample output:
- legendary
- rare
- uncommon
- common
- common
- rare
- uncommon
- common
- common
- uncommon